Draft detection ideas from the Raccoon Stealer v2 static pass. These are **not** finished Sigma rules yet. They capture what I want to encode after config decode and a second sample check.

## Rule ideas

### 1. Chromium Login Data access by non-browser process

- **Idea:** Process that is not a signed browser touches `*\User Data\Default\Login Data` or equivalent.
- **Why here:** Vanguard surfaced the `Login Data` string across the ImpHash cluster.
- **Caveat:** Installers and backup tools false-positive. Needs parent/image allowlists.

### 2. WinInet HTTP client + dynamic API resolve in a small MSVC PE

- **Idea:** Image imports or resolves `LoadLibrary`/`GetProcAddress` and WinInet send/open APIs, size band ~50–80 KB, no rich network IAT at rest.
- **Why here:** Thin static IAT plus string-resolved `InternetOpenUrl*` / `HttpSendRequestW`.
- **Caveat:** Many droppers look similar. Pair with ImpHash or path strings.

### 3. ImpHash hunt (not Sigma)

- **ImpHash:** `4ec5227a81c3e90d891321c143c67557`
- Use VT / local ImpHash index rather than forcing this into Sigma.

## Status

Sigma YAML will land here once I am willing to ship a rule that survives a false-positive pass. Until then, prefer the IOC tab ImpHash and the analysis narrative.
