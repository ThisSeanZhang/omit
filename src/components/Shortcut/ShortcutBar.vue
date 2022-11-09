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
      @click="sendCmd(shortcut.value)"
      quaternary strong>
        <n-ellipsis style="max-width: 100px">
        {{shortcut.title}}
        </n-ellipsis>
      </n-button>
    </n-space>
  <!-- </n-space> -->
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
  computed,
} from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { shortcutStore } from '@/store/shortcut';
import Shortcut from '@/lib/Shortcut';

export default defineComponent({
  name: 'ShortcutBar',
  components: {
  },
  setup() {
    const current = getCurrent();
    const store = shortcutStore();
    store.FETCH_SHORTCURS();
    const shortcuts = computed(() => store.shortcuts);
    function sendCmd(data: string) {
      current.emit('ssh-data-from-frontend', `${data}`);
    }
    return {
      sendCmd,
      shortcuts,
    };
  },
});
</script>
<style scoped>
</style>
