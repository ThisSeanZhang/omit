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
    this.selected = false;
    this.option_type = OptionType.NONE;
  }

  isMultip() {
    return this.option_type >= 2 ** 14;
  }
}