<script setup lang="ts">
import { MoreHorizontal, Server, Star } from "lucide-vue-next"

import { UiBadge, UiButton, UiCard } from "@/components/ui"
import type { Host } from "@/features/hosts/types"
import HostStatus from "@/features/hosts/components/HostStatus.vue"

defineProps<{
  host: Host
}>()

defineEmits<{
  open: [host: Host]
}>()
</script>

<template>
  <UiCard
      :icon="Server"
      :title="host.name"
      :subtitle="host.hostname"
      class-name="group cursor-pointer rounded-lg shadow-none transition hover:border-[var(--app-accent)] hover:bg-[var(--app-panel-soft)]"
      body-class-name="space-y-3"
      @click="$emit('open', host)"
  >
    <template #actions>
      <Star
          v-if="host.favorite"
          class="size-4 fill-[var(--app-warning)] text-[var(--app-warning)]"
          aria-label="Favorite host"
      />

      <UiButton
          appearance="icon"
          variant="secondary"
          size="sm"
          :icon="MoreHorizontal"
          aria-label="Host actions"
          class-name="opacity-70 hover:opacity-100"
          @click.stop
      />
    </template>

    <div class="flex items-center justify-between gap-3">
      <div class="min-w-0">
        <div class="truncate text-sm font-medium text-[var(--app-text)]">
          {{ host.username }}
        </div>

        <div class="mt-0.5 truncate text-xs text-[var(--app-muted)]">
          {{ host.username }}@{{ host.hostname }}
        </div>
      </div>

      <UiBadge
          v-if="host.port !== 22"
          variant="secondary"
          appearance="outline"
          size="sm"
      >
        :{{ host.port }}
      </UiBadge>
    </div>

    <div class="flex items-center justify-between gap-3">
      <HostStatus :status="host.status" />

      <span class="text-xs font-medium text-[var(--app-accent)]">
        Open
      </span>
    </div>
  </UiCard>
</template>
