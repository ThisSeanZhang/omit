<template>
<div data-tauri-drag-region class="titlebar">
  <n-button class="titlebar-button" text v-on:click="minimize" alt="minimize">
    <n-icon>
      <!-- <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path d="M480 480H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h448c17.7 0 32 14.3 32 32s-14.3 32-32 32z" fill="currentColor"></path></svg> -->
      <WindowMinimizeRegular></WindowMinimizeRegular>
    </n-icon>
  </n-button>

  <n-button class="titlebar-button" text v-on:click="toggleMaximize" alt="maximize">
    <n-icon>
      <WindowRestoreRegular v-if="max" />
      <WindowMaximizeRegular v-else />
    </n-icon>
  </n-button>

  <n-button class="titlebar-button" text v-on:click="close" alt="close">
    <n-icon>
      <WindowCloseRegular />
    </n-icon>
  </n-button>
</div>
</template>
<script lang="ts">
import {
  ref,
} from 'vue';
import { appWindow } from '@tauri-apps/api/window';
import {
  WindowCloseRegular,
  WindowMaximizeRegular,
  WindowMinimizeRegular,
  WindowRestoreRegular,
} from '@vicons/fa';

export default {
  name: 'HeadBar',
  components: {
    WindowCloseRegular,
    WindowMaximizeRegular,
    WindowMinimizeRegular,
    WindowRestoreRegular,
  },
  setup() {
    const max = ref(false);
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

    return {
      max,
      minimize,
      toggleMaximize,
      close,
    };
  },
};
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
