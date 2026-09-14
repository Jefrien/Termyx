<script setup lang="ts">
import { Server } from "lucide-vue-next"

import { UiButton, UiCard } from "@/components/ui"
</script>

<template>
  <UiCard
      title="No hosts yet"
      subtitle="Add your first SSH profile to start a mock session."
      :icon="Server"
      class-name="max-w-md shadow-none"
  >
    <UiButton size="sm">
      New Host
    </UiButton>
  </UiCard>
</template>
