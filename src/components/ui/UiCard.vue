<script setup lang="ts">
import { computed } from "vue"
import type { Component } from "vue"

import { cn } from "@/lib/cn"

type CardVariant = "primary" | "secondary" | "white"
type CardAppearance = "filled" | "outline"

const props = withDefaults(
  defineProps<{
    title?: string
    subtitle?: string
    variant?: CardVariant
    appearance?: CardAppearance
    icon?: Component
    className?: string
    headerClassName?: string
    bodyClassName?: string
  }>(),
  {
    variant: "secondary",
    appearance: "filled",
  },
)

const baseClasses =
    "overflow-hidden rounded-lg text-[var(--app-text)]"

const appearanceClasses: Record<CardVariant, Record<CardAppearance, string>> = {
  primary: {
    filled: "border border-[var(--app-accent)] bg-[var(--app-accent-soft)] shadow-[var(--app-shadow)]",
    outline: "border border-[var(--app-accent)] bg-transparent",
  },
  secondary: {
    filled: "border border-[var(--app-border)] bg-[var(--app-elevated)] shadow-[var(--app-shadow)]",
    outline: "border border-[var(--app-border-strong)] bg-transparent",
  },
  white: {
    filled: "border border-white bg-white text-[#211f1d] shadow-[var(--app-shadow)]",
    outline: "border border-white/70 bg-transparent text-white",
  },
}

const classes = computed(() => cn(
    baseClasses,
    appearanceClasses[props.variant][props.appearance],
    props.className,
))

const headerClasses = computed(() => cn(
    "flex items-start justify-between gap-3 border-b border-[var(--app-border)] px-4 py-3",
    props.headerClassName,
))

const bodyClasses = computed(() => cn("px-4 py-3", props.bodyClassName))
</script>

<template>
  <section :class="classes">
    <header
        v-if="title || subtitle || icon || $slots.actions || $slots.header"
        :class="headerClasses"
    >
      <slot name="header">
        <div class="flex min-w-0 items-start gap-3">
          <div
              v-if="icon"
              class="grid size-9 shrink-0 place-items-center rounded-md bg-[var(--app-panel-soft)] text-[var(--app-accent)]"
          >
            <component
                :is="icon"
                class="size-4"
                aria-hidden="true"
            />
          </div>

          <div class="min-w-0">
            <h2
                v-if="title"
                class="truncate text-sm font-semibold text-current"
            >
              {{ title }}
            </h2>

            <p
                v-if="subtitle"
                class="mt-0.5 truncate text-xs text-[var(--app-muted)]"
            >
              {{ subtitle }}
            </p>
          </div>
        </div>

        <div
            v-if="$slots.actions"
            class="flex shrink-0 items-center gap-2"
        >
          <slot name="actions" />
        </div>
      </slot>
    </header>

    <div :class="bodyClasses">
      <slot />
    </div>
  </section>
</template>
