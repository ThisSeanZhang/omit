import CmdOption from './CmdOption';
// import CmdParam from './CmdParam';
import Snapshot from './Snapshot';

export enum SnapExhibitModel {
  ONELINE,
  MULTLINE,
}

export enum SnapCardExhibitModel {
  SAVE,
  EXHIBIT_ON_SIDE,
}

export enum PanelType {
  TERMINAL,
  SNAPSHORT,
  COMMAOND,
  SHORTCUT,
  CONFIG,
}

function buildHyphen(option: { brief_name: string, full_name: string }) {
  if (option.brief_name === option.full_name) {
    return '--';
  }
  return '-';
}

function dealCommandExhibit(
  snapshot: Snapshot, model: SnapExhibitModel = SnapExhibitModel.ONELINE,
): string[] {
  let allRows = [snapshot.command_name];
  const cmdOption = snapshot.option_value
    // .map(option => option.value.filter(p => p.selected)
    // .map(p => `${buildHyphen(option)}${option.brief_name} ${p.value}`))
    .filter(option => option.selected)
    .map(option => `${buildHyphen(option)}${option.brief_name} ${option.value}`);
    // .reduce((a1, a2) => a1.concat(a2), []);
  // console.log(JSON.stringify(cmdOption));
  const cmdParam = snapshot.param_value
    .filter(option => option.selected)
    .map(p => p.value)
    .filter(p => p !== '');
  allRows = allRows.concat(cmdOption);
  allRows = allRows.concat(cmdParam);
  return model === SnapExhibitModel.ONELINE
    ? [allRows.join(' ')]
    : allRows.map((p, index) => (index === allRows.length - 1 ? p : `${p} \\`));
}

export {
  dealCommandExhibit,
};
