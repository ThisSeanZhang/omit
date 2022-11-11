import { SelectGroupOption } from 'naive-ui';
import Command from './Command';

export default class Repo {
  name: string;
  commands: Array<Command>;

  constructor(name: string) {
    this.name = name;
    this.commands = [];
  }
  // constructor(name: string, raw_cmds: any) {
  //   this.name = name;
  //   this.commands = [];
  //   Object.keys(raw_cmds).forEach(key => {
  //     const cmds = JSON.parse(raw_cmds[key]) as Command[];
  //     for (let i = 0; i < cmds.length; i += 1) {
  //       const each = Command.fromObj(cmds[i]);
  //       each.belong_file = key;
  //       each.belong_repo = name;
  //       each.command_id = each.getCommandId();
  //       console.log(each);
  //       this.commands.push(each);
  //     }
  //   });
  //   // for (const key in raw_cmds) {
  //   //   const cmds = JSON.parse(raw_cmds[key]) as Command[];
  //   //   for (let i = 0; i < cmds.length; i++) {
  //   //     const each = cmds[i];
  //   //     each.belong_file = key;
  //   //     each.belong_repo = name;
  //   //     this.commands.push(each);
  //   //   }
  //   // }
  // }

  toSelectGroup(): SelectGroupOption {
    return {
      type: 'group',
      label: this.name,
      key: this.name,
      children: this.commands.map(cmd => cmd.toSelectOption()),
    };
  }
}
