import Command from './Command';
import SnapOption from './SnapOption';
import SnapParam from './SnapParam';

export default class Snapshot {
  title: string;
  command_name: string;
  description: string;
  option_value: Array<SnapOption>;
  param_value: Array<SnapParam>;

  constructor() {
    this.title = '';
    this.command_name = '';
    this.description = '';
    this.param_value = [];
    this.option_value = [];
  }

  static fromCmd(cmd: Command): Snapshot {
    const snap = new Snapshot();
    snap.command_name = cmd.command_name;
    snap.option_value = SnapOption.default();
    snap.param_value = SnapParam.default();
    return snap;
  }

  static fromObj(obj: any): Snapshot {
    const snap = new Snapshot();
    snap.title = obj.title;
    snap.command_name = obj.command_name;
    snap.description = obj.description;
    snap.option_value = obj.option_value.map(SnapOption.fromObj);
    snap.param_value = obj.param_value.map(SnapParam.fromObj);
    return snap;
  }
}
