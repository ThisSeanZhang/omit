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
    <n-button size="small" dashed :focusable="false"
    @click="routePush('CommandManageView')">
      <template #icon>
        <n-icon><WindowDevEdit20Filled /></n-icon>
      </template>
    </n-button>
    <n-button size="small" dashed :focusable="false"
    @click="shortcut_manager_panel = !shortcut_manager_panel">
      <template #icon>
        <n-icon><TextBulletListSquareEdit24Regular /></n-icon>
      </template>
    </n-button>
    <n-button :focusable="false"
    size="small" dashed @click="routePush('SnapshotManageView')">
      <template #icon>
        <n-icon><CameraAdd24Regular /></n-icon>
      </template>
    </n-button>
    <n-button size="small" :focusable="false" dashed @click="routePush('TerminalWorkView')">
      <template #icon>
        <n-icon><WindowConsole20Regular /></n-icon>
      </template>
    </n-button>
    <n-button size="small" dashed>
      <template #icon>
        <n-icon><Settings24Regular /></n-icon>
      </template>
    </n-button>
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
  // NotepadEdit16Filled,
  TextBulletListSquareEdit24Regular,
  Settings24Regular,
  WindowDevEdit20Filled,
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
    // NotepadEdit16Filled,
    WindowDevEdit20Filled,
    TextBulletListSquareEdit24Regular,
    Settings24Regular,
  },
  setup() {
    const router = useRouter();
    const shortcut_manager_panel = ref(false);
    const exhibit_terminal = ref(true);

    function routePush(name: string) {
      router.push({ name });
    }

    return {
      routePush,
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
