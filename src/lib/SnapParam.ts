import ValueType from './ValueType';

export default class SnapParam {
  selected: boolean;
  param_type: ValueType;
  value: string;

  constructor() {
    this.selected = true;
    this.param_type = ValueType.STRING;
    this.value = '';
  }

  static default(): SnapParam[] {
    const p1 = new SnapParam();
    const p2 = new SnapParam();
    return [p1, p2];
  }
}
