<template>
<n-drawer :show="value"
  @update:show="$emit('update:value', false)"
  :on-after-leave="dealClose"
  width="60%" placement="left">
  <n-drawer-content closable>
    <template #header>
        {{i18n.TRANSLATE('param.editParam')}}
      </template>
    <n-space vertical v-if="shortcuts.length === 0">
      <n-button block type="primary" @click="add(0)" dashed>
        {{i18n.TRANSLATE('param.addParam')}}
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
  </n-drawer-content>
</n-drawer>
  <ShortcutSavePanel
  v-bind:shortcut="edit_shortcut"
  v-model:value="shortcut_save_panel" />
</template>
<script setup lang="ts">
import {
  onMounted,
  ref,
} from 'vue';
import Shortcut from '@/lib/Shortcut';
import { shortcutStore } from '@/store/shortcut';
import {
  Add16Filled,
  DismissCircle20Regular,
  ArrowUp16Filled,
  ArrowDown16Filled,
} from '@vicons/fluent';
import ShortcutSavePanel from '@/components/Shortcut/ShortcutSavePanel.vue';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
interface Props {
  value: Boolean
}
const props = withDefaults(defineProps<Props>(), {
  value: () => false
})

const storage = shortcutStore();

const await_fetch = storage.FETCH_SHORTCURS();
const shortcuts = ref<Shortcut[]>([]);

const edit_shortcut = ref(new Shortcut());
const shortcut_save_panel = ref(false);

onMounted(async () => {
  await await_fetch;
  shortcuts.value = storage.shortcuts;
});
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
  storage.SAVE_ALL_SHORTCUR(shortcuts.value);
}
</script>
