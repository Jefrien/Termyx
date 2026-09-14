<script setup lang="ts">
import { computed } from "vue"
import { Moon, Sun } from "lucide-vue-next"

import { UiButton } from "@/components/ui"

const props = defineProps<{
  isDark: boolean
}>()

const emit = defineEmits<{
  toggleTheme: []
}>()

const themeLabel = computed(() => props.isDark ? "Switch to light mode" : "Switch to dark mode")
</script>

<template>
  <header class="flex h-14 shrink-0 items-center justify-between border-b border-[var(--app-border)] bg-[var(--app-panel)] px-4">
    <div class="min-w-0 text-lg font-semibold tracking-tight text-[var(--app-text)]">
      Termyx
    </div>

    <UiButton
        appearance="icon"
        variant="secondary"
        :icon="isDark ? Sun : Moon"
        :aria-label="themeLabel"
        :title="themeLabel"
        @click="emit('toggleTheme')"
    />
  </header>
</template>
