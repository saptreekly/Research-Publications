I used my static analysis CLI, [Vanguard-RE](/projects/vanguard-re), on a passworded Raccoon Stealer v2 pack without executing anything. This write-up is both a stealer analysis and a product stress test: after Conti and WannaCry (high-scoring ransomware paths), I wanted a multi-hash crimeware pack that lives on delayed imports, browser DB strings, and HTTP exfil instead of ChaCha/RSA file encryption.

**Status: analysis in progress.** The Vanguard pass and tool log are done. Config/C2 decode and detection packaging are next.

## Context

| Field | Value |
| --- | --- |
| Family | Raccoon Stealer v2 (commodity stealer) |
| Archive | `Raccoon.Stealer.v2.sha.zip` |
| Members | 21 (20 PE32 + 1 raw outlier) |
| Shared ImpHash | `4ec5227a81c3e90d891321c143c67557` (20/21) |
| Representative SHA256 | `0123b26df3c79bac0a3fda79072e36c159cfd1824ae3fd4b7f9dea9bda9c7909` |
| Representative size | 57,344 B (cluster mostly 56,832 B) |
| Format | PE32 x86 GUI, MSVC (Rich header) |
| Containment | Static-only. Outer ZIP stayed in process memory |

Command I ran for the logged pass:

```bash
vanguard Raccoon.Stealer.v2.sha.zip -p infected --deep 5 --disasm-count 6000 --min-deep-score 40
```

Full stdout and a structured product log live under `.van_out/raccoon-stealer-v2.*` in the repo workspace (local research notes, not shipped to Pages).

## What I set out to do

1. Drive a stealer pack through the shipped CLI the same way Conti and WannaCry were driven.
2. See whether ImpHash clustering and string/YARA views compensate when the static IAT is deliberately thin.
3. Score Vanguard honestly where ransomware stress tests had flattered it.
4. Turn validated indicators into IOC/Sigma tabs once config decode is finished.

## What Vanguard got right

**Containment and speed.** Twenty-one members from a passworded ZIP in about 0.74s, ~58 MB RSS, exit 0. Nothing executed. That remains the feature I trust most.

**ImpHash clustering.** Twenty of twenty-one members share `4ec5227a81c3e90d891321c143c67557`. Vanguard emitted a single cluster with a VirusTotal ImpHash pivot. For a stealer “hash pack,” that is the correct first analytic move.

**Structural PE triage.** On the representative sample: PE32 x86, unsigned, entry `0x7486`, MSVC Rich header (conf 80), section entropies in the mid-6s (not a whole-file packer story). Behaviors correctly flagged `LoadLibraryW` + `GetProcAddress`.

**String evidence that tells the stealer story.** Interesting-strings consistently surfaced `Login Data`, `wallet.dat`, `SOFTWARE\Microsoft\Cryptography`, the Uninstall inventory key, and WinInet/Crypt32/Bcrypt DLL names. That is browser credential theft + wallet theft + host inventory, even when the formal capability engine only said `dyn_resolve` / `exec`.

**Outlier demotion.** `9ee50e94…` (55,360 B) failed PE parse, scored 35 as raw/DOS-MZ blob, and did not pollute the ImpHash cluster. Good.

## Where Vanguard under-performed

**Absolute scores are too low for this class.** The whole PE cluster capped at **50** (`likely malicious tooling`). Conti decryptors were 93; WannaCry dropper 96. Stealers that delay-load WinInet and speak in browser-path strings should not sit in the same band as lukewarm “tooling.”

**Genre mislabel.** Deep-dive YARA hit `RAT_WinINet_C2_Imports`. The API shape is fair. Calling it a RAT is not. This pack’s public story is stealer/exfil.

**Deep-dive budget slipped.** I asked for `--deep 5` and got **20** deep dives. Every score-50 PE above `--min-deep-score 40` was expanded. If `--deep` is a hard budget, tied scores currently blow past it.

**Secrets heuristics were noisy.** Multiple `[PASS?]` / `[KEY?]` hits on short opaque blobs. Treat as “decode these,” not as recovered passwords.

**No plaintext C2 in the top string window.** Exfil APIs are visible (`InternetOpenUrlA/W`, `HttpSendRequestW`). Destinations look encoded (recurring long Base64 at ~`0xb958`). That decode is still on my list.

## Provisional analytic read (pending decode)

Static evidence so far supports a **commodity stealer** reading, not ransomware and not a full interactive RAT:

1. Thin static IAT with dynamic resolution.
2. Strings aimed at Chromium `Login Data`, `wallet.dat`, and crypto/product inventory paths.
3. WinInet HTTP client imports resolved from strings / delay-load names.
4. Near-duplicate PE cluster under one ImpHash (build factory / pack distribution).

I am not publishing network IOCs until the encoded config blob is decoded and checked. ImpHash + member SHA256s are already hunt-worthy.

## Honest Vanguard scorecard (this sample)

| Area | Grade | Note |
| --- | --- | --- |
| Containment / ZIP ingest | A | Sub-second, in-memory |
| ImpHash clustering | A | 20/21 |
| Ranking usefulness | B− | Cluster yes; score ceiling too low |
| Capability typing | C+ | Misses stealer boost from strings |
| YARA naming | C | RAT label on stealer APIs |
| Secrets heuristics | C | Noisy |
| `--deep` hard cap | D | 5 requested → 20 ran |
| Speed | A | Fine for packs this size |

## Still to write

- Decode recurring Base64/config blob; confirm C2/path structure
- IOC tab (ImpHash, representative hashes, mutex/path strings once validated)
- Sigma tab aimed at browser DB access + WinInet stealer patterns without over-fitting one hash
- Optional: one side-by-side with AgentTesla (prior .NET negative control) to show native vs managed stealer scoring

## How to read this page

This is a defensive static pass and a tool review. It is not a claim about current campaigns, victims, or operators beyond what the sample bytes support.
