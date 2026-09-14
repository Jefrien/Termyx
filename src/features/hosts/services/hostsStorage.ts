import { invoke } from "@tauri-apps/api/core"

import type { Host, PrivateKeyStorageMode } from "@/features/hosts/types"

export interface AppVault {
  version: number
  hosts: Host[]
}

function isTauriRuntime() {
  return "__TAURI_INTERNALS__" in window
}

export async function vaultExists() {
  if (!isTauriRuntime()) {
    return false
  }

  return invoke<boolean>("vault_exists")
}

export async function loadHostsFromVault(passphrase: string) {
  if (!isTauriRuntime()) {
    return null
  }

  const vault = await invoke<AppVault | null>("load_app_vault", {
    passphrase,
  })

  return vault?.hosts ?? null
}

export async function resetVaultFile() {
  if (!isTauriRuntime()) {
    return
  }

  await invoke("reset_app_vault")
}

export async function saveHostsToVault(passphrase: string, hosts: Host[]) {
  if (!isTauriRuntime()) {
    return
  }

  await invoke("save_app_vault", {
    passphrase,
    vault: {
      version: 1,
      hosts,
    } satisfies AppVault,
  })
}

export async function saveHostToVault(
  passphrase: string,
  host: Host,
  password: string | null,
  privateKeyPath: string | null,
  privateKeyContent: string | null,
  privateKeyStorageMode: PrivateKeyStorageMode | null,
) {
  if (!isTauriRuntime()) {
    return
  }

  await invoke("upsert_vault_host", {
    passphrase,
    host,
    password,
    privateKeyPath,
    privateKeyContent,
    privateKeyStorageMode,
  })
}
