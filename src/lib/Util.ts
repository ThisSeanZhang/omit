export enum SnapExhibitModel {
  ONELINE,
  MULTLINE,
}

export enum SnapCardExhibitModel {
  SAVE,
  MORE,
  MANAGER_PANEL,
  EXHIBIT_ON_SIDE,
  CREATE_PANEL,
}

export enum PanelType {
  TERMINAL,
  SNAPSHORT,
  COMMAOND,
  SHORTCUT,
  CONFIG,
}

// function buildHyphen(option: { brief_name: string, full_name: string }) {
//   if (option.brief_name === option.full_name) {
//     return '--';
//   }
//   return '-';
// }

// function dealCommandExhibit(
//   snapshot: Snapshot, model: SnapExhibitModel = SnapExhibitModel.ONELINE,
// ): string[] {
//   let allRows = [snapshot.command_name];
//   const cmdOption = snapshot.option_value
//     // .map(option => option.value.filter(p => p.selected)
//     // .map(p => `${buildHyphen(option)}${option.brief_name} ${p.value}`))
//     .filter(option => option.selected)
//     .map(option => `${buildHyphen(option)}${option.brief_name} ${option.value}`);
//     // .reduce((a1, a2) => a1.concat(a2), []);
//   // console.log(JSON.stringify(cmdOption));
//   const cmdParam = snapshot.param_value
//     .filter(option => option.selected)
//     .map(p => p.value)
//     .filter(p => p !== '');
//   allRows = allRows.concat(cmdOption);
//   allRows = allRows.concat(cmdParam);
//   return model === SnapExhibitModel.ONELINE
//     ? [allRows.join(' ')]
//     : allRows.map((p, index) => (index === allRows.length - 1 ? p : `${p} \\`));
// }

/* eslint-disable */
export function guid() {
  return 'xxxx-xxxx-xxxx-4xxx-yxxx-xxxx-xxxx-xxxx'.replace(/[xy]/g, function (c) {
    const r = Math.random() * 16 | 0;
    const v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}
/* eslint-enable */
