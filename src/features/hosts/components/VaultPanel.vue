<script setup lang="ts">
import { computed, ref } from "vue"
import { Lock, RotateCcw, Save, Unlock } from "lucide-vue-next"

import { UiBadge, UiButton } from "@/components/ui"
import VaultUnlockModal from "@/features/hosts/components/VaultUnlockModal.vue"

const props = defineProps<{
  available: boolean
  unlocked: boolean
  loading: boolean
  error: string | null
}>()

const emit = defineEmits<{
  unlock: [passphrase: string]
  save: []
  lock: []
  reset: []
}>()

const isUnlockOpen = ref(false)
const canSave = computed(() => props.unlocked && !props.loading)
const statusLabel = computed(() => {
  if (props.unlocked) return "Unlocked"
  if (props.available) return "Locked"

  return "Not created"
})

function unlockVault(passphrase: string) {
  emit("unlock", passphrase)

  isUnlockOpen.value = false
}

function saveVault() {
  if (!canSave.value) return

  emit("save")
}

function resetVault() {
  const shouldReset = window.confirm("Reset the local vault? This removes the encrypted local vault file and restores the mock hosts.")

  if (!shouldReset) return

  emit("reset")
}
</script>

<template>
  <div>
    <div class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-[var(--app-border)] bg-[var(--app-panel)] px-3 py-2">
      <div class="flex min-w-0 items-center gap-2">
        <UiBadge
            :icon="unlocked ? Unlock : Lock"
            :variant="unlocked ? 'primary' : 'secondary'"
            appearance="outline"
        >
          {{ statusLabel }}
        </UiBadge>

        <span class="truncate text-sm text-[var(--app-muted)]">
          Encrypted local file
        </span>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <UiButton
            :icon="unlocked ? Unlock : Lock"
            variant="secondary"
            appearance="outline"
            size="sm"
            :disabled="loading"
            @click="unlocked ? emit('lock') : isUnlockOpen = true"
        >
          {{ unlocked ? "Lock" : available ? "Unlock" : "Create" }}
        </UiButton>

        <UiButton
            :icon="Save"
            size="sm"
            :disabled="!canSave"
            @click="saveVault"
        >
          Save
        </UiButton>

        <UiButton
            v-if="available && !unlocked"
            :icon="RotateCcw"
            variant="secondary"
            appearance="text"
            size="sm"
            :disabled="loading"
            class-name="text-red-500 hover:text-red-500"
            @click="resetVault"
        >
          Reset
        </UiButton>
      </div>
    </div>

    <span
        v-if="error"
        class="mt-2 block text-xs text-red-500"
    >
      {{ error }}
    </span>

    <VaultUnlockModal
        :open="isUnlockOpen"
        :available="available"
        :loading="loading"
        @close="isUnlockOpen = false"
        @submit="unlockVault"
    />
  </div>
</template>
