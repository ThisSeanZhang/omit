export default class CmdOption {
  brief_name: string;
  full_name: string;
  description: Map<string, string>;
  value: Array<OptionValue>;
  // 使用的时候是否忽略类型
  ignore: boolean;
  // 是否能重复选择
  duplicate: boolean;
  selected: boolean;
  option_type: number;

  constructor() {
    this.brief_name = "";
    this.full_name ="";
    this.description = new Map();
    this.value = new Array();
    this.ignore = false;
    this.duplicate = false;
    this.selected = false;
    this.option_type = 0;
  }

  isMultip() {
    return this.option_type >= 2 ** 14;
  }
}

interface OptionValue {
  selected: boolean,
  value: string,
}