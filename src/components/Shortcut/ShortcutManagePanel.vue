<template>
  <n-modal
    :show="value"
    preset="card"
    style="width: 50%;"
    title="保存快照"
    size="small"
    :bordered="false"
    :segmented="segmented"
    @update:show="$emit('update:value', false)"
  >
  <n-list bordered>
    <!-- <template #header>
    </template> -->
    <!-- <template #footer>
    </template> -->
    <n-list-item
    v-for="(short, index) in shortcuts" :key="index">
      <!-- <template #prefix>
        <n-ellipsis style="max-width: 200px">
        {{short.title}}
        </n-ellipsis>
      </template> -->
      <template #suffix>
        <n-button>EDIT</n-button>
      </template>
      <n-space>
        <n-space style="width: 200px" item-style="width: 200px">
          <n-ellipsis style="max-width: 100%">
          {{short.title}}
          </n-ellipsis>
        </n-space>
        <n-space>
          {{short.value}}
        </n-space>
      </n-space>
    </n-list-item>
  </n-list>
  </n-modal>
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
} from 'vue';
import Shortcut from '@/lib/Shortcut';
import { useStore } from '@/store/shortcut';

export default defineComponent({
  name: 'ShortcutManagePanel',
  props: {
    value: {
      type: Boolean,
      require: true,
      default: () => false,
    },
  },
  setup(props) {
    const storage = useStore();

    storage.FETCH_SHORTCURS();
    const shortcuts = ref(storage.shortcuts);
    return {
      shortcuts,
    };
  },
});
</script>
