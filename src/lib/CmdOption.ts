import OptionType from './OptionType';

interface OptionValue {
  selected: boolean,
  value: string,
}

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
  option_type: OptionType;

  constructor() {
    this.brief_name = '';
    this.full_name = '';
    this.description = new Map();
    this.value = [];
    this.ignore = false;
    this.duplicate = false;
    this.selected = false;
    this.option_type = OptionType.NONE;
  }

  isMultip() {
    return this.option_type >= 2 ** 14;
  }

  static default(): CmdOption[] {
    const op1 = new CmdOption();
    op1.brief_name = 'p';
    op1.full_name = 'port';
    op1.option_type = OptionType.PAIR;

    const op2 = new CmdOption();
    op2.brief_name = 'd';
    op2.full_name = 'detach';
    op2.option_type = OptionType.NONE;

    const op3 = new CmdOption();
    op3.brief_name = 'v';
    op3.full_name = 'volume';
    op3.option_type = OptionType.PAIR;
    return [op1, op2, op3];
  }
}
