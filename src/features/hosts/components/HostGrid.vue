<script setup lang="ts">
import type { Host } from "@/features/hosts/types"
import HostCard from "@/features/hosts/components/HostCard.vue"

defineProps<{
  hosts: Host[]
}>()

defineEmits<{
  open: [host: Host]
}>()
</script>

<template>
  <div class="grid grid-cols-[repeat(auto-fill,minmax(220px,1fr))] gap-3">
    <HostCard
        v-for="host in hosts"
        :key="host.id"
        :host="host"
        @open="$emit('open', $event)"
    />
  </div>
</template>
