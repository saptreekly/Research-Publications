Actionable indicators from the WannaCry Vanguard stress-test. Prefer the dropper hash and Tor onions for hunting. The recovered ZIP password is a static unpacking artifact, not a live campaign secret by itself.

## File indicators

| Type | Value | Notes |
| --- | --- | --- |
| Dropper SHA256 | `ed01ebfbc9eb5bbea545af4d01bf5f1071661840480439c6e5babe8e080e41aa` | Classic WannaCry sample |
| Dropper MD5 | `84c82835a5d21bbcf75a61706d8ab549` | |
| ImpHash | `68f013d7437aa653a8a98a05807afeb1` | VT pivot |
| `u.wnry` SHA256 | `b9c5d4339809e0ad9a00d4d3dd26fdf44a32819a54abf846bb9b560d81391c25` | Encryptor / ransom UI |
| `u.wnry` MD5 | `7bf2b57f2a205768755c07f238fb32cc` | |
| `taskse.exe` SHA256 | `2ca2d550e603d74dedda03156023135b38da3630cb014e3d00b1263358c5f00d` | Helper |
| Embedded archive password | `WNcry@2ol7` | Recovered by Vanguard from sample strings |
| Helper / drop names | `tasksche.exe`, `taskdl.exe`, `taskse.exe` | |
| Config / payload names | `b.wnry`, `c.wnry`, `r.wnry`, `s.wnry`, `t.wnry`, `u.wnry` | |
| Mutex | `Global\MsWinZonesCacheCounterMutexA` | |

## Network indicators

| Type | Value | Notes |
| --- | --- | --- |
| Onion | `57g7spgrzlojinas.onion` | From `c.wnry` |
| Onion | `76jdd2ir2embyv47.onion` | From `c.wnry` |
| Onion | `cwwnhwhlz52maqm7.onion` | From `c.wnry` |
| Onion | `gx7ekbenv2riucmf.onion` | From `c.wnry` |
| Onion | `xxlvbrloxvriy2c5.onion` | From `c.wnry` |
| URL | `https://dist.torproject.org/torbrowser/6.5.1/tor-win32-0.2.9.10.zip` | Tor Browser fetch helper |
| URL | `http://www.btcfrog.com/qr/bitcoinPNG.php?address=%s` | Payment QR helper |
| BTC | `115p7UMMngoj1pMvkpHijcRdfJNXj6LrLn` | Ransom wallet |
| BTC | `12t9YDPgwueZ9NyMgw519p7AA8isjr6SMw` | Ransom wallet |
| BTC | `13AM4VW2dhxYgXeQepoHkHSQuy6NgaEb94` | Ransom wallet |

## Host and behavioral indicators

| Type | Value | MITRE |
| --- | --- | --- |
| Service install | `CreateServiceA` / `StartServiceA` | T1543.003 |
| File drop | `CopyFileA`, `CreateFileA`, `WriteFile` | T1105 / T1486 |
| Process launch | `CreateProcessA`, `cmd.exe /c "%s"` | T1059.003 |
| Attribute / path abuse | `SetFileAttributesW`, Windows directory staging | T1222 |
| Ransom UI download helper | `URLDownloadToFileA` in `u.wnry` | T1105 |
| Encrypted extension family | `.wnry` resource/payload naming | T1486 |

## Cryptographic indicators

| Type | Value | Notes |
| --- | --- | --- |
| AES | Forward S-box fingerprint (conf 95) | Dropper image |
| Inner ZIP password | `WNcry@2ol7` | Unlocks `embedded-1.zip` |

## Triage caveats from this Vanguard run

- Truncated Microsoft schema URL fragments and `osoft.com` appeared as low-value IOC noise. Ignore them.
- `taskdl.exe` scored 0 / benign in Vanguard. Do not treat that score as a clean bill of health for WannaCry helpers.
- Language-pack `m_*.wnry` members scored 35 and cluttered ranking. They are ransom note resources, not secondary payloads.
