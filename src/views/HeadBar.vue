<template>
<div data-tauri-drag-region class="titlebar">
  <n-button :focusable="false" class="titlebar-button" text v-on:click="change" alt="change">
    <n-icon>
      <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d=" M12.87 15.07l-2.54-2.51.03-.03c1.74-1.94 2.98-4.17 3.71-6.53H17V4h-7V2H8v2H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7l1.62-4.33L19.12 17h-3.24z "></path></svg>
    </n-icon>
  </n-button>
  <n-button-group>
  <n-button :focusable="false" class="titlebar-button" text v-on:click="minimize" alt="minimize">
    <n-icon>
      <!-- <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path d="M480 480H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h448c17.7 0 32 14.3 32 32s-14.3 32-32 32z" fill="currentColor"></path></svg> -->
      <WindowMinimizeRegular></WindowMinimizeRegular>
    </n-icon>
  </n-button>

  <n-button :focusable="false" class="titlebar-button" text v-on:click="toggleMaximize" alt="maximize">
    <n-icon>
      <WindowRestoreRegular v-if="max" />
      <WindowMaximizeRegular v-else />
    </n-icon>
  </n-button>

  <n-button :focusable="false" class="titlebar-button" text v-on:click="close" alt="close">
    <n-icon>
      <WindowCloseRegular />
    </n-icon>
  </n-button>
  </n-button-group>
</div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { appWindow } from '@tauri-apps/api/window';
import {
  WindowCloseRegular,
  WindowMaximizeRegular,
  WindowMinimizeRegular,
  WindowRestoreRegular,
} from '@vicons/fa';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
const max = ref(false);
function change() {
  const set_lang = i18n.language === 'cn' ? 'en' : 'cn';
  i18n.SET_LAN(set_lang);
}
function minimize():void {
  appWindow.minimize();
}
appWindow.listen('tauri://resize', () => {
  appWindow.isMaximized().then((b: boolean) => { max.value = b; });
});

function toggleMaximize():void {
  max.value = !max.value;
  appWindow.toggleMaximize();
}

function close():void {
  appWindow.close();
}

</script>

<style scoped>
.titlebar {
  height: 30px;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  font-size: 12px;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
/* // .titlebar-button:hover {
//   background: #5bbec3;
// } */
</style>
