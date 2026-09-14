<script setup lang="ts">
defineProps<{
  label?: string
}>()
</script>

<template>
  <footer class="flex h-8 shrink-0 items-center gap-3 border-t border-[var(--app-border)] bg-[var(--app-panel)] px-3 text-xs text-[var(--app-muted)]">
    <span class="text-[var(--app-accent)]">
      ●
    </span>

    <span>{{ label ?? "Local" }}</span>

    <span>Mock SSH</span>
  </footer>
</template>
