import { watch, ref, computed } from 'vue';
import { defineStore } from 'pinia';
import { useRouter, useRoute } from 'vue-router';

export class FooterSwitch {
  terminal: Boolean;
  snapshot: Boolean;
  shortcut: Boolean;

  constructor(obj: {
    terminal?: Boolean, snapshot?: Boolean, shortcut?: Boolean,
  }) {
    this.terminal = obj.terminal === undefined ? false : obj.terminal;
    this.snapshot = obj.snapshot === undefined ? false : obj.snapshot;
    this.shortcut = obj.shortcut === undefined ? false : obj.shortcut;
  }
}

export const footerSwitchStore = defineStore('footer-switch', () => {
  const raw_switch = ref(new FooterSwitch({}));
  
  const barBtnswitch = computed(() => raw_switch.value);

  function SET_CURRENT_WIEW(name: string) {
    switch (name) {
      case 'TerminalWorkView': {
        raw_switch.value = new FooterSwitch({
          snapshot: true,
          shortcut: true
        });
        break;
      };
      case 'SnapshotManageView': {
        raw_switch.value = new FooterSwitch({
          terminal: true
        });
        break;
      };
      default: {
        raw_switch.value = new FooterSwitch({});
      }
    }
  }
  return {
    barBtnswitch,
    SET_CURRENT_WIEW,
  };
});
export default footerSwitchStore;
