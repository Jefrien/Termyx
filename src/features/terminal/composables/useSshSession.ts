import { computed, onBeforeUnmount, ref, type Ref } from "vue"

import {
  testSshConnection,
  type SshConnectionResult,
} from "@/features/terminal/services/sshConnection"

export type TerminalSessionState =
  | "disconnected"
  | "connecting"
  | "connected"
  | "reconnecting"
  | "failed"

interface UseSshSessionOptions {
  hostId: Ref<string>
  passphrase: Ref<string | null>
}

const RETRY_DELAY_MS = 2_000

export function useSshSession(options: UseSshSessionOptions) {
  const desiredConnected = ref(false)
  const sessionState = ref<TerminalSessionState>("disconnected")
  const lastResult = ref<SshConnectionResult | null>(null)
  const attempt = ref(0)
  let retryTimer: number | null = null

  const canConnect = computed(() => Boolean(options.passphrase.value))

  function clearRetry() {
    if (!retryTimer) return

    window.clearTimeout(retryTimer)
    retryTimer = null
  }

  function scheduleReconnect() {
    clearRetry()

    retryTimer = window.setTimeout(() => {
      retryTimer = null
      void connect()
    }, RETRY_DELAY_MS)
  }

  async function connect() {
    desiredConnected.value = true

    if (!options.passphrase.value) {
      sessionState.value = "failed"
      lastResult.value = {
        status: "vaultError",
        message: "Unlock the vault before connecting.",
      }
      return
    }

    sessionState.value = attempt.value > 0 ? "reconnecting" : "connecting"
    attempt.value += 1

    const result = await testSshConnection(options.hostId.value, options.passphrase.value)

    if (!desiredConnected.value) return

    lastResult.value = result

    if (result.status === "connected") {
      sessionState.value = "connected"
      return
    }

    if (result.status === "timeout" || result.status === "networkFailed") {
      sessionState.value = "reconnecting"
      scheduleReconnect()
      return
    }

    sessionState.value = "failed"
  }

  function disconnect() {
    desiredConnected.value = false
    clearRetry()
    sessionState.value = "disconnected"
    attempt.value = 0
  }

  onBeforeUnmount(() => {
    disconnect()
  })

  return {
    attempt,
    canConnect,
    connect,
    disconnect,
    lastResult,
    sessionState,
  }
}
