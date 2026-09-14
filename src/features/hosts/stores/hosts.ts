import { defineStore } from "pinia"

import {
  loadHostsFromVault,
  resetVaultFile,
  saveHostToVault,
  saveHostsToVault,
  vaultExists,
} from "@/features/hosts/services/hostsStorage"
import type { Host, HostCreateInput } from "@/features/hosts/types"

const seedHosts: Host[] = [
  {
    id: "production-api",
    name: "Production API",
    hostname: "192.168.1.20",
    username: "ubuntu",
    port: 22,
    favorite: true,
    status: "online",
    authMethod: "privateKey",
  },
  {
    id: "database",
    name: "Database",
    hostname: "db.internal",
    username: "postgres",
    port: 5432,
    favorite: false,
    status: "idle",
    authMethod: "password",
  },
  {
    id: "minecraft",
    name: "Minecraft",
    hostname: "mc.local",
    username: "admin",
    port: 22,
    favorite: false,
    status: "offline",
    authMethod: "none",
  },
]

function createSeedHosts() {
  return seedHosts.map((host) => ({ ...host }))
}

function slugify(value: string) {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
}

export const useHostsStore = defineStore("hosts", {
  state: () => ({
    hosts: createSeedHosts(),
    vaultAvailable: false,
    vaultUnlocked: false,
    vaultError: null as string | null,
    isLoadingVault: false,
    vaultPassphrase: null as string | null,
  }),
  getters: {
    getHostById: (state) => (hostId: string) =>
      state.hosts.find((host) => host.id === hostId),
    favoriteHosts: (state) => state.hosts.filter((host) => host.favorite),
  },
  actions: {
    async checkVault() {
      this.vaultError = null

      try {
        this.vaultAvailable = await vaultExists()
      } catch (error) {
        this.vaultError = error instanceof Error ? error.message : String(error)
      }
    },
    async unlockVault(passphrase: string) {
      this.isLoadingVault = true
      this.vaultError = null

      try {
        const hosts = await loadHostsFromVault(passphrase)

        if (hosts) {
          this.hosts = hosts
        } else {
          await saveHostsToVault(passphrase, this.hosts)
          this.vaultAvailable = true
        }

        this.vaultUnlocked = true
        this.vaultPassphrase = passphrase
      } catch (error) {
        this.vaultUnlocked = false
        this.vaultPassphrase = null
        this.vaultError = error instanceof Error ? error.message : String(error)
      } finally {
        this.isLoadingVault = false
      }
    },
    async saveVault() {
      this.vaultError = null

      if (!this.vaultPassphrase) {
        this.vaultError = "Unlock the vault before saving."
        return
      }

      try {
        await saveHostsToVault(this.vaultPassphrase, this.hosts)
        this.vaultAvailable = true
      } catch (error) {
        this.vaultError = error instanceof Error ? error.message : String(error)
      }
    },
    async addHost(input: HostCreateInput) {
      const baseId = slugify(input.host.name) || "host"
      const existingIds = new Set(this.hosts.map((host) => host.id))
      let id = baseId
      let suffix = 2

      while (existingIds.has(id)) {
        id = `${baseId}-${suffix}`
        suffix += 1
      }

      const host: Host = {
        id,
        name: input.host.name,
        hostname: input.host.hostname,
        username: input.host.username,
        port: input.host.port,
        favorite: input.host.favorite,
        status: "idle",
        authMethod: input.privateKeyPath ? "privateKey" : input.password ? "password" : "none",
      }

      this.hosts.unshift(host)

      if (this.vaultUnlocked) {
        await this.saveHost(host, input.password, input.privateKeyPath)
      }
    },
    async saveHost(
      host: Host,
      password: string | null = null,
      privateKeyPath: string | null = null,
    ) {
      this.vaultError = null

      if (!this.vaultPassphrase) {
        this.vaultError = "Unlock the vault before saving."
        return
      }

      try {
        await saveHostToVault(this.vaultPassphrase, host, password, privateKeyPath)
        this.vaultAvailable = true
      } catch (error) {
        this.vaultError = error instanceof Error ? error.message : String(error)
      }
    },
    async toggleFavorite(hostId: string) {
      const host = this.hosts.find((item) => item.id === hostId)

      if (!host) return

      host.favorite = !host.favorite

      if (this.vaultUnlocked) {
        await this.saveVault()
      }
    },
    lockVault() {
      this.vaultUnlocked = false
      this.vaultPassphrase = null
    },
    async resetVault() {
      this.isLoadingVault = true
      this.vaultError = null

      try {
        await resetVaultFile()
        this.hosts = createSeedHosts()
        this.vaultAvailable = false
        this.vaultUnlocked = false
        this.vaultPassphrase = null
      } catch (error) {
        this.vaultError = error instanceof Error ? error.message : String(error)
      } finally {
        this.isLoadingVault = false
      }
    },
  },
})
