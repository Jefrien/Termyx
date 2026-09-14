<script setup lang="ts">
import { computed } from "vue"
import type { Component } from "vue"

import { cn } from "@/lib/cn"

type BadgeVariant = "primary" | "secondary" | "white" | "warning"
type BadgeAppearance = "filled" | "outline" | "text"
type BadgeSize = "sm" | "md"

const props = withDefaults(
  defineProps<{
    variant?: BadgeVariant
    appearance?: BadgeAppearance
    size?: BadgeSize
    icon?: Component
    className?: string
  }>(),
  {
    variant: "secondary",
    appearance: "filled",
    size: "md",
  },
)

const baseClasses =
    "inline-flex max-w-full items-center gap-1.5 rounded-md font-medium"

const sizeClasses: Record<BadgeSize, string> = {
  sm: "px-1.5 py-0.5 text-xs",
  md: "px-2 py-1 text-xs",
}

const appearanceClasses: Record<BadgeVariant, Record<BadgeAppearance, string>> = {
  primary: {
    filled: "bg-[var(--app-accent-soft)] text-[var(--app-text)]",
    outline: "border border-[var(--app-accent)] bg-transparent text-[var(--app-accent)]",
    text: "text-[var(--app-accent)]",
  },
  secondary: {
    filled: "bg-[var(--app-panel-soft)] text-[var(--app-muted-strong)]",
    outline: "border border-[var(--app-border-strong)] bg-transparent text-[var(--app-muted-strong)]",
    text: "text-[var(--app-muted-strong)]",
  },
  white: {
    filled: "bg-white text-[#211f1d]",
    outline: "border border-white/70 bg-transparent text-white",
    text: "text-white",
  },
  warning: {
    filled: "bg-[var(--app-warning)] text-[#21170c]",
    outline: "border border-[var(--app-warning)] bg-transparent text-[var(--app-warning)]",
    text: "text-[var(--app-warning)]",
  },
}

const classes = computed(() => cn(
    baseClasses,
    sizeClasses[props.size],
    appearanceClasses[props.variant][props.appearance],
    props.className,
))
</script>

<template>
  <span :class="classes">
    <component
        :is="icon"
        v-if="icon"
        class="size-3.5"
        aria-hidden="true"
    />

    <span class="min-w-0 truncate">
      <slot />
    </span>
  </span>
</template>
