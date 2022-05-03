<template>
  <!-- <n-space item-style="align-items: center;display: flex;" class="shortcut-bar"
    justify="space-between"> -->
    <n-space>
      <!-- {{shortcuts}} -->
      <!-- <n-tag type="info"
        v-for="(shortcut, index) in shortcuts" :key="index"
        style="width: 100px; display: flex;justify-content: center;">
        <n-ellipsis style="max-width: 100px">
        {{shortcut.title}}
        </n-ellipsis>
      </n-tag> -->
      <n-button size="tiny" type="info"
      style="width: 100px;"
      v-for="(shortcut, index) in shortcuts" :key="index"
      quaternary strong>
        <n-ellipsis style="max-width: 100px">
        {{shortcut.title}}
        </n-ellipsis>
      </n-button>
    </n-space>
  <!-- </n-space> -->
  <ShortcutSavePanel v-model:value="shortcut_save_panel" />
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
  computed,
} from 'vue';
import { useStore } from '@/store/shortcut';
import ShortcutSavePanel from '@/components/Shortcut/ShortcutSavePanel.vue';
import Shortcut from '@/lib/Shortcut';

export default defineComponent({
  name: 'ShortcutBar',
  components: {
    ShortcutSavePanel,
  },
  setup() {
    const store = useStore();
    store.FETCH_SHORTCURS();
    const shortcut_save_panel = ref(false);
    const shortcuts = computed(() => store.shortcuts);
    return {
      shortcut_save_panel,
      shortcuts,
    };
  },
});
</script>
<style scoped>
</style>
