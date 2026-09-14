export type HostStatus = "online" | "idle" | "offline"

export interface Host {
  id: string
  name: string
  hostname: string
  username: string
  port: number
  favorite: boolean
  status: HostStatus
}
