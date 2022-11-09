export default class OmitSession {
  name: string;
  ip: string;
  port: number;
  username: string;
  passwd: string;

  constructor() {
    this.name = '';
    this.ip = '';
    this.port = 22;
    this.username = '';
    this.passwd = '';
  }

  static fromObj(obj: any): OmitSession {
    const result = new OmitSession();
    result.name = obj.name;
    result.ip = obj.ip;
    result.port = obj.port;
    result.username = obj.username;
    result.passwd = obj.passwd;
    return result;
  }
}
