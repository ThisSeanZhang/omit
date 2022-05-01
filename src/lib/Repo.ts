import Command from './Command';

export default class Repo {
  name: string;
  commands: Map<string, Array<Command>>;

  constructor(name: string, raw_cmds: Map<string, string>) {
    this.name = name;
    this.commands = new Map();
    raw_cmds.forEach((v, k) => this.commands.set(k, JSON.parse(v)));
  }
}
