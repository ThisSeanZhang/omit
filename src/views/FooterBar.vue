<template>
<n-space item-style="align-items: center;display: flex;" class="footer-bar"
justify="space-between">
  <ShortcutBar />

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
    <n-button size="small" dashed @click="shortcut_manager_panel = !shortcut_manager_panel">
      <template #icon>
        <n-icon><NotepadEdit16Filled /></n-icon>
      </template>
    </n-button>
    <n-button type="info" v-if="exhibit_terminal"
    size="small" dashed @click="workZomeControl(false)">
      <template #icon>
        <n-icon><CameraAdd24Regular /></n-icon>
      </template>
    </n-button>
    <n-button type="info" v-else size="small" dashed @click="workZomeControl(true)">
      <template #icon>
        <n-icon><WindowConsole20Regular /></n-icon>
      </template>
    </n-button>
    <!-- <n-button size="small" ghost @click="shortcut_manager_panel = !shortcut_manager_panel">
      <template #icon>
        <n-icon><TextBulletListAdd24Regular /></n-icon>
      </template>
    </n-button> -->
  </n-button-group>
</n-space>
<ShortcutManagePanel v-model:value="shortcut_manager_panel" />
</template>
<script lang="ts">
import {
  onMounted,
  ref,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import {
  CameraAdd24Regular,
  WindowConsole20Regular,
  NotepadEdit16Filled,
  TextBulletListAdd24Regular,
} from '@vicons/fluent';
import ShortcutBar from '@/components/Shortcut/ShortcutBar.vue';
import ShortcutManagePanel from '@/components/Shortcut/ShortcutManagePanel.vue';

export default {
  name: 'FooterBar',
  components: {
    CameraAdd24Regular,
    WindowConsole20Regular,
    ShortcutManagePanel,
    ShortcutBar,
    NotepadEdit16Filled,
    // TextBulletListAdd24Regular,
  },
  setup() {
    const router = useRouter();
    const shortcut_manager_panel = ref(false);
    const exhibit_terminal = ref(true);

    function workZomeControl(value: boolean) {
      exhibit_terminal.value = value;
      router.push({ name: value ? 'TerminalWorkView' : 'SnapshotManageView' });
    }
    return {
      workZomeControl,
      shortcut_manager_panel,
      exhibit_terminal,
    };
  },
};
</script>

<style lang="scss" scoped>
.footer-bar{
  margin-top: 0px!important;
  padding: 0px 9px;
}
</style>
