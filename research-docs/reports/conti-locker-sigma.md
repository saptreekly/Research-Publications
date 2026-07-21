Detection content derived from the Conti (ContiLocker_v2) analysis. Because this artifact is a builder-tree debug build with templated config, the highest-fidelity static signatures target the **immutable crypto core and PDB path** (YARA), while behavioral coverage targets the family's **on-host tradecraft** — SMB spreading, shadow-copy deletion, and process termination (Sigma). Rules are **experimental**; tune before production.

## Rule index

| Rule | Type | MITRE | Level |
| --- | --- | --- | --- |
| Conti crypto core + PDB (YARA) | file | T1486 | high |
| Binary hash (SHA256) | Sysmon | T1486 | critical |
| VSS shadow copy deletion | process | T1490 | high |
| SMB network share enumeration burst | network/EDR | T1135 / T1021.002 | medium |
| Suspicious `ntdll` reload (unhooking) | image_load | T1562.001 | medium |
| Mass file rename (locker extension) | file_event | T1486 | medium |

## YARA — Conti crypto core and build path

Anchors on the ChaCha20 sigma constant, the Conti-specific config strings, and the leaked PDB path. The sigma constant alone is shared with all ChaCha20 code, so it is combined with Conti-specific markers to avoid false positives.

```yara
rule Conti_Locker_v2_Core
{
    meta:
        description = "Conti/ContiLocker_v2 crypto core, config placeholders, and debug PDB path"
        author      = "Jack Weekly"
        reference   = "Internal analysis report: Conti Locker (2026-07)"
        date        = "2026-07-21"
        tlp         = "CLEAR"
    strings:
        $sigma      = "expand 32-byte k"                       ascii
        $pdb        = "ContiLocker_v2" ascii nocase
        $note       = "__DECRYPT_NOTE__" ascii
        $mutex      = "__MUTEX_NAME__" ascii
        $keysz      = "EncryptedKeySize" ascii
        $imp1       = "CryptImportKey" ascii
        $imp2       = "NetShareEnum" ascii
    condition:
        uint16(0) == 0x5a4d and
        $sigma and
        (
            $pdb or
            (2 of ($note, $mutex, $keysz)) or
            (all of ($imp1, $imp2))
        )
}
```

## Sigma — binary hash execution

Tracks the specific analyzed binaries (useful for corpus/sandbox tracking). Requires hash logging (Sysmon Event 1 / EDR).

```yaml
title: Conti Locker v2 Binary Execution (Hash)
id: 6b1c9f2a-4e7d-4c3a-9b8e-1f0a2d5c7e94
status: experimental
description: Detects execution of the analyzed ContiLocker_v2 decryptor binaries by SHA256 hash.
references:
    - Internal analysis report Conti Locker (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.impact
    - attack.t1486
logsource:
    category: process_creation
    product: windows
detection:
    selection:
        Hashes|contains:
            - 'SHA256=9aed278f54f65e546fb9f7f34dff26d0835a0a21a5a4fc4c026bf84596ed277e'
            - 'SHA256=142cf75bc8dbfbd76f21f48b86ecbe11297e94071c9c55c1ee280d95c6ac6814'
    condition: selection
falsepositives:
    - Malware analysis sandboxes running the sample intentionally
level: critical
```

## Sigma — VSS shadow copy deletion

Conti's locker calls `DeleteShadowCopies()`, typically realized via `vssadmin`/WMI. This is the single highest-value behavioral rule for any ransomware.

```yaml
title: Conti Shadow Copy Deletion
id: a3f5c1e9-2b6d-4d7a-8c0e-5f9b1a3d7c62
status: experimental
description: Detects Volume Shadow Copy deletion consistent with Conti pre-encryption recovery inhibition.
references:
    - Internal analysis report Conti Locker (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.impact
    - attack.t1490
logsource:
    category: process_creation
    product: windows
detection:
    selection_vss:
        Image|endswith: '\vssadmin.exe'
        CommandLine|contains|all:
            - 'Delete'
            - 'Shadows'
    selection_wmic:
        Image|endswith: '\wmic.exe'
        CommandLine|contains: 'shadowcopy delete'
    condition: 1 of selection_*
falsepositives:
    - Legitimate backup maintenance
level: high
```

## Sigma — SMB network share enumeration burst

Approximates Conti's `network_scanner.cpp` behaviour: ARP-seeded host discovery followed by `NetShareEnum` and rapid SMB/445 fan-out. Best implemented as a threshold/correlation rule in the SIEM.

```yaml
title: Conti-style SMB Share Enumeration and Fan-out
id: c7e0a2f4-8d1b-4a6c-9e3f-2b5d7c9a1e08
status: experimental
description: Detects a single host performing rapid SMB/445 connections across many internal peers, consistent with Conti network encryption spreading.
references:
    - Internal analysis report Conti Locker (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.lateral_movement
    - attack.t1021.002
    - attack.discovery
    - attack.t1135
logsource:
    category: network_connection
    product: windows
detection:
    selection:
        Initiated: 'true'
        DestinationPort: 445
    timeframe: 1m
    condition: selection | count(DestinationIp) by SourceIp > 30
falsepositives:
    - Vulnerability scanners, backup agents, SCCM/asset inventory
level: medium
```

## Sigma — suspicious ntdll reload (EDR unhooking)

Conti's `removeHooks()` maps a fresh copy of `ntdll.dll` to strip userland EDR hooks. Detect an image load of `ntdll` from a non-System path, or a second mapping of `ntdll` into an already-running process.

```yaml
title: Conti Userland Unhooking via ntdll Reload
id: d9b2f4a6-1c7e-4e8a-b0d3-6a2f8c1e5b74
status: experimental
description: Detects loading of ntdll.dll from a non-standard path, consistent with EDR userland unhooking (removeHooks).
references:
    - Internal analysis report Conti Locker (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.defense_evasion
    - attack.t1562.001
logsource:
    category: image_load
    product: windows
detection:
    selection:
        ImageLoaded|endswith: '\ntdll.dll'
    filter_system:
        ImageLoaded|startswith:
            - 'C:\Windows\System32\'
            - 'C:\Windows\SysWOW64\'
    condition: selection and not filter_system
falsepositives:
    - Some sandboxing/instrumentation tools remap ntdll
level: medium
```

## Sigma — mass file rename with locker extension

The locker appends a ransom extension (templated as `.EXTEN` in this build) to each encrypted file. Detect high-rate rename/create by one process.

```yaml
title: Conti Mass File Rename (Encryption Extension)
id: e1c3a5f7-2d8b-4f9a-8c1e-7b3d9a2f6c50
status: experimental
description: Detects a single process renaming a large number of files with a uniform new extension, consistent with ransomware encryption.
references:
    - Internal analysis report Conti Locker (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.impact
    - attack.t1486
logsource:
    category: file_event
    product: windows
detection:
    selection:
        EventType: 'Rename'
    timeframe: 1m
    condition: selection | count() by Image > 200
falsepositives:
    - Bulk file conversion or archival tools
level: medium
```
