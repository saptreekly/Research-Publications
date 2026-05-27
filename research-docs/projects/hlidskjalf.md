Ultra-lightweight Type-1.5 thin hypervisor for legacy End-of-Life x86_64 hosts, starting with Windows 10. The goal is a hardware-enforced security layer that retrofits isolation and monitoring onto operating systems that no longer receive dependable vendor support.

## Problem

Windows 10 end-of-life forces migration to heavier, more restrictive platforms on hardware that still runs well. For security research and operational hardening, that creates a gap. Host kernels on unsupported systems cannot be trusted to police themselves. Hliðskjálf virtualizes a live running kernel on-the-fly and enforces policy from Ring -1 instead of asking the guest OS for permission.

## Architecture

The hypervisor compiles to a native Windows kernel driver. On load it enables Intel VT-x, builds VMCS state, identity-maps guest physical memory through EPT, and transitions the host into a virtualized execution context without a reboot.

| Layer | Role |
| --- | --- |
| Driver entry | CPU feature discovery, VMX enable, VMCS setup, VMLaunch |
| EPT subsystem | 1:1 identity mapping with page-level R/W/X enforcement |
| VM-Exit path | Assembly context save, Rust handler, VMRESUME loop |
| Policy hooks | Write-protect critical kernel dispatch structures, trap unauthorized changes |

Key design constraints:

- **Bare-metal core.** No std, no allocator dependency in the hot path.
- **Passive at rest.** Guest runs at native speed until a policy violation triggers VM-Exit.
- **Small footprint.** Target under 10MB with hardware pass-through when idle.
- **Anti-evasion.** Virtualizes selected timing instructions to frustrate sandbox and VM detection routines.

## Technical stack

- Rust for driver logic and VM-Exit handling
- Inline x86_64 assembly for VMXON, context save/restore, and VMRESUME
- Extended Page Tables for transparent guest memory views
- GitHub Actions for MSVC builds, CodeQL, and libFuzzer harnesses

## Module layout

```plaintext
src/
├── lib.rs           # DriverEntry and lifecycle
└── vmx/
    ├── init.rs      # CR4.VMXE and VMXON
    ├── config.rs    # Host/guest VMCS matrix
    ├── ept.rs       # Identity page tables
    ├── exit.rs      # Rust interception logic
    └── exit_asm.s   # VM-Exit wrapper
```

## Status

Active research. The repository includes build automation, fuzz targets, and low-level VMX bring-up code. See the GitHub repo for build instructions and the full technical README.
