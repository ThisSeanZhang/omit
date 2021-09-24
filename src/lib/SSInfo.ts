export default class SSInfo {
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
}
