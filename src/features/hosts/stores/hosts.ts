import { defineStore } from "pinia"

import type { Host } from "@/features/hosts/types"

const mockHosts: Host[] = [
  {
    id: "production-api",
    name: "Production API",
    hostname: "192.168.1.20",
    username: "ubuntu",
    port: 22,
    favorite: true,
    status: "online",
  },
  {
    id: "database",
    name: "Database",
    hostname: "db.internal",
    username: "postgres",
    port: 5432,
    favorite: false,
    status: "idle",
  },
  {
    id: "minecraft",
    name: "Minecraft",
    hostname: "mc.local",
    username: "admin",
    port: 22,
    favorite: false,
    status: "offline",
  },
]

export const useHostsStore = defineStore("hosts", {
  state: () => ({
    hosts: mockHosts,
  }),
  getters: {
    getHostById: (state) => (hostId: string) =>
      state.hosts.find((host) => host.id === hostId),
    favoriteHosts: (state) => state.hosts.filter((host) => host.favorite),
  },
})
