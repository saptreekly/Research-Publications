I used my static analysis TUI, [Vanguard-RE](/projects/vanguard-re), to reverse-engineer the leaked Conti (ContiLocker_v2) ransomware build tree without executing anything. This write-up follows the STAR method (Situation, Task, Action, Result) and walks through how I drove Vanguard, what it surfaced, and how I checked those findings against the leaked C++ source.

## Situation

I keep a local malware corpus under password-protected ZIPs (`infected`) for defensive research. For this exercise I needed a sample that would stress Vanguard end to end: in-memory ZIP quarantine, PE triage, ImpHash, entropy, IAT heuristics, crypto fingerprinting, network IOC scanning, and iced-x86 disassembly.

I chose `conti_locker.zip` because it is not a single opaque payload. It is the leaked ContiLocker_v2 Visual Studio project. The archive holds `.cpp` and `.h` source, build logs, `.obj` and `.pdb` artifacts, and two compiled `decryptor.exe` binaries. That gave me a rare setup where I could treat the PE files as the unknown and the source as ground truth.

| Field | Value |
| --- | --- |
| Family | Conti (Conti Locker v2) |
| Archive | `conti_locker.zip` |
| Archive SHA256 | `e403aa3f4273cd11fa13597586cbf353ecef4040344f6ee9fff23168156fb29a` |
| Archive MD5 | `33724d7ca2a6888d7880e0fa49c43e9b` |
| Members | 141 (source tree + build artifacts + 2 PE binaries) |
| Analyzed binaries | `decryptor.exe` ×2 (Debug builds) |
| PDB path | `c:\source\ContiLocker_v2\Debug\decryptor.pdb` |
| Classification | Ransomware (crypto-locker + SMB worming component) |
| Containment | Static-only, in-memory decryption via Vanguard |

## Task

My goals were:

1. Run the full Conti archive through Vanguard under the same containment rules I ship in the TUI (map and parse only, never execute, keep ZIP members in RAM).
2. Let Vanguard rank members, deep-dive the top hits, and report crypto, imports, strings, YARA, and disassembly insights.
3. Cross-check every high-confidence Vanguard claim against the leaked source so I could document where the tool was exact, where it was suggestive, and where the wording overreached.
4. Produce defensive artifacts (IOCs, YARA, Sigma) from those validated findings.

I was not trying to recover a victim environment or recover encrypted files. I was stress-testing my own triage pipeline on a well-documented ransomware family.

## Action

### How I used Vanguard

Vanguard is my Rust TUI for high-speed, memory-safe static malware triage. Its design matches what I needed here:

| Pillar | What I relied on |
| --- | --- |
| Speed | `memmap2` zero-copy I/O and a focused static pipeline |
| Accuracy | Formal PE parsing (`goblin`), ImpHash, Shannon entropy, IAT capability tags, crypto-constant fingerprints, iced-x86 disasm |
| Safety | In-memory quarantine. Samples are never executed. Passworded ZIPs decrypt into process memory only |

The interactive TUI needs a real terminal. For this scripted write-up I drove the **same library path** the TUI uses (`collect_samples` → `investigate`) through a small headless example binary I keep in the Vanguard-RE repo. That matters: I did not invent a second analysis stack. I called Vanguard's public API with the Conti ZIP path and password `infected`, then read the ranking, triage, and deep-dive report it returned.

In practice my workflow looked like this:

1. Point Vanguard at `/Users/.../Malware/conti_locker/conti_locker.zip` with password `infected`.
2. Let `collect_samples` unpack 141 members into quarantined in-memory buffers (no runnable files on disk).
3. Run `investigate` with deep-dives on the top-scoring members and a large disasm instruction budget.
4. Read the ranking first, then the PE triage blocks, then the deep-dive sections (crypto, imports, strings, YARA, disasm insights).
5. Stream selected source files from the ZIP read-only (`main.cpp`, `chacha.c`, `locker.h`, `network_scanner.cpp`, `global_parameters.cpp`) so I could validate or reject each Vanguard claim.

### What Vanguard ranked first

Vanguard ranked both `decryptor.exe` variants at **93/100 ("critical")**. They were the only PE files in the tree. Everything else (source, logs, `.tlog`, `.obj`) was correctly treated as raw or low-interest.

| | Variant A | Variant B |
| --- | --- | --- |
| SHA256 | `9aed278f54f65e546fb9f7f34dff26d0835a0a21a5a4fc4c026bf84596ed277e` | `142cf75bc8dbfbd76f21f48b86ecbe11297e94071c9c55c1ee280d95c6ac6814` |
| MD5 | `31ee2755a455b1c8c743f51cec3845fd` | `04dce97942dfb520fb4c12527c82164b` |
| Size | 110,592 B | 158,720 B |
| ImpHash | `ac80ef907c0b25dff5f5ca08ca8f21ef` | `8d4376cd52bea4b09fa664f1bb329e91` |
| Compiled (PE) | 2020-09-15 | 2020-08-27 |
| Entry | `0x585e` | `0x11a00` |
| CRT linkage | statically linked | dynamic **debug** CRT (`ucrtbased`, `VCRUNTIME140D`, `MSVCP140D`) |
| Toolchain (Vanguard) | MSVC C/C++ (Rich header, 11 records) | MSVC C/C++ (Rich header, 10 records) |

I treated both as debug builds of the same ContiLocker_v2 project. Vanguard surfaced the shared PDB path `c:\source\ContiLocker_v2\Debug\decryptor.pdb`, and Variant B still carried debug CRT names plus a `.textbss` incremental-link section. These are developer artifacts, not polished campaign payloads. That shaped how I later wrote the IOC tab.

### PE and section analysis in Vanguard

Vanguard's formal PE parse told me both binaries were PE32 (x86), unsigned, MSVC console apps. I used its section entropy map to rule packing in or out:

```text
Variant A (110,592 B), Vanguard section report
  .text    vsize=0x10d5e  entropy=6.64
  .rdata   vsize=0x6492   entropy=4.82
  .data    vsize=0x2b90   entropy=0.87
  .reloc   vsize=0x11fc   entropy=6.53
```

The peak entropy (~6.6) sits below the ~7.0+ band Vanguard treats as packed or encrypted. I concluded Conti was not relying on whole-file packing. Obfuscation lived elsewhere, which the disasm insight later confirmed.

### Crypto findings from Vanguard's scanner

On both binaries, Vanguard's crypto engine reported:

| Finding | Confidence | Evidence Vanguard cited |
| --- | --- | --- |
| ChaCha20/Salsa20 (stream) | 92 | `"expand 32-byte k"` sigma constant |
| Windows CryptoAPI/CNG (api) | 72 | `CryptAcquireContextA`, `CryptDecrypt`, `CryptImportKey` |

That combination is exactly what I wanted Vanguard to catch: a stream-cipher constant plus CryptoAPI key-import imports. I then opened `chacha20/chacha.c` from the archive. It is D. J. Bernstein's public-domain `chacha-merged.c`, keyed at 256 bits:

```c
static const char sigma[16] = "expand 32-byte k";
...
if (kbits == 256) { /* recommended */ constants = sigma; }
```

Vanguard's fingerprint was exact. I then checked `locker/locker.h` for the per-file crypto record:

```c
typedef struct file_info {
    ...
    ECRYPT_ctx CryptCtx;
    BYTE ChachaIV[8];
    BYTE ChachaKey[32];
    BYTE EncryptedKey[524];
} FILE_INFO, *LPFILE_INFO;
```

Each victim file gets a fresh 256-bit ChaCha20 key and 64-bit IV. Conti RSA-encrypts that key into the 524-byte `EncryptedKey` blob using the operator public key through CryptoAPI. Vanguard also surfaced the `EncryptedKeySize` string in the interesting-strings list, which lined up with that struct field.

Because this artifact is the *decryptor*, the imports are `CryptImportKey` and `CryptDecrypt` rather than `CryptEncrypt`. It still shares the crypto core with the locker. Vanguard did not invent that distinction for me. I had to interpret the import set against the module name and the source.

Vanguard's disassembly insight also flagged an **"XOR loop / inline decryption / string decoding" pattern (12 hits)**. I mapped that to Conti's compile-time string obfuscation in `MetaString.h` and `MetaRandom2.h`. That explained why Vanguard's plaintext string yield looked thin and why I leaned on its import and crypto views more than its string dump.

### Network and spreading signals Vanguard gave me

Vanguard tagged a **"Network / C2" capability (confidence 52)** from `WSASocketW`, and listed imports for `NetShareEnum` (NETAPI32), `GetIpNetTable` (IPHLPAPI), and `WSAIoctl` / `WSAAddressToStringW` (WS2_32). Alone, that set could mean many things. I used it as a hypothesis, then opened `network_scanner.cpp`.

That file is an SMB (445/tcp) scanner and spreader built on I/O completion ports:

```c
#define SMB_PORT 445
#define STOP_MARKER 0xFFFFFFFF
enum COMPLETION_KEYS { START_COMPLETION_KEY = 1, CONNECT_COMPLETION_KEY = 2, TIMER_COMPLETION_KEY = 3 };
```

`GetIpNetTable` seeds hosts from the ARP cache. `NetShareEnum` enumerates shares. Share paths feed the encryption threadpool. `main.cpp` exposes the operating modes:

```c
enum EncryptModes { ALL_ENCRYPT = 10, LOCAL_ENCRYPT = 11, NETWORK_ENCRYPT = 12 };
```

and drives local and network work together:

```c
if (filesystem::EnumirateDrives(&DriveList))
    hLocalSearch = CreateThread(NULL, 0, filesystem::StartLocalSearch, &DriveList, 0, NULL);
TAILQ_FOREACH(String, &g_HostList, Entries)
    network_scanner::EnumShares(String->wszString, &ShareList);
TAILQ_FOREACH(ShareInfo, &ShareList, Entries)
    filesystem::SearchFiles(ShareInfo->wszSharePath, threadpool::NETWORK_THREADPOOL);
```

Vanguard had already shown me `CreateIoCompletionPort`, `GetQueuedCompletionStatus`, and `CreateThread` in the import table. Once I had the source, I could call that Conti's multithreaded IOCP encryption design, the same design that made Conti one of the faster lockers of its era.

### Anti-analysis: what Vanguard saw vs what only source showed

| Technique | Signal from Vanguard | What I confirmed in source |
| --- | --- | --- |
| Debugger evasion | `IsDebuggerPresent` (behavior, severity 55) | Present in decryptor import set |
| Dynamic API resolution | `LoadLibraryExW` + `GetProcAddress` (behavior, severity 40) | `GetApi.h`, `api.h`, `ntdll.h` |
| String obfuscation | XOR string-decode loop (insight, 12 hits) | `MetaString.h`, `MetaRandom2.h` |
| Userland unhooking | No direct PE signal | `antihook/antihooks.h`: `removeHooks(HMODULE)` |
| Inhibit recovery | Absent from decryptor deep-dive | `locker.h`: `DeleteShadowCopies()` |
| Process termination | Absent from decryptor deep-dive | `process_killer.h`: `KillAll()`, `GetWhiteListProcess()` |

I care about that last point as a Vanguard user. The locker module shipped mostly as `.obj` in this tree, so the decryptor PE correctly lacked static signals for shadow-copy deletion and process killing. Vanguard did not invent those behaviors. Absence of evidence stayed absence of evidence until I read the locker headers.

## Honest review of Vanguard

Conti was not only a malware exercise for me. It was a product test of my own tool. I am proud of what Vanguard got right here, and I am equally willing to name where it wasted my time or misled me. Automated triage is a first pass. I still have to read the evidence.

### Strengths I actually used

**Containment that I trust.** I pointed Vanguard at a passworded malware ZIP and never wrote a runnable Conti PE to disk. `collect_samples` kept members in RAM. For defensive research on a personal machine, that is the feature I care about most.

**Ranking that cut through noise.** ContiLocker_v2 is a 141-member mess of source, logs, and objects. Vanguard still put both `decryptor.exe` binaries at the top with score 93. That is the job I built it for: tell me what to open first.

**Crypto fingerprinting earned its keep.** The `"expand 32-byte k"` hit at confidence 92 was the single best static finding in this run. When I opened `chacha.c`, Vanguard was already sitting on the right cipher. That is the kind of win that shortens an analysis from hours to minutes.

**PE, toolchain, and ImpHash views were solid.** Rich-header MSVC detection, section entropy that correctly argued "not packed," and stable ImpHashes gave me a clean identity for both binaries before I touched source.

**Capability tags beat raw API lists.** Seeing crypto, file_drop, dyn_resolve, anti_debug, and a network tag in one place helped me form a hypothesis fast. I still had to interpret those tags, but I did not start from a blank import dump.

**Conservative absences.** Vanguard did not invent shadow-copy deletion or process killing on the decryptor PE. Those live in the locker module. I want a tool that stays quiet when the PE does not support a claim.

### Weaknesses Conti exposed (and how I plan to fix them)

**1. The critical risk label lies.**  
When Conti scored 93, Vanguard printed a top-line label that claimed "injection / hollow patterns." That wording is wrong for this sample. The classic injection triad (`VirtualAllocEx`, `WriteProcessMemory`, `CreateRemoteThread`) is absent. I checked my own scoring code after this run. The bug is blunt: any score at or above 90 is hard-coded to that injection phrase, whether injection was matched or not. That is my mistake, not Conti's.

*Plan:* I will rewrite `risk_label` so the high-severity text reflects the actual matched behaviors and top capability IDs (for Conti that should read something like crypto / file_drop / network, not injection). The score can stay numeric. The prose must stop inventing techniques.

**2. Builtin YARA still fires noisy false positives.**  
Vanguard raised `Suspicious_Delphi_CODE_Section` on an MSVC C++ binary whose own toolchain view already said MSVC. I should never have to reconcile that contradiction by hand.

*Plan:* I will gate that rule behind Delphi toolchain signals (or drop it from the default ruleset), and I will add a "contradicts toolchain" demotion so a Delphi hit cannot outrank a high-confidence MSVC Rich-header result.

**3. Network capability naming is too C2-shaped.**  
Vanguard tagged Conti's SMB spreader as "Network / C2." Conti has no locker beacon. The real story is intranet SMB share discovery and encryption. The imports were useful. The label pushed me toward the wrong mental model until I read `network_scanner.cpp`.

*Plan:* I will split network capabilities into narrower tags (for example `smb_enum`, `socket_client`, `http_client`, `c2_suspect`) and reserve "C2" for stronger evidence such as hardcoded endpoints, beacon-shaped strings, or confirmed C2 libraries.

**4. Source-tree and build-artifact noise.**  
Vanguard scored many `.tlog` and text members around 35 as unrecognized or DOS-like. That is technically consistent with "raw blob" handling, but it pollutes ranking when I feed it a whole Visual Studio tree.

*Plan:* I will add content-class filters (skip or demote compiler logs, `.tlog`, `.obj`, `.pdb`, and obvious source extensions unless I ask for a full dump) so ranking stays focused on PE/ELF/Mach-O and packed containers.

**5. Native IAT heuristics are blind on managed malware.**  
I also ran Vanguard against an AgentTesla sample while choosing Conti. It correctly identified .NET, then scored the PE as 0 / "benign" because the import table is basically `mscoree.dll!_CorExeMain`. Conti hid that weakness. AgentTesla did not.

*Plan:* I will add a managed triage path (CLR metadata, module names, suspicious .NET type/method strings, and a .NET-aware score floor) so Vanguard stops calling packed or obfuscated stealers "benign" just because the native IAT is empty.

**6. Disassembly function recovery is still shallow.**  
On Conti, Vanguard gave me useful XOR-loop insights near the entry path, but function recovery stayed thin (essentially an entry blob). That is fine for first-pass triage. It is not yet a substitute for a real RE session when I need call-graph structure.

*Plan:* I will keep improving function boundary heuristics and cluster labels, but I am not pretending Vanguard replaces IDA or Ghidra. My goal is better "where should I look next," not full decompilation.

**7. No first-class headless CLI in the shipped binary.**  
I had to drive this write-up through the library API / example binary because the TUI refuses non-interactive terminals. That is awkward for reproducible reports and CI-style corpus tests.

*Plan:* I will promote the headless path to a supported `vanguard investigate <path>` (or similar) subcommand that emits the same report the TUI deep-dive shows, so I can script write-ups without a fake terminal.

### Conti-specific caveats I still had to apply by hand

- Builder templates (`__DECRYPT_NOTE__`, `__MUTEX_NAME__`, `.EXTEN`) are not live campaign IOCs. Vanguard surfaced them correctly. I had to interpret them as placeholders.
- An empty high-confidence network-IOC list was the right answer. Conti is human-operated. The locker has no beacon.
- I still needed the leaked source to turn "suggestive imports" into a confident SMB-spreader narrative. Vanguard got me most of the way. It did not finish the story alone.

## Result

### What I got from using Vanguard on Conti

| Behaviour | What Vanguard told me (static only) | Leaked source | My verdict |
| --- | --- | --- | --- |
| Stream cipher = ChaCha20 | `"expand 32-byte k"`, confidence 92 | `chacha.c` (DJB, 256-bit) | Exact |
| Asymmetric key wrap | CryptoAPI `CryptImportKey` / `CryptDecrypt` | `EncryptedKey[524]` in `FILE_INFO` | Exact |
| SMB network spreading | Inferred from `NetShareEnum` / `GetIpNetTable` | `network_scanner.cpp`, `SMB_PORT 445` | Confirmed |
| Multithreaded IOCP encryption | IOCP + thread imports | `threadpool.*`, `main.cpp` | Confirmed |
| Compile-time string encryption | XOR-decode insight | `MetaString.h` | Confirmed |
| Dynamic API resolution | Behavior match | `GetApi.h` | Confirmed |
| Shadow-copy deletion / proc-kill | Not present on decryptor | `locker.h`, `process_killer.h` | Correctly absent |

Vanguard got me from a 141-member passworded ZIP to a ranked, hashed, capability-tagged deep-dive in one containment-safe pass. The leaked source then let me grade the tool. Crypto and PE/toolchain views were excellent. Network labeling and the critical risk phrase still need work that I own. The missing locker-only behaviors stayed missing, which I still count as a strength.

### Kill chain I reconstructed (source + Vanguard)

```text
conti locker.exe
├── Resolve imports via hashed-API loader (GetApi.h)         [T1106, T1027]
├── removeHooks(): reload clean ntdll, strip EDR hooks       [T1562.001]
├── IsDebuggerPresent() anti-debug check                     [T1622]
├── Decode obfuscated strings on stack (MetaString XOR)      [T1027]
├── Mode = ALL_ENCRYPT | LOCAL_ENCRYPT | NETWORK_ENCRYPT
├── process_killer::KillAll() (spare whitelist)              [T1489]
├── DeleteShadowCopies()                                     [T1490]
├── Enumerate drives + ARP table (GetIpNetTable)             [T1083, T1016]
├── NetShareEnum() over SMB/445 → discover shares            [T1135, T1021.002]
├── For each file (local + network share):
│   ├── ChaCha20 key(32B) + IV(8B) = fresh random
│   ├── Encrypt file body with ChaCha20                      [T1486]
│   ├── RSA-encrypt the ChaCha key (CryptImportKey pubkey)
│   └── Append EncryptedKey[524] + rename with extension
└── Drop ransom note (__DECRYPT_NOTE__ template)             [T1486]
```

### Defensive outputs

I packaged the hashes, ImpHashes, host artifacts, and template-config caveats on the **IOCs** tab. On the **Sigma** tab I wrote detection content from the validated Vanguard findings: YARA for the ChaCha20 sigma constant plus Conti-specific markers, and Sigma for shadow-copy deletion, SMB fan-out, and related host behaviors.

For me, Conti was both a ransomware analysis and a hard look at my own tooling. Vanguard earned its place as my first pass. It kept me inside a static-only containment model, ranked the right PE files out of a noisy build tree, and nailed the ChaCha20 fingerprint hard enough that source review became confirmation instead of a cold start. It also showed me exactly which labels and heuristics I still need to fix before I trust the prose as much as I trust the evidence.
