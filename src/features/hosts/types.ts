export type HostStatus = "online" | "idle" | "offline"
export type HostAuthMethod = "password" | "privateKey" | "none"
export type PrivateKeyStorageMode = "path" | "imported"

export interface Host {
  id: string
  name: string
  hostname: string
  username: string
  port: number
  favorite: boolean
  status: HostStatus
  authMethod: HostAuthMethod
  privateKeyStorageMode?: PrivateKeyStorageMode
}

export interface HostDraft {
  name: string
  hostname: string
  username: string
  port: number
  favorite: boolean
}

export interface HostCreateInput {
  host: HostDraft
  password: string | null
  privateKeyPath: string | null
  privateKeyContent: string | null
  privateKeyStorageMode: PrivateKeyStorageMode | null
}
