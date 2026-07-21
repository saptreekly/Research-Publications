Actionable indicators extracted from the Conti (ContiLocker_v2) analysis. Note that the two analyzed binaries are **debug builds from the leaked source/builder tree** — their config fields are unfilled templates, so host/network indicators reflect the family's *design*, not a live campaign. Use the hashes for corpus/sample tracking and the behavioral indicators for hunting real Conti intrusions.

## File indicators

| Type | Value | Notes |
| --- | --- | --- |
| Archive SHA256 | `e403aa3f4273cd11fa13597586cbf353ecef4040344f6ee9fff23168156fb29a` | `conti_locker.zip` |
| Archive MD5 | `33724d7ca2a6888d7880e0fa49c43e9b` | password-protected pack |
| Binary SHA256 (A) | `9aed278f54f65e546fb9f7f34dff26d0835a0a21a5a4fc4c026bf84596ed277e` | `decryptor.exe`, 110,592 B |
| Binary MD5 (A) | `31ee2755a455b1c8c743f51cec3845fd` | statically-linked CRT |
| ImpHash (A) | `ac80ef907c0b25dff5f5ca08ca8f21ef` | VT pivot: `imphash:ac80ef907c0b25dff5f5ca08ca8f21ef` |
| Binary SHA256 (B) | `142cf75bc8dbfbd76f21f48b86ecbe11297e94071c9c55c1ee280d95c6ac6814` | `decryptor.exe`, 158,720 B |
| Binary MD5 (B) | `04dce97942dfb520fb4c12527c82164b` | debug CRT (`ucrtbased`) |
| ImpHash (B) | `8d4376cd52bea4b09fa664f1bb329e91` | VT pivot: `imphash:8d4376cd52bea4b09fa664f1bb329e91` |
| PDB path | `c:\source\ContiLocker_v2\Debug\decryptor.pdb` | embedded debug path |
| Type | PE32 x86 Windows console, MSVC, unsigned | no packer |

## Config placeholders (builder templates — NOT live IOCs)

These strings appear verbatim in the binary because the builder had not patched them. Seeing the *placeholders* is itself a strong indicator that a sample was built from this leaked kit.

| Field | Placeholder value | Source |
| --- | --- | --- |
| Ransom note | `__DECRYPT_NOTE__` | `global_parameters.cpp` (`g_DecryptNote[2048]`) |
| Mutex name | `__MUTEX_NAME__` | `global_parameters.cpp` (`g_MutexName[65]`) |
| Encrypted extension | `.EXTEN` (`L".EXTEN"`) | `global_parameters.cpp` (`g_Extention[7]`) |

## Network indicators (design, from source)

| Type | Value | Notes |
| --- | --- | --- |
| Spreading protocol | SMB, `445/tcp` | `network_scanner.cpp` `#define SMB_PORT 445` |
| Host discovery | ARP cache (`GetIpNetTable`) | seeds intranet host list |
| Share discovery | `NetShareEnum` (NETAPI32) | enumerates admin/user shares |
| C2 / beacon | **none** | human-operated; no locker beacon by design |

## Host and behavioral indicators

| Type | Value | MITRE |
| --- | --- | --- |
| File encryption | ChaCha20 (256-bit) body + RSA key wrap | T1486 |
| Encrypted-key blob | 524-byte `EncryptedKey` appended per file | T1486 |
| Shadow copy deletion | `DeleteShadowCopies()` (locker module) | T1490 |
| Process termination | `process_killer::KillAll()` w/ whitelist | T1489 |
| Network share discovery | `NetShareEnum` over SMB/445 | T1135 / T1021.002 |
| Local + ARP discovery | drive enum + `GetIpNetTable` | T1083 / T1016 |
| Dynamic API resolution | hashed-API loader (`GetApi.h`) | T1106 / T1027 |
| Userland unhooking | `removeHooks()` — reload clean `ntdll` | T1562.001 |
| Compile-time string obf. | `MetaString.h` XOR string encoding | T1027 |
| Anti-debug | `IsDebuggerPresent` | T1622 |
| Multithreaded encryption | IOCP (`CreateIoCompletionPort`) | — |

## Cryptographic indicators

| Type | Value | Notes |
| --- | --- | --- |
| Stream cipher constant | `expand 32-byte k` (ChaCha20 256-bit sigma) | high-fidelity YARA anchor |
| Cipher source | DJB `chacha-merged.c` (public domain) | `chacha20/chacha.c` |
| Key transport | Windows CryptoAPI `CryptImportKey` + `CryptDecrypt` | RSA envelope |
| Per-file key material | 32-byte ChaCha key + 8-byte IV | fresh per file |
| Config string | `EncryptedKeySize` | present in binary |

## Triage caveats

- **False positive:** YARA `Suspicious_Delphi_CODE_Section` fires, but these are MSVC C++ binaries — ignore the Delphi attribution.
- **Label caveat:** the automated "injection / hollow patterns" verdict is not supported by the imports — no `VirtualAllocEx`/`WriteProcessMemory`/`CreateRemoteThread`. Treat as generic high-severity, not injection.
