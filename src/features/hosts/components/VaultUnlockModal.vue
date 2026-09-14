<script setup lang="ts">
import { computed, ref, watch } from "vue"
import { Lock, Unlock } from "lucide-vue-next"

import { UiButton, UiModal } from "@/components/ui"

const props = defineProps<{
  open: boolean
  available: boolean
  loading: boolean
}>()

const emit = defineEmits<{
  close: []
  submit: [passphrase: string]
}>()

const passphrase = ref("")
const canSubmit = computed(() => passphrase.value.trim().length >= 8 && !props.loading)

function submitForm() {
  if (!canSubmit.value) return

  emit("submit", passphrase.value)
  passphrase.value = ""
}

watch(
  () => props.open,
  (open) => {
    if (!open) {
      passphrase.value = ""
    }
  },
)
</script>

<template>
  <UiModal
      :open="open"
      :title="available ? 'Unlock vault' : 'Create vault'"
      description="Termyx needs the local vault unlocked before saving host credentials."
      @close="emit('close')"
  >
    <form
        class="space-y-3"
        @submit.prevent="submitForm"
    >
      <input
          v-model="passphrase"
          type="password"
          autocomplete="current-password"
          placeholder="At least 8 characters"
          :disabled="loading"
          class="h-9 w-full rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)]"
      >

      <div class="flex justify-end gap-2">
        <UiButton
            variant="secondary"
            appearance="outline"
            size="sm"
            type="button"
            @click="emit('close')"
        >
          Cancel
        </UiButton>

        <UiButton
            :icon="available ? Unlock : Lock"
            size="sm"
            type="submit"
            :disabled="!canSubmit"
        >
          {{ available ? "Unlock" : "Create" }}
        </UiButton>
      </div>
    </form>
  </UiModal>
</template>
