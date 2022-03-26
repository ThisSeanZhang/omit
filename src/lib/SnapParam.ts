export default class SnapParam {
  selected: boolean;
  param_type: string;
  value: string;

  constructor() {
    this.selected = false;
    this.param_type = '';
    this.value = '';
  }
}