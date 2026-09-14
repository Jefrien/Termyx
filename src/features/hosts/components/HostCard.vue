<script setup lang="ts">
import { computed } from "vue"
import { Edit3, FileKey, KeyRound, Server, Star, Trash2 } from "lucide-vue-next"

import { UiBadge, UiButton, UiCard } from "@/components/ui"
import type { Host } from "@/features/hosts/types"
import HostStatus from "@/features/hosts/components/HostStatus.vue"

const props = defineProps<{
  host: Host
}>()

defineEmits<{
  delete: [host: Host]
  edit: [host: Host]
  open: [host: Host]
  toggleFavorite: [hostId: string]
}>()

const authLabel = computed(() => {
  if (props.host.authMethod === "privateKey") {
    return props.host.privateKeyStorageMode === "imported" ? "Imported key" : "Key file"
  }

  if (props.host.authMethod === "password") return "Password"

  return "No auth"
})

const authIcon = computed(() => props.host.authMethod === "privateKey" ? FileKey : KeyRound)
</script>

<template>
  <UiCard
      class-name="group cursor-pointer rounded-lg shadow-none transition hover:-translate-y-px hover:border-[var(--app-accent)] hover:bg-[var(--app-panel-soft)] hover:shadow-[var(--app-shadow)]"
      body-class-name="space-y-3"
      @click="$emit('open', host)"
  >
    <template #header>
      <div class="flex min-w-0 items-start gap-3">
        <div class="grid size-9 shrink-0 place-items-center rounded-md bg-[var(--app-panel-soft)] text-[var(--app-accent)]">
          <Server
              class="size-4"
              aria-hidden="true"
          />
        </div>

        <div class="min-w-0">
          <h2 class="truncate text-sm font-semibold text-current">
            {{ host.name }}
          </h2>

          <p class="mt-0.5 truncate text-xs text-[var(--app-muted)]">
            {{ host.hostname }}
          </p>
        </div>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <UiButton
            appearance="icon"
            variant="secondary"
            size="sm"
            :icon="Edit3"
            aria-label="Edit host"
            class-name="opacity-70 hover:opacity-100"
            @click.stop="$emit('edit', host)"
        />

        <UiButton
          appearance="icon"
          variant="secondary"
          size="sm"
          :icon="Star"
          :icon-class-name="host.favorite ? 'fill-[var(--app-warning)] text-[var(--app-warning)]' : undefined"
          :aria-label="host.favorite ? 'Remove favorite' : 'Mark favorite'"
          :class-name="host.favorite ? 'opacity-100' : 'opacity-70 hover:opacity-100'"
          @click.stop="$emit('toggleFavorite', host.id)"
        />

        <UiButton
            appearance="icon"
            variant="secondary"
            size="sm"
            :icon="Trash2"
            aria-label="Delete host"
            class-name="opacity-70 hover:text-red-500 hover:opacity-100"
            @click.stop="$emit('delete', host)"
        />
      </div>
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

      <div class="flex items-center gap-2">
        <UiBadge
            :icon="authIcon"
            variant="secondary"
            appearance="text"
            size="sm"
        >
          {{ authLabel }}
        </UiBadge>

        <span class="text-xs font-medium text-[var(--app-accent)]">
          Open
        </span>
      </div>
    </div>
  </UiCard>
</template>
