I used my static analysis CLI, [Vanguard-RE](/projects/vanguard-re), to reverse-engineer the leaked Conti (ContiLocker_v2) ransomware build tree without executing anything. This write-up walks through why I picked Conti, how I drove Vanguard before and after its CLI rewrite, what it surfaced, and how I checked those findings against the leaked C++ source.

## Context

I keep a local malware corpus under password-protected ZIPs (`infected`) for defensive research. I needed a sample that would stress Vanguard end to end: in-memory ZIP quarantine, PE triage, ImpHash, entropy, IAT heuristics, crypto fingerprinting, network IOC scanning, and iced-x86 disassembly.

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

## What I set out to do

My goals were:

1. Run the full Conti archive through Vanguard under containment-first rules (map and parse only, never execute, keep ZIP members in RAM).
2. Let Vanguard rank members, deep-dive the top hits, and report crypto, imports, strings, signatures, and disassembly insights.
3. Cross-check every high-confidence Vanguard claim against the leaked source so I could document where the tool was exact, where it was suggestive, and where the wording overreached.
4. Produce defensive artifacts (IOCs, YARA, Sigma) from those validated findings.
5. After shipping a path-first CLI, re-run Conti the same way and compare the old workflow to the new one.

I was not trying to recover a victim environment or recover encrypted files. I was stress-testing my own triage pipeline on a well-documented ransomware family.

## How I analyzed Conti with Vanguard

### Before: TUI plus a hacked headless path

My first Conti pass happened when Vanguard was still a `ratatui` TUI. That UI refuses to launch without an interactive terminal, so for this write-up I had to call the library path (`collect_samples` → `investigate`) through a throwaway example binary. It worked, but it was awkward. I could not just type `vanguard conti_locker.zip` in a script or CI job and capture a report.

### Now: path-first CLI

I replaced the TUI with a path-first CLI. Conti now looks like this:

```bash
vanguard /path/to/conti_locker.zip -p infected --deep 3 --disasm-count 6000
```

That is the same investigate pipeline I used before, just promoted to the shipped `vanguard` binary. ZIP members still stay in process memory. Nothing is executed. The report prints ranking, ImpHash clusters, triage, and deep-dives to stdout, which is exactly what I want for reproducible notes.

| Pillar | What I relied on |
| --- | --- |
| Speed | `memmap2` zero-copy I/O and a focused static pipeline |
| Accuracy | Formal PE parsing (`goblin`), ImpHash, Shannon entropy, IAT capability tags, crypto-constant fingerprints, iced-x86 disasm |
| Safety | In-memory quarantine. Samples are never executed. Passworded ZIPs decrypt into process memory only |

In practice my Conti workflow is now:

1. Run `vanguard conti_locker.zip -p infected --deep 3 --disasm-count 6000`.
2. Read ranking first, then the PE triage blocks, then the deep-dive sections (crypto, imports, strings, signatures, disasm insights).
3. Stream selected source files from the ZIP read-only (`main.cpp`, `chacha.c`, `locker.h`, `network_scanner.cpp`, `global_parameters.cpp`) so I can validate or reject each Vanguard claim.

### Before vs now on Conti

| Dimension | First pass (TUI era) | Re-run (CLI) |
| --- | --- | --- |
| How I launched it | Library API via `examples/headless.rs` | `vanguard <path> -p infected` |
| Interactive UI | Required for the shipped binary | Removed. CLI only |
| Members collected | 141 | 141 |
| Top ranking | Both `decryptor.exe` at score 93 | Same |
| Top-line risk phrase | Claimed injection / hollow patterns | Still the same overstated phrase |
| ChaCha20 fingerprint | `"expand 32-byte k"`, conf 92 | Same |
| CryptoAPI imports | `CryptImportKey` / `CryptDecrypt` | Same |
| Delphi signature hit | `Suspicious_Delphi_CODE_Section` FP | Still fires |
| Build-artifact noise | Many `.tlog` / source rows at score 35 | Still present |
| Network tag | "Network / C2" from `WSASocketW` | Still C2-shaped |
| Reproducibility | Fragile outside a real terminal | Easy to script and redirect |

The Conti *malware* story did not change between passes. The *tooling* story did. The CLI rewrite fixed the biggest workflow complaint from my first write-up. The heuristic and labeling issues Conti exposed are still open, and I am keeping them on the roadmap below.

### What Vanguard ranked first

Vanguard ranked both `decryptor.exe` variants at **93/100**. They were the only PE files in the tree. Everything else (source, logs, `.tlog`, `.obj`) fell into raw or low-interest buckets.

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

Vanguard's disassembly insight also flagged an XOR-loop / inline string-decoding pattern (12 hits). I mapped that to Conti's compile-time string obfuscation in `MetaString.h` and `MetaRandom2.h`. That explained why Vanguard's plaintext string yield looked thin and why I leaned on its import and crypto views more than its string dump.

### Network and spreading signals Vanguard gave me

Vanguard tagged a "Network / C2" capability (confidence 52) from `WSASocketW`, and listed imports for `NetShareEnum` (NETAPI32), `GetIpNetTable` (IPHLPAPI), and `WSAIoctl` / `WSAAddressToStringW` (WS2_32). Alone, that set could mean many things. I used it as a hypothesis, then opened `network_scanner.cpp`.

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

**Containment that I trust.** I pointed Vanguard at a passworded malware ZIP and never wrote a runnable Conti PE to disk. Members stay in RAM. For defensive research on a personal machine, that is the feature I care about most.

**A CLI I can finally script.** The biggest before/after win is operational. I can now run Conti as `vanguard conti_locker.zip -p infected`, redirect stdout, and paste the ranking into notes. I no longer need a fake TTY or a private example binary to publish a reproducible analysis.

**Ranking that cut through noise.** ContiLocker_v2 is a 141-member mess of source, logs, and objects. Vanguard still put both `decryptor.exe` binaries at the top with score 93. That is the job I built it for: tell me what to open first.

**Crypto fingerprinting earned its keep.** The `"expand 32-byte k"` hit at confidence 92 was the single best static finding in this run. When I opened `chacha.c`, Vanguard was already sitting on the right cipher.

**PE, toolchain, and ImpHash views were solid.** Rich-header MSVC detection, section entropy that correctly argued "not packed," and stable ImpHashes gave me a clean identity for both binaries before I touched source.

**Conservative absences.** Vanguard did not invent shadow-copy deletion or process killing on the decryptor PE. Those live in the locker module. I want a tool that stays quiet when the PE does not support a claim.

### Weaknesses Conti still exposes (and how I plan to fix them)

The CLI rewrite closed the workflow gap. Re-running Conti showed the same analysis bugs as the first pass. That is useful. It means the report still has unfinished product work, not just nostalgia about the TUI.

**1. The critical risk label still lies.**  
When Conti scored 93, Vanguard printed a top-line label that claimed injection / hollow patterns. That wording is wrong for this sample. The classic injection triad (`VirtualAllocEx`, `WriteProcessMemory`, `CreateRemoteThread`) is absent. I checked my own scoring code. Any score at or above 90 is hard-coded to that injection phrase, whether injection was matched or not.

*Plan:* Rewrite `risk_label` so the high-severity text reflects the actual matched behaviors and top capability IDs (for Conti that should read crypto / file_drop / network, not injection). The score can stay numeric. The prose must stop inventing techniques.

**2. Builtin signature noise is still there.**  
Vanguard still raises `Suspicious_Delphi_CODE_Section` on an MSVC C++ binary whose own toolchain view already said MSVC. I should never have to reconcile that contradiction by hand.

*Plan:* Gate that rule behind Delphi toolchain signals (or drop it from the default ruleset), and add a "contradicts toolchain" demotion so a Delphi hit cannot outrank a high-confidence MSVC Rich-header result.

**3. Network capability naming is still too C2-shaped.**  
Vanguard still tags Conti's SMB spreader as "Network / C2." Conti has no locker beacon. The real story is intranet SMB share discovery and encryption. The imports were useful. The label still pushes me toward the wrong mental model until I read `network_scanner.cpp`.

*Plan:* Split network capabilities into narrower tags (for example `smb_enum`, `socket_client`, `http_client`, `c2_suspect`) and reserve "C2" for stronger evidence such as hardcoded endpoints, beacon-shaped strings, or confirmed C2 libraries.

**4. Source-tree and build-artifact noise.**  
Vanguard still scores many `.tlog` and text members around 35 as unrecognized or DOS-like. That is technically consistent with "raw blob" handling, but it pollutes ranking when I feed it a whole Visual Studio tree.

*Plan:* Add content-class filters (skip or demote compiler logs, `.tlog`, `.obj`, `.pdb`, and obvious source extensions unless I ask for a full dump) so ranking stays focused on PE/ELF/Mach-O and packed containers.

**5. Native IAT heuristics are still blind on managed malware.**  
I re-checked AgentTesla with the new CLI while re-running Conti. Vanguard still identifies .NET poorly on that sample and scores it 0 / "benign" because the import table is basically `mscoree.dll!_CorExeMain`. Conti hides that weakness. AgentTesla does not.

*Plan:* Add a managed triage path (CLR metadata, module names, suspicious .NET type/method strings, and a .NET-aware score floor) so Vanguard stops calling obfuscated stealers "benign" just because the native IAT is empty.

**6. Disassembly function recovery is still shallow.**  
On Conti, Vanguard gives me useful XOR-loop insights near the entry path, but function recovery stays thin (essentially an entry blob). That is fine for first-pass triage. It is not yet a substitute for a real RE session when I need call-graph structure.

*Plan:* Keep improving function boundary heuristics and cluster labels, but I am not pretending Vanguard replaces IDA or Ghidra. My goal is better "where should I look next," not full decompilation.

### Conti-specific caveats I still had to apply by hand

- Builder templates (`__DECRYPT_NOTE__`, `__MUTEX_NAME__`, `.EXTEN`) are not live campaign IOCs. Vanguard surfaced them correctly. I had to interpret them as placeholders.
- An empty high-confidence network-IOC list was the right answer. Conti is human-operated. The locker has no beacon.
- I still needed the leaked source to turn "suggestive imports" into a confident SMB-spreader narrative. Vanguard got me most of the way. It did not finish the story alone.

## What came out of the Conti re-run

### Validated findings

| Behaviour | What Vanguard told me (static only) | Leaked source | My verdict |
| --- | --- | --- | --- |
| Stream cipher = ChaCha20 | `"expand 32-byte k"`, confidence 92 | `chacha.c` (DJB, 256-bit) | Exact |
| Asymmetric key wrap | CryptoAPI `CryptImportKey` / `CryptDecrypt` | `EncryptedKey[524]` in `FILE_INFO` | Exact |
| SMB network spreading | Inferred from `NetShareEnum` / `GetIpNetTable` | `network_scanner.cpp`, `SMB_PORT 445` | Confirmed |
| Multithreaded IOCP encryption | IOCP + thread imports | `threadpool.*`, `main.cpp` | Confirmed |
| Compile-time string encryption | XOR-decode insight | `MetaString.h` | Confirmed |
| Dynamic API resolution | Behavior match | `GetApi.h` | Confirmed |
| Shadow-copy deletion / proc-kill | Not present on decryptor | `locker.h`, `process_killer.h` | Correctly absent |

Vanguard got me from a 141-member passworded ZIP to a ranked, hashed, capability-tagged deep-dive in one containment-safe pass. After the CLI rewrite, that same pass is something I can run again tomorrow without ceremony. The leaked source still grades the tool. Crypto and PE/toolchain views remain excellent. Network labeling and the critical risk phrase still need work that I own.

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

For me, Conti remains both a ransomware analysis and a hard look at my own tooling. The CLI rewrite made Vanguard something I actually want to use for published reports. The Conti evidence still shows where the prose and heuristics need to catch up to the evidence.
