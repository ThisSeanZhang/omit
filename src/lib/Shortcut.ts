import { guid } from "./Util"

export default class Shortcut {
  id: string
  create_time: number
  title: string
  value: string

  constructor(o: {
    id?: string
    create_time?: number;
    title?: string;
    value?: string;
  }) {
    this.id = guid();
    this.create_time = o.create_time ?? new Date().getTime();
    this.title = o.title ?? '';
    this.value = o.value ?? '';
  }

}
