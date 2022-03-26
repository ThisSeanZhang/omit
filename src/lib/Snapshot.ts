import CmdParam from './CmdParam';
import CmdOption from './CmdOption';

export default class Snapshot {
  title: string;
  command_name: string;
  brief_name: string;
  option_value: Array<CmdOption>;
  param_value: Array<CmdParam>;

  constructor() {
    this.title = "";
    this.brief_name = "";
    this.param_value = new Array();
    this.option_value = new Array();
    this.command_name = "";
  }
}
