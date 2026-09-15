import { invoke } from "@tauri-apps/api/core"

export type SshConnectionStatus =
  | "connected"
  | "authFailed"
  | "missingCredentials"
  | "networkFailed"
  | "timeout"
  | "vaultError"

export interface SshConnectionResult {
  status: SshConnectionStatus
  message: string
}

function isTauriRuntime() {
  return "__TAURI_INTERNALS__" in window
}

export async function testSshConnection(
  hostId: string,
  passphrase: string,
  timeoutMs = 6_000,
) {
  if (!isTauriRuntime()) {
    return {
      status: "connected",
      message: "Mock browser session connected.",
    } satisfies SshConnectionResult
  }

  return invoke<SshConnectionResult>("test_ssh_connection", {
    hostId,
    passphrase,
    timeoutMs,
  })
}
