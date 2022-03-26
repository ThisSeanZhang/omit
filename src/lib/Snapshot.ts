import CmdParam from './CmdParam';
import CmdOption from './CmdOption';

export default class Snapshot {
  brief_name: string;
  param_value: Array<CmdParam>;
  option_value: Array<CmdOption>;
  command_name: string;

  constructor() {
    this.brief_name = "";
    this.param_value = new Array();
    this.option_value = new Array();
    this.command_name = "";
  }
}
