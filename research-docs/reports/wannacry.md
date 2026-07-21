I picked WannaCry as the second Vanguard stress test because Conti never exercised the features I claim as unique: carving an encrypted ZIP out of a PE resource section, recovering the ZIP password from the sample's own strings, and promoting Tor onion endpoints out of an unlocked config blob. Conti was a native MSVC decryptor with leaked source. WannaCry is a self-contained dropper that only becomes readable after Vanguard does recursive in-memory unpacking.

I also ran AgentTesla and Mirai on the same CLI so I could say honestly where Vanguard still collapses.

## Context

After the Conti write-up and the path-first CLI rewrite, I wanted a sample that would punish shallow PE triage. WannaCry is famous for packing its payload as an encrypted ZIP inside `.rsrc`, protected by the hard-coded password `WNcry@2ol7`. If Vanguard cannot carve that archive, crack it from strings, and keep every member in RAM, the rest of the analysis is theater.

| Field | Value |
| --- | --- |
| Family | WannaCry / WannaCryptor |
| Archive | `Ransomware.WannaCry.zip` |
| Dropper SHA256 | `ed01ebfbc9eb5bbea545af4d01bf5f1071661840480439c6e5babe8e080e41aa` |
| Dropper MD5 | `84c82835a5d21bbcf75a61706d8ab549` |
| ImpHash | `68f013d7437aa653a8a98a05807afeb1` |
| Size | 3,514,368 B |
| Format | PE32 x86 GUI, MSVC |
| Members after unlock | 37 (dropper + 36 embedded ZIP members) |
| Containment | Static-only. Passworded outer ZIP and inner ZIP both stayed in process memory |

Contrast samples I ran the same way:

| Sample | Why I included it | Headline Vanguard result |
| --- | --- | --- |
| WannaCry | Embedded-ZIP + onion + BTC stress test | Dropper 96, unlocked 36 members, password recovered |
| AgentTesla | Managed .NET stealer | Toolchain `.NET` conf 100, threat score **0 / benign** |
| IoT.Mirai | Source tree + multi-arch ELF bots | Source files rank at 35. ELF bots score **0 / benign** |

## What I set out to do

1. Drive WannaCry through the shipped CLI with a deep budget and no interactive UI.
2. Force every containment path: outer ZIP decrypt, PE map, resource carve, inner ZIP password recovery, recursive member triage.
3. Measure what Vanguard got for free versus what I still had to interpret.
4. Use AgentTesla and Mirai as negative controls for managed and ELF scoring.
5. Turn the validated findings into IOCs and detection content, and keep an honest product review.

Command I actually ran:

```bash
vanguard Ransomware.WannaCry.zip -p infected --deep 5 --disasm-count 8000 --min-deep-score 50
```

## How I used Vanguard on WannaCry

### The unpack that Conti never needed

Vanguard collected **37 members** from one passworded outer ZIP. The dropper alone was not the story. Inside the PE, at file offset `26062`, Vanguard carved `embedded-1.zip` (span ~3.48 MB), marked every member encrypted, tried plaintext strings from the sample as passwords, and recovered **`WNcry@2ol7`**. It then extracted 36 members in memory, including:

- `u.wnry` (encryptor / ransom UI PE, score 87)
- `taskse.exe` / `taskdl.exe` (helpers)
- `c.wnry` (Tor/onion config)
- `b.wnry`, `s.wnry`, `t.wnry`, `r.wnry`
- 28 localized `msg/m_*.wnry` ransom note resources

That is the strongest single pass I have seen from Vanguard. Conti ranked two debug decryptors. WannaCry made the tool invent a whole second corpus out of one PE and then analyze it.

### Ranking and PE triage

| Rank | Member | Score | What Vanguard said |
| --- | --- | --- | --- |
| 0 | dropper `.exe` | 96 | persistence / file_drop / dyn_resolve |
| 1 | `u.wnry` | 87 | exec / file_drop / c2 |
| 2 | `taskse.exe` | 47 | dyn_resolve |
| … | language packs, configs | 35 | unrecognized / raw |
| last PE | `taskdl.exe` | **0** | benign / low interest |

On the dropper, Vanguard got the structural story right before I touched any deep-dive pane:

- PE32 x86, unsigned, MSVC (Rich header, conf 100)
- `.rsrc` entropy **8.00** with a packer hint: "1 high-entropy section(s) (packed/compressed/encrypted payload likely)"
- Behaviors: service persistence (`CreateServiceA`, `StartServiceA`) and dynamic API resolve
- Capabilities: persistence (88), file_drop (76), dyn_resolve, exec
- Suspicious APIs included `VirtualAlloc`, `VirtualProtect`, `CreateProcessA`, `CreateServiceA`, `RegSetValueExA`

`u.wnry` looked like the encryptor / UI stage: `ShellExecuteA`, `URLDownloadToFileA`, `CreateProcessA`, screenshot APIs (`BitBlt`), and ransom-note strings about bitcoin.

### Network IOCs after recursive unlock

Because Vanguard merges hostname-class IOCs from unlocked children into the parent deep-dive, the dropper report included Tor endpoints that actually live in `c.wnry`:

| Kind | Value | Confidence |
| --- | --- | --- |
| BTC | `115p7UMMngoj1pMvkpHijcRdfJNXj6LrLn` | 96 |
| BTC | `12t9YDPgwueZ9NyMgw519p7AA8isjr6SMw` | 96 |
| BTC | `13AM4VW2dhxYgXeQepoHkHSQuy6NgaEb94` | 96 |
| ONION | `57g7spgrzlojinas.onion` | 95 |
| ONION | `76jdd2ir2embyv47.onion` | 95 |
| ONION | `cwwnhwhlz52maqm7.onion` | 95 |
| ONION | `gx7ekbenv2riucmf.onion` | 95 |
| ONION | `xxlvbrloxvriy2c5.onion` | 95 |
| URL | `https://dist.torproject.org/torbrowser/6.5.1/tor-win32-0.2.9.10.zip` | 85 |
| URL | `http://www.btcfrog.com/qr/bitcoinPNG.php?address=%s` | 85 |

The `c.wnry` deep-dive confirmed the onion list as a single config string. That is exactly the recursive IOC promotion I wanted Conti to prove and could not, because Conti had no embedded payload pack.

### Crypto and secrets

| Finding | Confidence | Notes |
| --- | --- | --- |
| AES forward S-box | 95 | Strong static AES fingerprint on the dropper |
| CRC32 polynomial | 55 | Weak / common, treat as low value |
| Secret candidate `WNcry@2ol7` | 96 | Same string used to unlock the embedded ZIP |

Vanguard did not just list the password as a curiosity. It used it. `recovered_password: Some("WNcry@2ol7")` on the embedded archive record is the product moment that Conti could never surface.

### Strings that lined up with the kill chain

From the dropper interesting-strings list I cared about:

- `tasksche.exe`, `taskdl.exe`, `taskse.exe`
- `Global\MsWinZonesCacheCounterMutexA`
- `cmd.exe /c "%s"`
- every `msg/m_*.wnry` locale name
- `b.wnry` / `c.wnry` / `r.wnry` / `s.wnry` / `t.wnry` / `u.wnry`

From `u.wnry`:

- `Send $%d worth of bitcoin to this address:`
- `How to buy bitcoins?`
- Tor Browser download URL and payment-helper URLs

### Contrast runs that hurt

**AgentTesla.** Vanguard correctly said `.NET` (conf 100) via BSJB / mscorlib / `mscoree.dll`, then scored the sample **0 / benign** because the native IAT is only `_CorExeMain`. Conti hid this. WannaCry does not fix it. Managed malware is still a blind spot.

**Mirai.** The ZIP is mostly leaked source plus multi-arch ELF bots. Source files got the default unrecognized score of 35 and filled the top of the ranking. The ELF bots Vanguard *did* parse as ELF (ARM, MIPS, x86) scored **0 / benign**. So on a Linux botnet tree, my "what should I open first" answer was `admin.go`, not a bot binary. That is ranking failure, not a formatting failure.

## Honest review: where WannaCry pushed Vanguard harder than Conti

### Ultimate strengths this run proved

1. **Recursive in-memory unpacking is real.** Carve → password guess from strings → unlock → re-triage, all without writing runnable files. Conti never needed this. WannaCry lives or dies by it.
2. **Credential recovery that changes the analysis.** Recovering `WNcry@2ol7` was not a side note. It unlocked the encryptor, helpers, config, and 28 language packs.
3. **Child IOC promotion works.** Onions in `c.wnry` appeared on the parent deep-dive with confidence 95. That is how a dropper report should behave.
4. **High-value IOCs are high quality when the shape is right.** Checksum-validated BTC addresses and `.onion` hostnames were clean and actionable.
5. **Entropy + resource size told the truth early.** `.rsrc` at entropy 8.00 plus a 3.4 MB raw size correctly screamed "encrypted payload inside."
6. **Windows service persistence heuristics fired correctly.** `CreateServiceA` + `StartServiceA` is the right story for the WannaCry service install path.
7. **The CLI made the stress test reproducible.** One command, redirected stdout, no TUI ceremony.

### Ultimate weaknesses this run exposed

1. **Critical labels still invent injection.** Score 96 still printed "injection / hollow patterns" even though matched behaviors were service persistence and dynamic resolve. Same Conti bug, still open.
2. **Resource / language-pack noise floods ranking.** Twenty-eight `m_*.wnry` blobs at score 35 bury `taskdl.exe` and make the ranking look like a directory listing. I need content-class demotion for message packs and configs once a PE parent exists.
3. **Small helper PEs can fall to zero.** `taskdl.exe` scored benign. That is dangerous if an analyst trusts the number more than the filename context.
4. **IOC extractor still emits garbage around real gold.** Truncated Microsoft schema fragments (`http://schemas.micr`, domain `osoft.com`) sat next to perfect onion and BTC hits. Precision on URL truncation needs work.
5. **C2 tagging is still too greedy.** `u.wnry` got a Network / C2 capability partly on `SendMessageA`, which is a UI message API, not command-and-control. `URLDownloadToFileA` is fairer signal. Conti's SMB-as-C2 problem has a cousin here.
6. **Managed and ELF scoring remain structurally weak.** AgentTesla (.NET) and Mirai ELF bots both scored 0 despite correct format/toolchain detection. Vanguard's threat model is still Windows IAT-shaped.
7. **Source trees outrank real binaries when both are "uninteresting."** Mirai source at 35 beat ELF bots at 0. Default raw-blob scoring is higher than "parsed but no suspicious imports," which is backwards for triage.

### What I plan to fix next

| Gap | Plan |
| --- | --- |
| Fake injection label | Make `risk_label` derive from matched behaviors / top capability IDs |
| Language-pack ranking noise | Demote known resource extensions and non-PE children under an unlocked parent unless `--full` |
| Helper PE score 0 | Add filename / sibling context boosts and a floor for PE children of a high-score dropper |
| Truncated URL IOCs | Require balanced hostnames and reject obvious schema truncations |
| Greedy C2 tags | Split `c2` from `url_download`, `ui_msg`, `smb_enum` |
| .NET score 0 | Managed triage path + score floor when toolchain is `.NET` with high confidence |
| ELF score 0 | ELF import / symbol / string heuristics, and never let raw source outrank parsed ELF/PE at equal interest |

## What came out of the run

### Kill chain Vanguard let me reconstruct statically

```text
WannaCry dropper (ed01ebf…exe)
├── High-entropy .rsrc holds encrypted ZIP (embedded-1.zip)
├── Password WNcry@2ol7 recovered from sample strings
├── Unlocks:
│   ├── u.wnry          encryptor / ransom UI
│   ├── taskse.exe      session helper
│   ├── taskdl.exe      cleanup helper
│   ├── c.wnry          Tor onion config + Tor Browser URL
│   ├── b/s/t/r.wnry    supporting payload/config blobs
│   └── msg/m_*.wnry    localized ransom notes
├── Installs Windows service (CreateServiceA / StartServiceA)
├── Drops tasksche.exe / helpers, uses MsWinZonesCacheCounterMutexA
├── Displays ransom UI with BTC addresses + onion payment sites
└── AES-related constants present in dropper image
```

I did not detonate WannaCry. Everything above is static, containment-safe, and re-runnable from the CLI.

### Compared with Conti

| Question | Conti | WannaCry |
| --- | --- | --- |
| Best Vanguard win | ChaCha20 constant + source validation | Embedded ZIP unlock + onion/BTC harvest |
| Needed leaked source? | Yes, to finish the SMB story | No. The sample unpacked itself under Vanguard |
| Ranking quality | Good on PEs, noisy on build logs | Good on dropper/`u.wnry`, noisy on language packs |
| Label honesty | Overstated injection | Same overstated injection |
| Hardest new failure | n/a | Helper PE and ELF/managed score-0 collapses |

### Defensive outputs

Hashes, BTC addresses, onions, the recovered ZIP password, mutex, and helper names are on the **IOCs** tab. Detection content for the dropper hash, embedded-password string, onion endpoints, and service creation path is on the **Sigma** tab.

WannaCry is the sample I will keep using when I claim Vanguard is more than a pretty IAT printer. Conti proved accuracy against source. WannaCry proved the containment and unpacking story is worth shipping. The contrast runs proved I still have to earn trust on .NET and ELF before I call the threat score anything close to ultimate.
