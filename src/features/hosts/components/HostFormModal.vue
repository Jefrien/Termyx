<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue"
import { FileKey, KeyRound, Server, ShieldOff } from "lucide-vue-next"
import { open as openDialog } from "@tauri-apps/plugin-dialog"
import { readTextFile } from "@tauri-apps/plugin-fs"

import { UiButton, UiModal } from "@/components/ui"
import type {
  CredentialAction,
  Host,
  HostAuthMethod,
  HostCreateInput,
  HostDraft,
  PrivateKeyStorageMode,
} from "@/features/hosts/types"

const props = withDefaults(
  defineProps<{
    open: boolean
    host?: Host | null
    mode?: "create" | "edit"
    vaultUnlocked: boolean
  }>(),
  {
    host: null,
    mode: "create",
  },
)

const emit = defineEmits<{
  close: []
  submit: [input: HostCreateInput]
}>()

const form = reactive<HostDraft>({
  name: "",
  hostname: "",
  username: "",
  port: 22,
  favorite: false,
})
const authMethod = ref<HostAuthMethod>("password")
const privateKeyStorageMode = ref<PrivateKeyStorageMode>("path")
const password = ref("")
const privateKeyPath = ref("")
const privateKeyContent = ref("")
const filePickerError = ref<string | null>(null)

const modalTitle = computed(() => props.mode === "edit" ? "Edit host" : "New host")
const submitLabel = computed(() => props.mode === "edit" ? "Save changes" : "Create")
const canPreserveCredential = computed(() =>
  props.mode === "edit"
  && props.host?.authMethod === authMethod.value
  && authMethod.value !== "none",
)
const credentialAction = computed<CredentialAction>(() => {
  if (authMethod.value === "none") return "clear"

  const hasNewCredential = password.value.length > 0
    || privateKeyPath.value.trim().length > 0
    || privateKeyContent.value.length > 0

  if (hasNewCredential) return "replace"

  return canPreserveCredential.value ? "preserve" : "clear"
})

const canSubmit = computed(() => {
  const hasBaseFields = form.name.trim().length > 0
    && form.hostname.trim().length > 0
    && form.username.trim().length > 0
    && Number.isInteger(form.port)
    && form.port > 0
    && form.port <= 65_535

  if (!hasBaseFields) return false
  if (authMethod.value === "none") return true
  if (credentialAction.value === "preserve") return true
  if (authMethod.value === "password") return password.value.length > 0
  if (privateKeyStorageMode.value === "path") return privateKeyPath.value.trim().length > 0

  return privateKeyContent.value.length > 0
})

function resetForm() {
  form.name = props.host?.name ?? ""
  form.hostname = props.host?.hostname ?? ""
  form.username = props.host?.username ?? ""
  form.port = props.host?.port ?? 22
  form.favorite = props.host?.favorite ?? false
  authMethod.value = props.host?.authMethod ?? "password"
  privateKeyStorageMode.value = props.host?.privateKeyStorageMode ?? "path"
  password.value = ""
  privateKeyPath.value = ""
  privateKeyContent.value = ""
  filePickerError.value = null
}

function isTauriRuntime() {
  return "__TAURI_INTERNALS__" in window
}

async function selectPrivateKeyFile() {
  filePickerError.value = null

  if (!isTauriRuntime()) {
    filePickerError.value = "File picker is available in the desktop app."
    return
  }

  try {
    const selectedPath = await openDialog({
      multiple: false,
      directory: false,
      title: "Select SSH private key",
    })

    if (!selectedPath || Array.isArray(selectedPath)) return

    privateKeyPath.value = selectedPath

    if (privateKeyStorageMode.value === "imported") {
      privateKeyContent.value = await readTextFile(selectedPath)
    }
  } catch (error) {
    filePickerError.value = error instanceof Error ? error.message : String(error)
  }
}

function submitForm() {
  if (!canSubmit.value) return

  emit("submit", {
    host: {
      name: form.name.trim(),
      hostname: form.hostname.trim(),
      username: form.username.trim(),
      port: form.port,
      favorite: form.favorite,
    },
    password: props.vaultUnlocked && authMethod.value === "password" && password.value.length > 0
      ? password.value
      : null,
    privateKeyPath: props.vaultUnlocked && authMethod.value === "privateKey" && privateKeyPath.value.trim().length > 0
      ? privateKeyPath.value.trim()
      : null,
    privateKeyContent: props.vaultUnlocked && authMethod.value === "privateKey" && privateKeyStorageMode.value === "imported" && privateKeyContent.value.length > 0
      ? privateKeyContent.value
      : null,
    privateKeyStorageMode: props.vaultUnlocked && authMethod.value === "privateKey"
      ? privateKeyStorageMode.value
      : null,
    authMethod: authMethod.value,
    credentialAction: credentialAction.value,
  })
  resetForm()
}

watch(privateKeyStorageMode, () => {
  privateKeyContent.value = ""
})

watch(
  () => [props.open, props.host, props.mode] as const,
  () => {
    resetForm()
  },
  {
    immediate: true,
  },
)
</script>

<template>
  <UiModal
      :open="open"
      :title="modalTitle"
      description="Create a local SSH profile and choose how Termyx should authenticate later."
      @close="emit('close')"
  >
    <form
        class="space-y-3"
        @submit.prevent="submitForm"
    >
      <label class="block">
        <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
          Name
        </span>

        <input
            v-model="form.name"
            type="text"
            autocomplete="off"
            placeholder="Production API"
            class="h-9 w-full rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)]"
        >
      </label>

      <label class="block">
        <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
          Hostname or IP
        </span>

        <input
            v-model="form.hostname"
            type="text"
            autocomplete="off"
            placeholder="192.168.1.20"
            class="h-9 w-full rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)]"
        >
      </label>

      <div class="grid grid-cols-[minmax(0,1fr)_6rem] gap-3">
        <label class="block min-w-0">
          <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
            Username
          </span>

          <input
              v-model="form.username"
              type="text"
              autocomplete="off"
              placeholder="ubuntu"
              class="h-9 w-full rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)]"
          >
        </label>

        <label class="block">
          <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
            Port
          </span>

          <input
              v-model.number="form.port"
              type="number"
              min="1"
              max="65535"
              class="h-9 w-full rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)]"
          >
        </label>
      </div>

      <div>
        <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
          Authentication
        </span>

        <div class="grid grid-cols-3 overflow-hidden rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] p-1">
          <button
              type="button"
              :class="[
                'flex h-8 items-center justify-center gap-2 rounded px-2 text-sm font-medium text-[var(--app-muted-strong)]',
                authMethod === 'none' ? 'bg-[var(--app-accent-soft)] text-[var(--app-text)]' : 'hover:bg-[var(--app-panel-soft)]',
              ]"
              @click="authMethod = 'none'"
          >
            <ShieldOff
                class="size-4"
                aria-hidden="true"
            />
            None
          </button>

          <button
              type="button"
              :class="[
                'flex h-8 items-center justify-center gap-2 rounded px-2 text-sm font-medium text-[var(--app-muted-strong)]',
                authMethod === 'password' ? 'bg-[var(--app-accent-soft)] text-[var(--app-text)]' : 'hover:bg-[var(--app-panel-soft)]',
              ]"
              @click="authMethod = 'password'"
          >
            <KeyRound
                class="size-4"
                aria-hidden="true"
            />
            Password
          </button>

          <button
              type="button"
              :class="[
                'flex h-8 items-center justify-center gap-2 rounded px-2 text-sm font-medium text-[var(--app-muted-strong)]',
                authMethod === 'privateKey' ? 'bg-[var(--app-accent-soft)] text-[var(--app-text)]' : 'hover:bg-[var(--app-panel-soft)]',
              ]"
              @click="authMethod = 'privateKey'"
          >
            <FileKey
                class="size-4"
                aria-hidden="true"
            />
            Private key
          </button>
        </div>
      </div>

      <label
          v-if="authMethod === 'password'"
          class="block"
      >
        <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
          Password
        </span>

        <input
            v-model="password"
            type="password"
            autocomplete="new-password"
            :disabled="!vaultUnlocked"
            :placeholder="mode === 'edit' ? 'Leave empty to keep current' : 'Required'"
            class="h-9 w-full rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)] disabled:opacity-60"
        >
      </label>

      <label
          v-else-if="authMethod === 'privateKey'"
          class="block"
      >
        <span class="mb-1 block text-xs font-medium text-[var(--app-muted)]">
          Private key
        </span>

        <div class="mb-2 grid grid-cols-2 overflow-hidden rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] p-1">
          <button
              type="button"
              :class="[
                'flex h-8 items-center justify-center px-2 text-sm font-medium text-[var(--app-muted-strong)]',
                privateKeyStorageMode === 'path' ? 'rounded bg-[var(--app-accent-soft)] text-[var(--app-text)]' : 'rounded hover:bg-[var(--app-panel-soft)]',
              ]"
              @click="privateKeyStorageMode = 'path'"
          >
            Use path
          </button>

          <button
              type="button"
              :class="[
                'flex h-8 items-center justify-center px-2 text-sm font-medium text-[var(--app-muted-strong)]',
                privateKeyStorageMode === 'imported' ? 'rounded bg-[var(--app-accent-soft)] text-[var(--app-text)]' : 'rounded hover:bg-[var(--app-panel-soft)]',
              ]"
              @click="privateKeyStorageMode = 'imported'"
          >
            Import
          </button>
        </div>

        <div class="flex gap-2">
          <input
              v-model="privateKeyPath"
              type="text"
              autocomplete="off"
              :disabled="!vaultUnlocked"
              :placeholder="mode === 'edit' ? 'Leave empty to keep current' : '~/.ssh/id_ed25519'"
              class="h-9 min-w-0 flex-1 rounded-md border border-[var(--app-border-strong)] bg-[var(--app-elevated)] px-3 text-sm text-[var(--app-text)] outline-none placeholder:text-[var(--app-muted)] focus:border-[var(--app-accent)] disabled:opacity-60"
          >

          <UiButton
              :icon="FileKey"
              variant="secondary"
              appearance="outline"
              size="sm"
              type="button"
              :disabled="!vaultUnlocked"
              @click="selectPrivateKeyFile"
          >
            Browse
          </UiButton>
        </div>

        <span class="mt-1 block text-xs text-[var(--app-muted)]">
          {{ mode === "edit" && privateKeyPath.length === 0 ? "Leave empty to keep the current key." : privateKeyStorageMode === "imported" ? "The selected key content will be copied into the encrypted vault." : "Termyx will store only the key path, like ssh -i." }}
        </span>

        <span
            v-if="filePickerError"
            class="mt-1 block text-xs text-red-500"
        >
          {{ filePickerError }}
        </span>
      </label>

      <label class="flex items-center gap-2 rounded-md border border-[var(--app-border)] bg-[var(--app-elevated)] px-3 py-2 text-sm text-[var(--app-muted-strong)]">
        <input
            v-model="form.favorite"
            type="checkbox"
            class="size-4 accent-[var(--app-accent)]"
        >

        Favorite host
      </label>

      <div class="flex justify-end gap-2 pt-1">
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
            :icon="Server"
            size="sm"
            type="submit"
            :disabled="!canSubmit"
        >
          {{ submitLabel }}
        </UiButton>
      </div>
    </form>
  </UiModal>
</template>
