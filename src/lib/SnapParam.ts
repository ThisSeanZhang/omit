import ValueType, { fromValue } from './ValueType';

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

  static fromObj(obj: any): SnapParam {
    const sp = new SnapParam();
    sp.value = obj.value;
    sp.selected = obj.selected;
    sp.param_type = fromValue(obj.param_type);
    return sp;
  }

  clone():SnapParam {
    const sp = new SnapParam();
    sp.value = this.value;
    sp.selected = this.selected;
    sp.param_type = this.param_type;
    return sp;
  }
}
