<script setup lang="ts">
import { AlertTriangle } from "lucide-vue-next"

import { UiButton, UiModal } from "@/components/ui"

withDefaults(
  defineProps<{
    open: boolean
    title: string
    description: string
    confirmLabel?: string
    cancelLabel?: string
    loading?: boolean
  }>(),
  {
    confirmLabel: "Confirm",
    cancelLabel: "Cancel",
    loading: false,
  },
)

const emit = defineEmits<{
  cancel: []
  confirm: []
}>()
</script>

<template>
  <UiModal
      :open="open"
      :title="title"
      :description="description"
      @close="emit('cancel')"
  >
    <div class="flex items-start gap-3">
      <div class="grid size-9 shrink-0 place-items-center rounded-md bg-red-500/10 text-red-500">
        <AlertTriangle
            class="size-4"
            aria-hidden="true"
        />
      </div>

      <div class="min-w-0 flex-1">
        <p class="text-sm text-[var(--app-muted-strong)]">
          This action cannot be undone.
        </p>

        <div class="mt-4 flex justify-end gap-2">
          <UiButton
              variant="secondary"
              appearance="outline"
              size="sm"
              :disabled="loading"
              @click="emit('cancel')"
          >
            {{ cancelLabel }}
          </UiButton>

          <UiButton
              variant="secondary"
              size="sm"
              :disabled="loading"
              class-name="border-red-500 bg-red-500 text-white hover:bg-red-600"
              @click="emit('confirm')"
          >
            {{ confirmLabel }}
          </UiButton>
        </div>
      </div>
    </div>
  </UiModal>
</template>
