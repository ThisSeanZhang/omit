<template>
  <!-- <n-modal
    :show="value"
    preset="card"
    style="width: 50%;"
    title="保存快照"
    size="small"
    :bordered="false"
    :segmented="segmented"
    @update:show="$emit('update:value', false)"
  > -->
<n-drawer :show="value"
  @update:show="$emit('update:value', false)"
  :on-after-leave="dealClose"
  width="60%" placement="left">
  <n-drawer-content closable>
    <template #header>
        选项编辑
      </template>
    <n-space vertical v-if="shortcuts.length === 0">
      <n-button block type="primary" @click="add(0)" dashed>
        搞点快捷按钮
        <template #icon>
          <n-icon><Add16Filled /></n-icon>
        </template>
      </n-button>
    </n-space>
  <n-list v-else>
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
        <n-button-group>
          <n-button ghost
            @click="add(index)">
            <template #icon>
              <n-icon><Add16Filled /></n-icon>
            </template>
          </n-button>
          <n-button ghost
            :disabled="index === 0"
            @click="move(index, -1)">
            <template #icon>
              <n-icon><ArrowUp16Filled /></n-icon>
            </template>
          </n-button>
          <n-button ghost
            :disabled="index === shortcuts.length - 1"
            @click="move(index, 1)">
            <template #icon>
              <n-icon><ArrowDown16Filled /></n-icon>
            </template>
          </n-button>
          <n-button round type="error" ghost @click="move(index, undefined)">
            <template #icon>
              <n-icon><DismissCircle20Regular /></n-icon>
            </template>
          </n-button>
        </n-button-group>
      </template>
      <!-- <n-space>
        <n-space style="width: 200px" item-style="width: 200px">
          <n-ellipsis style="max-width: 100%">
          {{short.title}}
          </n-ellipsis>
        </n-space>
        <n-space>
          {{short.value}}
        </n-space>
      </n-space> -->
      <n-space vertical>
        <n-input v-model:value="short.title" type="text" placeholder="title" />
        <n-input
          v-model:value="short.value"
          type="textarea"
          placeholder="value"
        />
      </n-space>
    </n-list-item>
  </n-list>
  <!-- </n-modal> -->
  </n-drawer-content>
</n-drawer>
  <ShortcutSavePanel
  v-bind:shortcut="edit_shortcut"
  v-model:value="shortcut_save_panel" />
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
} from 'vue';
import Shortcut from '@/lib/Shortcut';
import { useStore } from '@/store/shortcut';
import {
  Add16Filled,
  DismissCircle20Regular,
  ArrowUp16Filled,
  ArrowDown16Filled,
} from '@vicons/fluent';
import ShortcutSavePanel from '@/components/Shortcut/ShortcutSavePanel.vue';

export default defineComponent({
  name: 'ShortcutManagePanel',
  components: {
    ShortcutSavePanel,
    Add16Filled,
    ArrowUp16Filled,
    ArrowDown16Filled,
    DismissCircle20Regular,
  },
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
    const edit_shortcut = ref(new Shortcut());
    const shortcut_save_panel = ref(false);
    function edit(index: number|undefined) {
      edit_shortcut.value = index === undefined ? new Shortcut() : shortcuts.value[index];
      console.log(edit_shortcut.value);
      shortcut_save_panel.value = true;
    }

    function move(source: number, step: number | undefined) {
      const option = shortcuts.value.splice(source, 1);
      if (step !== undefined) {
        shortcuts.value.splice(source + step, 0, option[0]);
      }
    }
    function add(source: number) {
      shortcuts.value.splice(source + 1, 0, new Shortcut());
    }
    function dealClose() {
      console.log('close');
      storage.SAVE_ALL_SHORTCUR();
    }

    return {
      dealClose,
      add,
      move,
      edit,
      edit_shortcut,
      shortcut_save_panel,
      shortcuts,
    };
  },
});
</script>
