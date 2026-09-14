<script setup lang="ts">
import type { Host } from "@/features/hosts/types"
import HostCard from "@/features/hosts/components/HostCard.vue"

defineProps<{
  hosts: Host[]
}>()

defineEmits<{
  open: [host: Host]
  toggleFavorite: [hostId: string]
}>()
</script>

<template>
  <TransitionGroup
      name="host-list"
      tag="div"
      class="grid grid-cols-[repeat(auto-fill,minmax(220px,1fr))] gap-3"
  >
    <HostCard
        v-for="host in hosts"
        :key="host.id"
        :host="host"
        @open="$emit('open', $event)"
        @toggle-favorite="$emit('toggleFavorite', $event)"
    />
  </TransitionGroup>
</template>
