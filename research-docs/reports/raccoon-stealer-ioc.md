IOC packaging for the Raccoon Stealer v2 Vanguard pass. Network destinations are withheld until the encoded config blob is decoded. Prefer the ImpHash for clustering hunts.

## File indicators

| Type | Value | Notes |
| --- | --- | --- |
| ImpHash | `4ec5227a81c3e90d891321c143c67557` | Shared by 20/21 members; primary pivot |
| Representative SHA256 | `0123b26df3c79bac0a3fda79072e36c159cfd1824ae3fd4b7f9dea9bda9c7909` | Deep-dive lead sample (57,344 B) |
| Cluster member SHA256 | `022432f770bf0e7c5260100fcde2ec7c49f68716751fd7d8b9e113bf06167e03` | Same ImpHash |
| Cluster member SHA256 | `960ce3cc26c8313b0fe41197e2aff5533f5f3efb1ba2970190779bc9a07bea63` | Same ImpHash |
| Outlier SHA256 | `9ee50e94a731872a74f47780317850ae2b9fae9d6c53a957ed7187173feb4f42` | Non-PE / raw; excluded from ImpHash cluster |
| Archive password | `infected` | Local corpus packaging only |

## Host / string indicators (static)

| Type | Value | Notes |
| --- | --- | --- |
| Browser DB string | `Login Data` | Chromium family credential store name |
| Wallet string | `wallet.dat` | Common crypto wallet filename |
| Registry string | `SOFTWARE\Microsoft\Cryptography` | Often used for machine GUID / crypto context |
| Registry string | `SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall` | Installed-software inventory |
| API names (strings) | `InternetOpenUrlA`, `InternetOpenUrlW`, `HttpSendRequestW` | WinInet exfil shape |
| Delay-load DLL names | `WinInet.dll`, `Crypt32.dll`, `Bcrypt.dll` | Resolved outside thin static IAT |

## Network indicators

Pending config decode. Do not treat the opaque Base64 blobs from Vanguard’s secrets pane as confirmed C2 URLs yet.

## Notes

- Full member SHA256 list is in the Vanguard stdout log (21 names = ZIP member filenames).
- This tab will expand once C2/path decode is finished.
