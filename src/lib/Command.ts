import CmdOption from './CmdOption';
import CmdParam from './CmdParam';

export default class Command {
  cid: string;

  command_name: string;
  brief_desc: Map<string, string>;
  description: Map<string, string>;
  version: string;
  platform: string;
  arg_num: number;
  frequency: number;
  options: Array<CmdOption>;
  params: Array<CmdParam>;

  constructor() {
    this.cid = '';
    this.command_name = '';
    this.brief_desc = new Map();
    this.description = new Map();
    this.version = '';
    this.platform = '';
    this.arg_num = 0;
    this.frequency = 0;
    this.options = [];
    this.params = [];
  }

  static default(): Command {
    const cmd = new Command();
    cmd.command_name = 'docker run';
    cmd.description.set('CN', 'test aaaa');
    cmd.options = CmdOption.default();
    return cmd;
  }
}
