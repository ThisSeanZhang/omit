export default class Shortcut {
  create_time: number;
  title: string;
  value: string;

  constructor() {
    this.create_time = new Date().getTime();
    this.title = '';
    this.value = '';
  }

  static fromObj(obj: any): Shortcut {
    const sc = new Shortcut();
    sc.create_time = obj.create_time;
    sc.title = obj.title;
    sc.value = obj.value;
    return sc;
  }
}
