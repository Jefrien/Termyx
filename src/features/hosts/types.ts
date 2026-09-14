export type HostStatus = "online" | "idle" | "offline"
export type HostAuthMethod = "password" | "privateKey" | "none"

export interface Host {
  id: string
  name: string
  hostname: string
  username: string
  port: number
  favorite: boolean
  status: HostStatus
  authMethod: HostAuthMethod
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
}
