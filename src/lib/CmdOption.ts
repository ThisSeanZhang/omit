export default class CmdOption {
  brief_name: string;
  full_name: string;
  description: Map<string, string>;
  value: Array<string>;
  // 使用的时候是否忽略类型
  ignore: boolean;
  // 是否能重复选择
  duplicate: boolean;

  constructor() {
    this.brief_name = "";
    this.full_name ="";
    this.description = new Map();
    this.value = new Array();
    this.ignore = false;
    this.duplicate = false;
  }
}
