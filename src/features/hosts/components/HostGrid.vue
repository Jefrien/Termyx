<script setup lang="ts">
import type { Host } from "@/features/hosts/types"
import HostCard from "@/features/hosts/components/HostCard.vue"

defineProps<{
  hosts: Host[]
}>()

defineEmits<{
  delete: [host: Host]
  edit: [host: Host]
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
        @delete="$emit('delete', $event)"
        @edit="$emit('edit', $event)"
        @open="$emit('open', $event)"
        @toggle-favorite="$emit('toggleFavorite', $event)"
    />
  </TransitionGroup>
</template>
