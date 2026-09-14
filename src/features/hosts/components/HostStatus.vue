<script setup lang="ts">
import { computed } from "vue"

import { cn } from "@/lib/cn"
import type { HostStatus } from "@/features/hosts/types"

const props = defineProps<{
  status: HostStatus
  className?: string
}>()

const statusClasses: Record<HostStatus, string> = {
  online: "bg-emerald-500",
  idle: "bg-[var(--app-warning)]",
  offline: "bg-[var(--app-muted)]",
}

const label: Record<HostStatus, string> = {
  online: "Online",
  idle: "Idle",
  offline: "Offline",
}

const classes = computed(() =>
  cn("inline-flex items-center gap-1.5 text-xs text-[var(--app-muted)]", props.className),
)
</script>

<template>
  <span :class="classes">
    <span
        :class="cn('size-1.5 rounded-full', statusClasses[status])"
        aria-hidden="true"
    />

    {{ label[status] }}
  </span>
</template>
