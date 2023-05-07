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
    this.terminal = obj.terminal ?? false
    this.snapshot = obj.snapshot ?? false
    this.shortcut = obj.shortcut ?? false
  }
}

export const footerSwitchStore = defineStore('footer-switch', () => {
  const raw_switch = ref(new FooterSwitch({}));
  
  const barBtnswitch = computed(() => raw_switch.value);

  function SET_CURRENT_WIEW(name: string) {
    switch (name) {
      case 'Welcome': {
        raw_switch.value = new FooterSwitch({
          snapshot: true
        });
        break;
      };
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
