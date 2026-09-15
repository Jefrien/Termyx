import { invoke } from "@tauri-apps/api/core"
import { listen, type UnlistenFn } from "@tauri-apps/api/event"

export type SshConnectionStatus =
  | "connected"
  | "authFailed"
  | "disconnected"
  | "missingCredentials"
  | "networkFailed"
  | "timeout"
  | "vaultError"

export interface SshConnectionResult {
  status: SshConnectionStatus
  message: string
}

export interface SshSessionDataEvent {
  sessionId: string
  data: string
}

export interface SshSessionStatusEvent extends SshConnectionResult {
  sessionId: string
}

export interface StartSshSessionInput {
  hostId: string
  passphrase: string
  sessionId: string
  cols: number
  rows: number
  timeoutMs?: number
}

function isTauriRuntime() {
  return "__TAURI_INTERNALS__" in window
}

export async function startSshSession(input: StartSshSessionInput) {
  if (!isTauriRuntime()) {
    return {
      status: "connected",
      message: "Mock browser session connected.",
    } satisfies SshConnectionResult
  }

  return invoke<SshConnectionResult>("start_ssh_session", {
    hostId: input.hostId,
    passphrase: input.passphrase,
    sessionId: input.sessionId,
    cols: input.cols,
    rows: input.rows,
    timeoutMs: input.timeoutMs ?? 6_000,
  })
}

export async function writeSshSession(sessionId: string, data: string) {
  if (!isTauriRuntime()) return

  await invoke("write_ssh_session", {
    sessionId,
    data,
  })
}

export async function resizeSshSession(sessionId: string, cols: number, rows: number) {
  if (!isTauriRuntime()) return

  await invoke("resize_ssh_session", {
    sessionId,
    cols,
    rows,
  })
}

export async function stopSshSession(sessionId: string) {
  if (!isTauriRuntime()) return

  await invoke("stop_ssh_session", {
    sessionId,
  })
}

export async function onSshSessionData(
  handler: (event: SshSessionDataEvent) => void,
): Promise<UnlistenFn> {
  if (!isTauriRuntime()) return () => {}

  return listen<SshSessionDataEvent>("ssh-session-data", (event) => {
    handler(event.payload)
  })
}

export async function onSshSessionStatus(
  handler: (event: SshSessionStatusEvent) => void,
): Promise<UnlistenFn> {
  if (!isTauriRuntime()) return () => {}

  return listen<SshSessionStatusEvent>("ssh-session-status", (event) => {
    handler(event.payload)
  })
}
