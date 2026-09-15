<script setup lang="ts">
import { computed, ref, toRef } from "vue"

import { useTerminal } from "@/features/terminal/composables/useTerminal"
import type { TerminalSessionState } from "@/features/terminal/composables/useSshSession"

const props = defineProps<{
  isDark: boolean
  sessionState: TerminalSessionState
  hostName: string
  username: string
  hostname: string
}>()

const container = ref<HTMLDivElement | null>(null)
const prompt = computed(() => `${props.username}@${props.hostname}:~$ `)

useTerminal({
  container,
  isDark: toRef(props, "isDark"),
  sessionState: toRef(props, "sessionState"),
  hostLabel: toRef(props, "hostName"),
  prompt,
})
</script>

<template>
  <div class="h-full w-full overflow-hidden bg-[var(--app-terminal-bg)] p-3">
    <div
        ref="container"
        class="h-full w-full overflow-hidden"
    />
  </div>
</template>
