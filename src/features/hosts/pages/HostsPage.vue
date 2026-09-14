<script setup lang="ts">
import { useRouter } from "vue-router"
import { Plus } from "lucide-vue-next"

import { UiButton } from "@/components/ui"
import EmptyHostsState from "@/features/hosts/components/EmptyHostsState.vue"
import HostGrid from "@/features/hosts/components/HostGrid.vue"
import { useHostsStore } from "@/features/hosts/stores/hosts"
import type { Host } from "@/features/hosts/types"

const router = useRouter()
const hostsStore = useHostsStore()

function openHost(host: Host) {
  void router.push({
    name: "host-terminal",
    params: {
      hostId: host.id,
    },
  })
}
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col">
    <header class="flex shrink-0 items-center justify-between gap-4 border-b border-[var(--app-border)] px-5 py-4">
      <div class="min-w-0">
        <h1 class="truncate text-xl font-semibold tracking-tight text-[var(--app-text)]">
          Hosts
        </h1>

        <p class="mt-1 text-sm text-[var(--app-muted)]">
          Local SSH profiles for mock terminal sessions.
        </p>
      </div>

      <UiButton
          :icon="Plus"
          size="sm"
      >
        New Host
      </UiButton>
    </header>

    <div class="min-h-0 flex-1 overflow-auto p-5">
      <HostGrid
          v-if="hostsStore.hosts.length"
          :hosts="hostsStore.hosts"
          @open="openHost"
      />

      <EmptyHostsState v-else />
    </div>
  </section>
</template>
