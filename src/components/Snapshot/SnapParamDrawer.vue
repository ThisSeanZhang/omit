<template>
<n-drawer :show="show"
  @update:show="$emit('update:value', false)"
  to="#drawer-global"
  width="50%" placement="left">
  <n-drawer-content title="参数编辑" closable>
    <n-space vertical>
      <n-input-group >
        <n-input />
        <n-button-group>
          <n-button ghost>
            <template #icon>
              <n-icon><ArrowUp16Filled /></n-icon>
            </template>
          </n-button>
          <n-button ghost>
            <template #icon>
              <n-icon><ArrowDown16Filled /></n-icon>
            </template>
          </n-button>
          <n-button round type="error" ghost >
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
<script lang="ts">
import {
  defineComponent,
  ref,
  getCurrentInstance,
  computed,
} from 'vue';
import {
  DismissCircle20Regular,
  ArrowUp16Filled,
  ArrowDown16Filled,
} from '@vicons/fluent';
import SnapParam from '@/lib/SnapParam';

export default defineComponent({
  name: 'SnapParamDrawer',
  components: {
    ArrowUp16Filled,
    ArrowDown16Filled,
    DismissCircle20Regular,
  },
  props: {
    value: {
      type: Boolean,
      default: false,
    },
    params: {
      type: Array,
      default: () => [],
    },
  },
  setup(props: any) {
    // const a = getCurrentInstance();
    console.log(props.value);
    const show = computed(() => props.value);

    function dealClose() {
      console.log('close');
    }
    return {
      dealClose,
      show,
    };
  },
});
</script>
