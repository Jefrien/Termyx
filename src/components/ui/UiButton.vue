<script setup lang="ts">
import { computed } from "vue"
import type { Component } from "vue"

import { cn } from "@/lib/cn"

type ButtonVariant = "primary" | "secondary" | "white"
type ButtonAppearance = "filled" | "outline" | "text" | "link" | "icon"
type ButtonSize = "sm" | "md" | "lg"

const props = withDefaults(
  defineProps<{
    variant?: ButtonVariant
    appearance?: ButtonAppearance
    size?: ButtonSize
    type?: "button" | "submit" | "reset"
    href?: string
    target?: string
    icon?: Component
    iconClassName?: string
    iconPosition?: "left" | "right"
    disabled?: boolean
    active?: boolean
    className?: string
  }>(),
  {
    variant: "primary",
    appearance: "filled",
    size: "md",
    type: "button",
    iconPosition: "left",
  },
)

const baseClasses =
    "inline-flex shrink-0 items-center justify-center gap-2 rounded-md font-medium transition outline-none focus-visible:ring-2 focus-visible:ring-[var(--app-accent)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--app-bg)] disabled:pointer-events-none disabled:opacity-50"

const sizeClasses: Record<ButtonSize, string> = {
  sm: "h-8 px-2.5 text-xs",
  md: "h-9 px-3 text-sm",
  lg: "h-10 px-4 text-sm",
}

const iconSizeClasses: Record<ButtonSize, string> = {
  sm: "size-8 text-xs",
  md: "size-9 text-sm",
  lg: "size-10 text-sm",
}

const appearanceClasses: Record<ButtonVariant, Record<ButtonAppearance, string>> = {
  primary: {
    filled: "border border-[var(--app-accent)] bg-[var(--app-accent)] text-white shadow-sm hover:bg-[var(--app-accent-strong)]",
    outline: "border border-[var(--app-accent)] bg-transparent text-[var(--app-accent)] hover:bg-[var(--app-accent-soft)]",
    text: "border border-transparent bg-transparent text-[var(--app-accent)] hover:bg-[var(--app-accent-soft)]",
    link: "h-auto border border-transparent bg-transparent p-0 text-[var(--app-accent)] underline-offset-4 hover:underline",
    icon: "border border-[var(--app-border-strong)] bg-[var(--app-elevated)] text-[var(--app-accent)] shadow-sm hover:border-[var(--app-accent)]",
  },
  secondary: {
    filled: "border border-[var(--app-border)] bg-[var(--app-panel-soft)] text-[var(--app-text)] hover:bg-[var(--app-elevated)]",
    outline: "border border-[var(--app-border-strong)] bg-transparent text-[var(--app-muted-strong)] hover:bg-[var(--app-panel-soft)] hover:text-[var(--app-text)]",
    text: "border border-transparent bg-transparent text-[var(--app-muted-strong)] hover:bg-[var(--app-panel-soft)] hover:text-[var(--app-text)]",
    link: "h-auto border border-transparent bg-transparent p-0 text-[var(--app-muted-strong)] underline-offset-4 hover:text-[var(--app-text)] hover:underline",
    icon: "border border-[var(--app-border-strong)] bg-[var(--app-elevated)] text-[var(--app-muted-strong)] shadow-sm hover:border-[var(--app-accent)] hover:text-[var(--app-accent)]",
  },
  white: {
    filled: "border border-white bg-white text-[#211f1d] shadow-sm hover:bg-[#f3f1ed]",
    outline: "border border-white/70 bg-transparent text-white hover:bg-white/10",
    text: "border border-transparent bg-transparent text-white hover:bg-white/10",
    link: "h-auto border border-transparent bg-transparent p-0 text-white underline-offset-4 hover:underline",
    icon: "border border-white/70 bg-white text-[#211f1d] shadow-sm hover:bg-[#f3f1ed]",
  },
}

const isIconOnly = computed(() => props.appearance === "icon")
const component = computed(() => props.href ? "a" : "button")
const classes = computed(() => cn(
    baseClasses,
    isIconOnly.value ? iconSizeClasses[props.size] : sizeClasses[props.size],
    appearanceClasses[props.variant][props.appearance],
    props.active && "border-[var(--app-accent)] bg-[var(--app-accent-soft)] text-[var(--app-text)]",
    props.className,
))
</script>

<template>
  <component
      :is="component"
      :class="classes"
      :type="component === 'button' ? type : undefined"
      :href="href"
      :target="target"
      :aria-disabled="disabled || undefined"
      :disabled="component === 'button' ? disabled : undefined"
  >
    <component
        :is="icon"
        v-if="icon && iconPosition === 'left'"
        :class="cn('size-4', iconClassName)"
        aria-hidden="true"
    />

    <span
        v-if="$slots.default && appearance !== 'icon'"
        class="min-w-0 truncate"
    >
      <slot />
    </span>

    <component
        :is="icon"
        v-if="icon && iconPosition === 'right'"
        :class="cn('size-4', iconClassName)"
        aria-hidden="true"
    />
  </component>
</template>
