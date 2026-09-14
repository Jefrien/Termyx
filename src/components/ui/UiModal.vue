<script setup lang="ts">
import { X } from "lucide-vue-next"

import { UiButton } from "@/components/ui"
import { cn } from "@/lib/cn"

defineProps<{
  open: boolean
  title: string
  description?: string
  className?: string
}>()

const emit = defineEmits<{
  close: []
}>()
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
          v-if="open"
          class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
          @click.self="emit('close')"
      >
        <Transition
            name="modal-panel"
            appear
        >
          <section
              :class="cn('w-full max-w-sm overflow-hidden rounded-lg border border-[var(--app-border)] bg-[var(--app-panel)] text-[var(--app-text)] shadow-[var(--app-shadow)]', className)"
              role="dialog"
              aria-modal="true"
              :aria-label="title"
          >
            <header class="flex items-start justify-between gap-4 border-b border-[var(--app-border)] px-4 py-3">
              <div class="min-w-0">
                <h2 class="truncate text-sm font-semibold">
                  {{ title }}
                </h2>

                <p
                    v-if="description"
                    class="mt-1 text-sm leading-5 text-[var(--app-muted)]"
                >
                  {{ description }}
                </p>
              </div>

              <UiButton
                  :icon="X"
                  appearance="icon"
                  variant="secondary"
                  size="sm"
                  aria-label="Close"
                  @click="emit('close')"
              />
            </header>

            <div class="px-4 py-4">
              <slot />
            </div>
          </section>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
