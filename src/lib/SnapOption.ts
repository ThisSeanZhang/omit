import CmdOption from './CmdOption';
import OptionType from './OptionType';

export default class SnapOption {
  brief_name: string;
  full_name: string;

  selected: boolean;
  value: string;

  // 使用的时候是否忽略类型
  ignore: boolean;
  option_type: OptionType;

  constructor() {
    this.brief_name = '';
    this.full_name = '';
    this.value = '';
    this.ignore = false;
    this.selected = true;
    this.option_type = OptionType.NONE;
  }

  isMultip() {
    return this.option_type >= 2 ** 14;
  }

  static fromCmdOption(op: CmdOption): SnapOption {
    const op1 = new SnapOption();
    op1.full_name = op.full_name;
    op1.brief_name = op.brief_name;
    op1.option_type = op.option_type;
    return op1;
  }

  static default(): SnapOption[] {
    const op1 = new SnapOption();
    op1.brief_name = 'p';
    op1.full_name = 'port';
    op1.option_type = OptionType.PAIR;

    const op2 = new SnapOption();
    op2.brief_name = 'd';
    op2.full_name = 'detach';
    op2.option_type = OptionType.NONE;
    return [op1, op2];
  }
}
