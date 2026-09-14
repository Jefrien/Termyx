<script setup lang="ts">
import { onMounted, ref, watch } from "vue"
import { RouterView } from "vue-router"

import AppHeader from "@/components/layout/AppHeader.vue"
import AppSidebar from "@/components/layout/AppSidebar.vue"
import StatusBar from "@/components/layout/StatusBar.vue"

const storedTheme = localStorage.getItem("theme")
const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches
const isDark = ref(storedTheme ? storedTheme === "dark" : prefersDark)

function applyTheme() {
  document.documentElement.classList.toggle("dark", isDark.value)
  localStorage.setItem("theme", isDark.value ? "dark" : "light")
}

function toggleTheme() {
  isDark.value = !isDark.value
}

onMounted(applyTheme)
watch(isDark, applyTheme)
</script>

<template>
  <main class="h-screen overflow-hidden bg-[var(--app-bg)] text-[var(--app-text)]">
    <AppHeader
        :is-dark="isDark"
        @toggle-theme="toggleTheme"
    />

    <div class="flex h-[calc(100vh-3.5rem)] min-h-0">
      <AppSidebar />

      <section class="flex min-w-0 flex-1 flex-col bg-[var(--app-bg)]">
        <RouterView v-slot="{ Component }">
          <Transition
              name="page"
              mode="out-in"
          >
            <component
                :is="Component"
                :is-dark="isDark"
            />
          </Transition>
        </RouterView>

        <StatusBar />
      </section>
    </div>
  </main>
</template>
