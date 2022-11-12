<template>
<n-drawer :show="value"
  @update:show="$emit('update:value', false)"
  to="#drawer-global"
  width="50%" placement="right">
  <n-drawer-content :title="i18n.TRANSLATE('snap.paramEdit')" closable>
    <n-space vertical v-if="params.length === 0">
      <n-button block type="primary" @click="add(0)" dashed>
        {{i18n.TRANSLATE('snap.paramAdd')}}
        <template #icon>
          <n-icon><Add16Filled /></n-icon>
        </template>
      </n-button>
    </n-space>
    <n-space v-else vertical >
      <n-input-group v-for="(param, index) in params" :key="index">
        <n-input v-model:value="param.value" />
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
            :disabled="index === params.length - 1"
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
      </n-input-group>
    </n-space>
  </n-drawer-content>
</n-drawer>
</template>
<script setup lang="ts">
import {
  ref,
  watch,
} from 'vue';
import {
  Add16Filled,
  DismissCircle20Regular,
  ArrowUp16Filled,
  ArrowDown16Filled,
} from '@vicons/fluent';
import SnapParam from '@/lib/SnapParam';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
interface Props {
  value: Boolean;
  snap_params: Array<SnapParam>;
}
const props = withDefaults(defineProps<Props>(), {
  value: () => false,
  snap_params: () =>  [],
});
const params = ref(props.snap_params);

watch(
  () => props.snap_params,
  newOne => {
    params.value = newOne;
  },
);

function move(source: number, step: number | undefined) {
  const option = params.value.splice(source, 1);
  if (step !== undefined) {
    params.value.splice(source + step, 0, option[0]);
  }
}

function add(source: number) {
  params.value.splice(source + 1, 0, new SnapParam());
}
</script>
