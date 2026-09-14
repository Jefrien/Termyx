<script setup lang="ts">
import {
  Plus,
  Server,
  Settings,
} from "lucide-vue-next"
import { RouterLink, useRoute } from "vue-router"

import { UiButton } from "@/components/ui"

const route = useRoute()
</script>

<template>
  <aside
      class="flex h-full w-56 shrink-0 flex-col border-r border-[var(--app-border)] bg-[var(--app-panel)]"
  >
    <nav class="min-h-0 flex-1 overflow-y-auto px-2 py-4">
      <div class="mb-2 px-2 text-xs font-medium uppercase tracking-wider text-[var(--app-muted)]">
        Hosts
      </div>

      <RouterLink
          to="/hosts"
          custom
          v-slot="{ navigate }"
      >
        <UiButton
            variant="primary"
            appearance="text"
            :icon="Server"
            :active="route.path.startsWith('/hosts')"
            class-name="w-full justify-start"
            @click="navigate"
        >
          Hosts
        </UiButton>
      </RouterLink>

      <UiButton
          variant="secondary"
          appearance="outline"
          :icon="Plus"
          class-name="mt-3 w-full justify-start border-dashed text-[var(--app-muted)]"
      >
        New host
      </UiButton>
    </nav>

    <div class="border-t border-[var(--app-border)] p-2">
      <UiButton
          variant="secondary"
          appearance="text"
          :icon="Settings"
          class-name="w-full justify-start"
      >
        Settings
      </UiButton>
    </div>
  </aside>
</template>
