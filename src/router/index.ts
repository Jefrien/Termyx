import { createRouter, createWebHistory } from "vue-router"

import HostTerminalPage from "@/features/hosts/pages/HostTerminalPage.vue"
import HostsPage from "@/features/hosts/pages/HostsPage.vue"

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/hosts",
    },
    {
      path: "/hosts",
      name: "hosts",
      component: HostsPage,
    },
    {
      path: "/hosts/:hostId",
      name: "host-terminal",
      component: HostTerminalPage,
      props: true,
    },
  ],
})
