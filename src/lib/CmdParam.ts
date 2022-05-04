export default class CmdParam {
  cpid: string;
  cid: string;
  sort: number;
  param_name: string;
  description: Map<string, string>;
  required: boolean;
  param_type: string;
  value: string;

  constructor() {
    this.cpid = '';
    this.cid = '';
    this.sort = Number.MAX_VALUE;
    this.param_name = '';
    this.description = new Map();
    this.required = false;
    this.param_type = '';
    this.value = '';
  }
}
