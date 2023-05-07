import { guid } from "./Util"

export default class OmitSession {
  id: string
  name: string
  ip: string
  port: number
  username: string
  passwd: string

  constructor(o: {
    id?: string
    name?: string
    ip?: string
    port?: number
    username?: string
    passwd?: string
  }) {
    this.id = o.id ?? guid();
    this.name = o.name ?? '';
    this.ip = o.ip ?? '';
    this.port = o.port ?? 22;
    this.username = o.username ?? '';
    this.passwd = o.passwd ?? '';
  }
  
}
