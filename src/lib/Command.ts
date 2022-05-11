import { SelectOption } from 'naive-ui';
import { guid } from './Util';
import CmdOption from './CmdOption';
import CmdParam from './CmdParam';

export default class Command {
  command_id: string;

  command_name: string;
  brief_desc: Map<string, string>;
  description: Map<string, string>;
  version: string;
  platform: string;
  arg_num: number;
  frequency: number;
  options: Array<CmdOption>;
  params: Array<CmdParam>;
  belong_file: string;
  belong_repo: string;

  constructor() {
    this.command_id = '';
    this.command_name = '';
    this.brief_desc = new Map();
    this.description = new Map();
    this.version = '';
    this.platform = '';
    this.arg_num = 0;
    this.frequency = 0;
    this.options = [];
    this.params = [];
    this.belong_file = '';
    this.belong_repo = '';
  }

  static default(): Command {
    const cmd = new Command();
    cmd.command_name = 'docker run';
    cmd.description.set('CN', 'test aaaa');
    cmd.options = CmdOption.default();
    return cmd;
  }

  static fromObj(obj: any):Command {
    const cmd = new Command();
    cmd.command_name = obj.command_name;
    cmd.brief_desc = new Map(Object.entries(obj.brief_desc));
    cmd.description = new Map(Object.entries(obj.description));
    cmd.version = obj.version;
    cmd.platform = obj.platform;
    cmd.arg_num = obj.arg_num;
    cmd.frequency = obj.frequency;
    cmd.options = CmdOption.fromObj(obj.options);
    cmd.params = obj.params;
    cmd.belong_file = obj.belong_file;
    cmd.belong_repo = obj.belong_repo;
    return cmd;
  }

  getLabel() :string {
    return `${this.command_name}`;
  }

  getCommandId():string {
    return `${this.belong_repo}|${this.command_name}`.replace(/\s+/g, ' ');
  }

  toSelectOption(): SelectOption {
    return {
      label: this.getLabel(),
      value: this.getCommandId(),
    };
  }
}
