# Research-Publications

This repository is the source code for Jack Weekly's portfolio, architected as a high-performance, brutalist web application.

## Tech Stack
- **Framework:** [Leptos](https://leptos.dev/) (Client-Side Rendering)
- **Language:** Rust (compiled to WebAssembly)
- **Build System:** [Trunk](https://trunkrs.dev/)
- **Infrastructure:** GitHub Actions (automated CI/CD)

## Live Deployment
View the live portfolio here: [https://saptreekly.github.io/Research-Publications/](https://saptreekly.github.io/Research-Publications/)

## Contact form (Web3Forms)

Production builds read `WEB3FORMS_ACCESS_KEY` from GitHub Actions secrets (Settings → Secrets and variables → Actions). The key is compiled into the WASM bundle at build time so it is not stored in the repository, but it is still visible to anyone who inspects the deployed site or network traffic. Web3Forms domain restrictions are the real protection.

Local development:

```bash
WEB3FORMS_ACCESS_KEY=your-key-here trunk serve
```
