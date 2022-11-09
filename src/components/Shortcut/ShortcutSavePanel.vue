<template>
  <n-modal
    :show="value"
    preset="card"
    style="width: 50%;"
    title="保存快捷按钮"
    size="small"
    :bordered="false"
    @update:show="$emit('update:value', false)"
  >
    <template #header-extra>
    </template>
    {{short}}
    <n-input v-model:value="short.title" type="text" placeholder="输入Title" />
    <template #footer>
      <n-space justify="end">
        <!-- TODO when save the snap need validate snap title -->
        <n-button type="success" @click="saveShortcut">保存</n-button>
      </n-space>
    </template>
  </n-modal>
</template>
<script lang="ts">
import Shortcut from '@/lib/Shortcut';
import { shortcutStore } from '@/store/shortcut';
import {
  defineComponent,
  ref,
  watch,
} from 'vue';

export default defineComponent({
  name: 'ShortcutSavePanel',
  props: {
    value: {
      type: Boolean,
      default: () => false,
    },
    shortcut: {
      type: Shortcut,
      default: () => new Shortcut(),
    },
  },
  setup(props) {
    const storage = shortcutStore();
    const short = ref(props.shortcut);
    // when reopen the modal refresh the shortcut
    watch(() => props.value, () => { short.value = props.shortcut; });

    function saveShortcut() {
      storage.SAVE_SHORTCUR(short.value);
    }

    return {
      short,
      saveShortcut,
    };
  },
});
</script>
