import Command from './Command';
import SnapOption from './SnapOption';
import SnapParam from './SnapParam';
import { guid, SnapExhibitModel } from './Util';

function buildHyphen(option: { brief_name: string, full_name: string }) {
  if (option.brief_name === option.full_name) {
    return '--';
  }
  return '-';
}

export default class Snapshot {
  snap_id: string;
  title: string;
  command_name: string;
  command_id: string;
  description: string;
  option_value: Array<SnapOption>;
  param_value: Array<SnapParam>;

  constructor() {
    this.snap_id = guid();
    this.title = '';
    this.command_name = '';
    this.command_id = '';
    this.description = '';
    this.param_value = [];
    this.option_value = [];
  }

  clone() :Snapshot {
    const snap = new Snapshot();
    snap.snap_id = this.snap_id ?? guid();
    snap.title = this.title;
    snap.command_name = this.command_name;
    snap.command_id = this.command_id;
    snap.description = this.description;
    snap.option_value = this.option_value.map(sp => sp.clone());
    snap.param_value = this.param_value.map(e => e.clone());
    return snap;
  }

  static fromCmd(cmd: Command): Snapshot {
    const snap = new Snapshot();
    snap.command_name = cmd.command_name;
    snap.command_id = cmd.command_id;
    // snap.option_value = SnapOption.default();
    // snap.param_value = SnapParam.default();
    return snap;
  }

  static fromObj(obj: any): Snapshot {
    const snap = new Snapshot();
    snap.snap_id = obj.snap_id ?? guid();
    snap.title = obj.title;
    snap.command_name = obj.command_name;
    snap.command_id = obj.command_id;
    snap.description = obj.description;
    snap.option_value = obj.option_value.map(SnapOption.fromObj);
    snap.param_value = obj.param_value.map(SnapParam.fromObj);
    return snap;
  }

  dealCommandExhibit(
    model: SnapExhibitModel = SnapExhibitModel.ONELINE,
  ): string[] {
    let allRows = [this.command_name];
    const cmdOption = this.option_value
      // .map(option => option.value.filter(p => p.selected)
      // .map(p => `${buildHyphen(option)}${option.brief_name} ${p.value}`))
      .filter(option => option.selected)
      .map(option => `${buildHyphen(option)}${option.brief_name} ${option.value}`);
      // .reduce((a1, a2) => a1.concat(a2), []);
    // console.log(JSON.stringify(cmdOption));
    const cmdParam = this.param_value
      .filter(option => option.selected)
      .map(p => p.value)
      .filter(p => p !== '');
    allRows = allRows.concat(cmdOption);
    allRows = allRows.concat(cmdParam);
    return model === SnapExhibitModel.ONELINE
      ? [allRows.join(' ')]
      : allRows.map((p, index) => (index === allRows.length - 1 ? p : `${p} \\`));
  }
}
