import Command from './Command';
import SnapOption from './SnapOption';
import SnapParam from './SnapParam';

export default class Snapshot {
  title: string;
  command_name: string;
  brief_name: string;
  option_value: Array<SnapOption>;
  param_value: Array<SnapParam>;

  constructor() {
    this.title = '';
    this.command_name = '';
    this.brief_name = '';
    this.param_value = [];
    this.option_value = [];
  }

  static fromCmd(cmd: Command): Snapshot {
    const snap = new Snapshot();
    snap.command_name = cmd.command_name;
    snap.option_value = SnapOption.default();
    return snap;
  }
}
