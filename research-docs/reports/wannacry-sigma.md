Detection content from the WannaCry Vanguard stress-test. Highest-fidelity static anchors are the dropper hash, the embedded ZIP password string, and the Tor onion payment sites. Rules are experimental.

## Rule index

| Rule | Type | MITRE | Level |
| --- | --- | --- | --- |
| WannaCry dropper hash | process | T1486 | critical |
| Embedded ZIP password string | file/YARA | T1027 | high |
| Tor onion payment sites | network/dns | T1071.001 | high |
| MsWinZonesCacheCounter mutex | kernel/object | T1486 | medium |
| Service creation from suspicious parent | process | T1543.003 | high |
| Helper binary names | file/process | T1486 | medium |

## YARA: dropper password and payload names

```yara
rule WannaCry_Embedded_Pack_Markers
{
    meta:
        description = "WannaCry embedded ZIP password and payload member names"
        author      = "Jack Weekly"
        reference   = "Internal analysis report: WannaCry Vanguard stress test (2026-07)"
        date        = "2026-07-21"
    strings:
        $pass   = "WNcry@2ol7" ascii
        $mutex  = "Global\\MsWinZonesCacheCounterMutexA" ascii
        $u      = "u.wnry" ascii
        $c      = "c.wnry" ascii
        $task   = "tasksche.exe" ascii
    condition:
        uint16(0) == 0x5a4d and $pass and 2 of ($mutex, $u, $c, $task)
}
```

## Sigma: dropper hash

```yaml
title: WannaCry Dropper Binary Execution
id: 0f2c8a1e-6b4d-4e9a-9c3f-7a1d5e8b2c40
status: experimental
description: Detects execution of the classic WannaCry dropper by SHA256 hash.
references:
    - Internal analysis report WannaCry (July 2026)
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
        Hashes|contains: 'SHA256=ed01ebfbc9eb5bbea545af4d01bf5f1071661840480439c6e5babe8e080e41aa'
    condition: selection
falsepositives:
    - Malware analysis sandboxes running the sample intentionally
level: critical
```

## Sigma: onion payment sites

```yaml
title: WannaCry Tor Onion Payment Site Contact
id: 1a9e3c5f-8d2b-4f7a-b0c6-4e8f1a3d7c29
status: experimental
description: Detects DNS or proxy resolution attempts for WannaCry onion payment hosts observed in c.wnry.
references:
    - Internal analysis report WannaCry (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.command_and_control
    - attack.t1071.001
logsource:
    category: dns_query
    product: windows
detection:
    selection:
        QueryName|contains:
            - '57g7spgrzlojinas.onion'
            - '76jdd2ir2embyv47.onion'
            - 'cwwnhwhlz52maqm7.onion'
            - 'gx7ekbenv2riucmf.onion'
            - 'xxlvbrloxvriy2c5.onion'
    condition: selection
falsepositives:
    - Threat intel crawlers intentionally resolving historical WannaCry onions
level: high
```

## Sigma: service creation persistence

```yaml
title: WannaCry-style Service Creation Persistence
id: 2b7d4e6a-9c1f-4a8e-8d0b-5f3a7c9e1b64
status: experimental
description: Detects service creation patterns consistent with WannaCry installing itself as a Windows service.
references:
    - Internal analysis report WannaCry (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.persistence
    - attack.t1543.003
logsource:
    category: process_creation
    product: windows
detection:
    selection:
        Image|endswith:
            - '\sc.exe'
            - '\services.exe'
        CommandLine|contains|all:
            - 'create'
            - 'binPath'
    condition: selection
falsepositives:
    - Legitimate software installers creating services
level: high
```

## Sigma: helper binary names

```yaml
title: WannaCry Helper Binary Names
id: 3c8e5f7b-0d2a-4b9c-9e1f-6a4b8d0c2e75
status: experimental
description: Detects creation or execution of WannaCry helper names tasksche.exe, taskdl.exe, or taskse.exe.
references:
    - Internal analysis report WannaCry (July 2026)
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
        Image|endswith:
            - '\tasksche.exe'
            - '\taskdl.exe'
            - '\taskse.exe'
    condition: selection
falsepositives:
    - Unlikely outside malware labs replaying WannaCry
level: medium
```

## Sigma: mutex

```yaml
title: WannaCry MsWinZonesCacheCounter Mutex
id: 4d9f6a8c-1e3b-4c0d-a2f7-7b5c9e1d3f86
status: experimental
description: Detects creation of the Global\MsWinZonesCacheCounterMutexA mutex used by WannaCry.
references:
    - Internal analysis report WannaCry (July 2026)
author: Jack Weekly
date: 2026/07/21
tags:
    - attack.impact
    - attack.t1486
logsource:
    category: create_remote_thread
    product: windows
detection:
    selection:
        Mutex|contains: 'MsWinZonesCacheCounterMutexA'
    condition: selection
falsepositives:
    - Rare outside intentional WannaCry detonation
level: medium
```
