<script lang="ts" setup>
import {
computed,
  watchEffect,
  ref,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { footerSwitchStore } from '@/store/footer_switch';
import {
  CameraAdd24Regular,
  WindowConsole20Regular,
  // NotepadEdit16Filled,
  TextBulletListSquareEdit24Regular,
  // Settings24Regular,
  // WindowDevEdit20Filled,
} from '@vicons/fluent';
import {
  Home
} from '@vicons/carbon'
import ShortcutBar from '@/components/Shortcut/ShortcutBar.vue';
import ShortcutManagePanel from '@/components/Shortcut/ShortcutManagePanel.vue';


const router = useRouter();
const route = useRoute();
const switchStore = footerSwitchStore();

const shortcut_manager_panel = ref(false);
const current_panel = ref(false);
const footerSwitch = computed(() => switchStore.barBtnswitch);

const currentViewName = computed(() => route.name ? route.name.toString() : '');
watchEffect(async () => {
  switchStore.SET_CURRENT_WIEW(currentViewName.value);
})

function routePush(name: string) {
  current_panel.value = name === 'TerminalWorkView';
  console.log(current_panel.value);
  router.push({ name });
}
</script>
<template>
<n-space class="footer-bar"
  justify="space-between" align="center"
  item-style="display: flex; align-items: center;" 
>
  <ShortcutBar v-show="footerSwitch.shortcut" />

  <n-button-group>
    <!-- <n-button size="small" ghost>
      <template #icon>
        <n-icon><log-in-icon /></n-icon>
      </template>
      再吃
    </n-button>
    <n-button size="small" ghost>
      <template #icon>
        <n-icon><log-in-icon /></n-icon>
      </template>
      一颗
    </n-button> -->
    <!-- <n-button size="small" dashed :focusable="false"
    @click="routePush('CommandManageView')">
      <template #icon>
        <n-icon><WindowDevEdit20Filled /></n-icon>
      </template>
    </n-button> -->
    <n-button size="small" dashed :focusable="false" @click="routePush('Welcome')">
      <template #icon>
        <n-icon><Home /></n-icon>
      </template>
    </n-button>
    <n-button size="small" dashed :focusable="false" v-if="footerSwitch.shortcut"
      @click="shortcut_manager_panel = !shortcut_manager_panel">
      <template #icon>
        <n-icon><TextBulletListSquareEdit24Regular /></n-icon>
      </template>
    </n-button>
    <n-button size="small" dashed :focusable="false" v-if="footerSwitch.snapshot"
    @click="routePush('SnapshotManageView')">
      <template #icon>
        <n-icon><CameraAdd24Regular /></n-icon>
      </template>
    </n-button>
    <n-button size="small" dashed :focusable="false" v-if="footerSwitch.terminal"
    @click="routePush('TerminalWorkView')">
      <template #icon>
        <n-icon><WindowConsole20Regular /></n-icon>
      </template>
    </n-button>
    <!-- <n-button size="small" dashed>
      <template #icon>
        <n-icon><Settings24Regular /></n-icon>
      </template>
    </n-button> -->
  </n-button-group>
</n-space>
<ShortcutManagePanel v-model:value="shortcut_manager_panel" />
</template>


<style scoped>
.footer-bar{
  height: 100%;
  padding: 0px 5px;
}
</style>
