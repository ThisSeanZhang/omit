<template>
<div>
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
</div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { shortcutStore } from '@/store/shortcut';
import { useStore as termStore } from '@/store/terminal';

const current = getCurrent();
const store = shortcutStore();
const term = termStore();

store.FETCH_SHORTCURS();
const shortcuts = computed(() => store.shortcuts);
function sendCmd(data: string) {
  current.emit(`data-from-front_${term.current_term_uid}`, `${data}`);
}
</script>
<style scoped>
</style>
