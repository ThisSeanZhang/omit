export default class CmdParam {
  cpid: String;
  cid: String;
  sort: number;
  param_name: String;
  description: Map<String, String>;
  required: boolean;
  param_type: String;
  value: String;

  constructor() {
    this.cpid = "";
    this.cid = "";
    this.sort = Number.MAX_VALUE;
    this.param_name = "";
    this.description = new Map();
    this.required = false;
    this.param_type = "";
    this.value = "";
  }
}