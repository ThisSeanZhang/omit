import Command from './Command';

export default class Repo {
  name: string;
  commands: Map<string, Array<Command>>;

  constructor(name: string, raw_cmds: any) {
    this.name = name;
    this.commands = new Map();
    Object.keys(raw_cmds).forEach(k => this.commands.set(k, JSON.parse(raw_cmds[k])));
  }
}
