# Termyx Agent Notes

Termyx is a desktop-first, open source remote connection manager built with Tauri 2, Vue 3, TypeScript, Vite, Tailwind CSS 4, Pinia, xterm.js, lucide-vue-next, and pnpm.

## Product Direction

- Keep Termyx local-first.
- Do not require a Termyx-owned backend.
- Treat sync providers such as WebDAV or S3 as transport for encrypted blobs only.
- Do not implement real SSH until the Vue to Tauri to Rust boundary is ready.
- Do not store passwords, private keys, passphrases, or tokens in Vue state longer than needed.

## Architecture

- Vue owns UI state and routing.
- Pinia stores frontend state.
- Tauri IPC is the boundary for persistence, secrets, filesystem access, and future SSH.
- Rust owns the local vault and will own SSH later.
- The current vault model is a single encrypted local file.

Expected direction:

```text
Vue
 ↓
Tauri IPC
 ↓
Rust core
 ↓
Encrypted local vault
 ↓
Optional sync transport
```

## Storage

- Keep host mocks centralized.
- Keep encrypted persistence behind service interfaces.
- Current encrypted vault file:
  - Versioned JSON wrapper.
  - Argon2id key derivation.
  - AES-256-GCM encryption.
  - Stored in the app data directory as `termyx-vault.json.enc`.
- Do not add backend assumptions to storage APIs.
- Do not add secrets to host records in the frontend model.

## Frontend

- Use Vue 3 Composition API with `<script setup lang="ts">`.
- Use strict TypeScript.
- Do not use `any`.
- Use `@/` imports.
- Use Tailwind CSS 4 with `@tailwindcss/vite` and `@import "tailwindcss";`.
- Use lucide-vue-next for icons.
- Do not use emojis in UI, comments, or icon placeholders.

## Design

- Continue the existing compact desktop developer-tool visual style.
- Reuse existing theme tokens from `src/assets/main.css`.
- Avoid gradients, oversized shadows, generic SaaS dashboard layouts, and overly rounded cards.
- Keep the main navigation focused on Hosts until future modules are intentionally added.
- The home page should show Hosts, not a terminal.
- Terminal pages are opened from `/hosts/:hostId`.

## Verification

Before handing off meaningful changes, run:

```bash
pnpm exec vue-tsc --noEmit
pnpm build
```

When Rust or Tauri code changes, also run:

```bash
cd src-tauri
cargo check
```
