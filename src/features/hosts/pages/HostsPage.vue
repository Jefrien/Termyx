<script setup lang="ts">
import { onMounted, ref } from "vue"
import { useRouter } from "vue-router"
import { Plus } from "lucide-vue-next"

import { UiButton } from "@/components/ui"
import EmptyHostsState from "@/features/hosts/components/EmptyHostsState.vue"
import HostFormModal from "@/features/hosts/components/HostFormModal.vue"
import HostGrid from "@/features/hosts/components/HostGrid.vue"
import VaultPanel from "@/features/hosts/components/VaultPanel.vue"
import VaultUnlockModal from "@/features/hosts/components/VaultUnlockModal.vue"
import { useHostsStore } from "@/features/hosts/stores/hosts"
import type { Host, HostCreateInput } from "@/features/hosts/types"

const router = useRouter()
const hostsStore = useHostsStore()
const isNewHostOpen = ref(false)
const isVaultPromptOpen = ref(false)
const editingHost = ref<Host | null>(null)

function openHost(host: Host) {
  void router.push({
    name: "host-terminal",
    params: {
      hostId: host.id,
    },
  })
}

async function createHost(input: HostCreateInput) {
  await hostsStore.addHost(input)
  isNewHostOpen.value = false
}

async function saveEditedHost(input: HostCreateInput) {
  if (!editingHost.value) return

  await hostsStore.updateHost({
    ...input,
    id: editingHost.value.id,
  })
  editingHost.value = null
}

function editHost(host: Host) {
  editingHost.value = host
}

async function deleteHost(host: Host) {
  const shouldDelete = window.confirm(`Delete ${host.name}? This also removes its saved credentials from the vault.`)

  if (!shouldDelete) return

  await hostsStore.deleteHost(host.id)
}

function startCreateHost() {
  if (hostsStore.vaultUnlocked) {
    isNewHostOpen.value = true
    return
  }

  isVaultPromptOpen.value = true
}

async function unlockForCreate(passphrase: string) {
  await hostsStore.unlockVault(passphrase)

  if (!hostsStore.vaultUnlocked) return

  isVaultPromptOpen.value = false
  isNewHostOpen.value = true
}

onMounted(() => {
  void hostsStore.checkVault()
})
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col">
    <header class="flex shrink-0 items-center justify-between gap-4 border-b border-[var(--app-border)] px-5 py-4">
      <div class="min-w-0">
        <h1 class="truncate text-xl font-semibold tracking-tight text-[var(--app-text)]">
          Hosts
        </h1>

        <p class="mt-1 text-sm text-[var(--app-muted)]">
          Local SSH profiles for mock terminal sessions.
        </p>
      </div>

      <UiButton
          :icon="Plus"
          size="sm"
          @click="startCreateHost"
      >
        New Host
      </UiButton>
    </header>

    <div class="min-h-0 flex-1 overflow-auto p-5">
      <VaultPanel
          class="mb-4"
          :available="hostsStore.vaultAvailable"
          :unlocked="hostsStore.vaultUnlocked"
          :loading="hostsStore.isLoadingVault"
          :error="hostsStore.vaultError"
          @unlock="hostsStore.unlockVault"
          @save="hostsStore.saveVault"
          @lock="hostsStore.lockVault"
          @reset="hostsStore.resetVault"
      />

      <HostGrid
          v-if="hostsStore.hosts.length"
          :hosts="hostsStore.hosts"
          @delete="deleteHost"
          @edit="editHost"
          @open="openHost"
          @toggle-favorite="hostsStore.toggleFavorite"
      />

      <EmptyHostsState
          v-else
          @create="startCreateHost"
      />
    </div>

    <VaultUnlockModal
        :open="isVaultPromptOpen"
        :available="hostsStore.vaultAvailable"
        :loading="hostsStore.isLoadingVault"
        @close="isVaultPromptOpen = false"
        @submit="unlockForCreate"
    />

    <HostFormModal
        :open="isNewHostOpen"
        mode="create"
        :vault-unlocked="hostsStore.vaultUnlocked"
        @close="isNewHostOpen = false"
        @submit="createHost"
    />

    <HostFormModal
        :open="editingHost !== null"
        mode="edit"
        :host="editingHost"
        :vault-unlocked="hostsStore.vaultUnlocked"
        @close="editingHost = null"
        @submit="saveEditedHost"
    />
  </section>
</template>
