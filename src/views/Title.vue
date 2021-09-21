<template>
<div data-tauri-drag-region class="titlebar">
  <div class="titlebar-button" >
    <img src="https://api.iconify.design/mdi:window-minimize.svg" v-on:click="minimize" alt="minimize"/>
  </div>
  <div class="titlebar-button" @click="toggleMaximize">
    <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize"/>
  </div>
  <div class="titlebar-button" @click="close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
</div>
</template>
<script lang="ts">
import { Vue } from 'vue-class-component';
import { appWindow } from '@tauri-apps/api/window';

export default class Title extends Vue {
  minimize():void {
    appWindow.minimize();
  }

  toggleMaximize():void {
    appWindow.toggleMaximize();
  }

  close():void {
    appWindow.close();
  }
}
</script>

<style lang="scss" scoped>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  // position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
</style>
