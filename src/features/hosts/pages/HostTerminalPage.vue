<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue"
import { RouterLink } from "vue-router"
import { ArrowLeft, Play, Square } from "lucide-vue-next"

import { UiBadge, UiButton } from "@/components/ui"
import HostStatus from "@/features/hosts/components/HostStatus.vue"
import { useHostsStore } from "@/features/hosts/stores/hosts"
import TerminalView from "@/features/terminal/components/TerminalView.vue"

const props = defineProps<{
  hostId: string
  isDark: boolean
}>()

const hostsStore = useHostsStore()
const host = computed(() => hostsStore.getHostById(props.hostId))
const sessionState = ref<"disconnected" | "connecting" | "connected">("disconnected")
let connectTimer: number | null = null

const sessionBadgeVariant = computed(() =>
  sessionState.value === "connected" ? "primary" : "secondary",
)
const sessionLabel = computed(() => {
  if (sessionState.value === "connected") return "Connected"
  if (sessionState.value === "connecting") return "Connecting"

  return "Disconnected"
})

function connectSession() {
  if (sessionState.value !== "disconnected") return

  sessionState.value = "connecting"
  connectTimer = window.setTimeout(() => {
    sessionState.value = "connected"
    connectTimer = null
  }, 450)
}

function disconnectSession() {
  if (connectTimer) {
    window.clearTimeout(connectTimer)
    connectTimer = null
  }

  sessionState.value = "disconnected"
}

onBeforeUnmount(() => {
  if (connectTimer) {
    window.clearTimeout(connectTimer)
  }
})
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col">
    <template v-if="host">
      <header class="flex shrink-0 items-center justify-between gap-4 border-b border-[var(--app-border)] px-5 py-4">
        <div class="flex min-w-0 items-center gap-3">
          <RouterLink
              to="/hosts"
              custom
              v-slot="{ navigate }"
          >
            <UiButton
                :icon="ArrowLeft"
                appearance="icon"
                variant="secondary"
                size="sm"
                aria-label="Back to hosts"
                @click="navigate"
            />
          </RouterLink>

          <div class="min-w-0">
            <h1 class="truncate text-xl font-semibold tracking-tight text-[var(--app-text)]">
              {{ host.name }}
            </h1>

            <div class="mt-1 flex items-center gap-2 text-sm text-[var(--app-muted)]">
              <span class="truncate">
                {{ host.username }}@{{ host.hostname }}
              </span>

              <UiBadge
                  v-if="host.port !== 22"
                  variant="secondary"
                  appearance="outline"
                  size="sm"
              >
                :{{ host.port }}
              </UiBadge>
            </div>
          </div>
        </div>

        <div class="flex shrink-0 items-center gap-2">
          <HostStatus :status="host.status" />

          <UiButton
              v-if="sessionState === 'disconnected'"
              :icon="Play"
              size="sm"
              @click="connectSession"
          >
            Connect
          </UiButton>

          <UiButton
              v-else
              :icon="Square"
              variant="secondary"
              appearance="outline"
              size="sm"
              @click="disconnectSession"
          >
            Disconnect
          </UiButton>
        </div>
      </header>

      <div class="min-h-0 flex-1 overflow-hidden p-3">
        <div class="flex h-full min-h-0 flex-col overflow-hidden rounded-lg border border-[var(--app-border-strong)] bg-[var(--app-terminal-bg)] shadow-[var(--app-shadow)]">
          <div class="flex h-10 shrink-0 items-center justify-between border-b border-[var(--app-border)] px-3">
            <div class="min-w-0 truncate text-sm font-medium text-[var(--app-text)]">
              {{ host.id }}
            </div>

            <UiBadge
                :variant="sessionBadgeVariant"
                size="sm"
            >
              {{ sessionLabel }}
            </UiBadge>
          </div>

          <div class="min-h-0 flex-1">
            <TerminalView
                :is-dark="isDark"
                :session-state="sessionState"
                :host-name="host.name"
                :username="host.username"
                :hostname="host.hostname"
            />
          </div>

          <footer class="flex h-8 shrink-0 items-center gap-3 border-t border-[var(--app-border)] px-3 text-xs text-[var(--app-muted)]">
            <span class="text-[var(--app-accent)]">
              ●
            </span>

            <span>Local/mock connection state</span>

            <span>{{ sessionLabel }}</span>
          </footer>
        </div>
      </div>
    </template>

    <div
        v-else
        class="flex min-h-0 flex-1 items-center justify-center p-5"
    >
      <div class="max-w-sm text-center">
        <h1 class="text-lg font-semibold text-[var(--app-text)]">
          Host not found
        </h1>

        <p class="mt-2 text-sm text-[var(--app-muted)]">
          The selected host profile does not exist.
        </p>

        <RouterLink
            to="/hosts"
            class="mt-4 inline-flex text-sm font-medium text-[var(--app-accent)] hover:underline"
        >
          Back to Hosts
        </RouterLink>
      </div>
    </div>
  </section>
</template>
