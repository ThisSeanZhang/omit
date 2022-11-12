<template>
<n-drawer :show="value"
  @update:show="$emit('update:value', false)"
  to="#drawer-global"
  width="50%" placement="right">
  <n-drawer-content title="选项编辑" closable>
    <n-space vertical>
      <n-input-group>
        <n-select
          :value="selectValue"
          :options="cops"
          @update:value="handleSelectValue"
          filterable />
      </n-input-group>
      <n-input-group v-for="(option, index) in ops" :key="index">
        <n-input-group-label>{{option.full_name}}</n-input-group-label>
        <n-input :disabled="option.option_type === ValueType.NONE" v-model:value="option.value" />
        <!-- TODO 多种数据类的判断 -->
        <n-button-group>
          <n-button ghost
            :disabled="index === 0"
            @click="move(index, -1)">
            <template #icon>
              <n-icon><ArrowUp16Filled /></n-icon>
            </template>
          </n-button>
          <n-button ghost
            :disabled="index === ops.length - 1"
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
    <!-- {{cops}} -->
  </n-drawer-content>
</n-drawer>
</template>
<script setup lang="ts">
import {
  ref,
  computed,
  watch,
  defineProps,
} from 'vue';
import {
  DismissCircle20Regular,
  ArrowUp16Filled,
  ArrowDown16Filled,
} from '@vicons/fluent';
import SnapOption from '@/lib/SnapOption';
import ValueType from '@/lib/ValueType';
import CmdOption from '@/lib/CmdOption';

interface Props {
  value: Boolean;
  snap_options: Array<SnapOption>;
  cmd_options: Array<CmdOption>;
}
const props = withDefaults(defineProps<Props>(), {
  value: () => false,
  snap_options: () =>  [],
  cmd_options: () =>  []
})

const selectValue = ref('');
const ops = ref(props.snap_options);
watch(
  () => props.snap_options,
  newOne => {
    ops.value = newOne;
  },
);
const cops = computed(
  () => props.cmd_options
    .map((op: CmdOption) => ({ label: op.full_name, value: op })),
);
function dealClose() {
  console.log('close');
}

function handleSelectValue(value: CmdOption) {
  selectValue.value = '';
  // ops.value.push(props.cmd_options[value]);
  ops.value.push(SnapOption.fromCmdOption(value));
}

function move(source: number, step: number | undefined) {
  const option = ops.value.splice(source, 1);
  if (step !== undefined) {
    ops.value.splice(source + step, 0, option[0]);
  }
}
</script>
