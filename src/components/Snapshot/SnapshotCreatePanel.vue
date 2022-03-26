<template>
<!-- <div style="height: 100%; position: relative">
  <n-layout position="absolute">
    <n-layout-header style="height: 64px; padding: 24px" bordered>
      颐和园路
    </n-layout-header>
    <n-layout position="absolute" style="top: 64px; bottom: 64px">
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
    </n-layout>
    <n-layout-footer
      bordered
      position="absolute"
      style="height: 64px; padding: 24px"
    >
      城府路
    </n-layout-footer>
  </n-layout>
</div> -->
<n-list style="padding: 0 10px;">
  <template #header>
    <n-input type="text"
      v-model:value="query_key"
      placeholder="filter"
      clearable
    />
    <n-thing style="margin-top: 10px;">
      <template #header>
        {{cmd.command_name}}
      </template>
        <template  #header-extra>
        <n-button type="warning" text size="small">
          <template #icon>
            <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
          </template>
        </n-button>
        <n-button type="warning" text size="small">
          <template #icon>
            <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
          </template>
        </n-button>
        <n-button type="warning" text size="small">
          <template #icon>
            <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
          </template>
        </n-button>
      </template>
    <template #description>
      {{cmd.description.get('CN')}}
    </template>
    </n-thing>
  </template>
  <template #footer>
    <n-thing>
      <template #header>
        指令预览
      </template>
      <template  #header-extra>
        <n-button type="warning" text size="small">
          <template #icon>
            <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
          </template>
        </n-button>
        <n-button type="warning" text size="small">
          <template #icon>
            <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
          </template>
        </n-button>
        <n-button type="warning" text size="small">
          <template #icon>
            <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
          </template>
        </n-button>
      </template>
      xxxxxx
    </n-thing>
  </template>
  <n-list-item>
    <template #prefix>
      可选项
    </template>
    <template #suffix>
      <n-button type="info" dashed>
        <template #icon>
          <n-icon>
            <TextBulletListSquareEdit20Regular />
          </n-icon>
        </template>
        编辑
      </n-button>
    </template>
    <n-space align="center" item-style="display: flex;">
      <n-tag closable @close="handleClose">
        爱在西元前
      </n-tag>
    </n-space>
  </n-list-item>
  <n-list-item>
    <template #prefix>
      参数
    </template>
    <n-space align="center" item-style="display: flex;">
      <n-tag checkable v-model:checked="check" >
        爱在西元前{{show_param_drawer}}
        <template #avatar>
          <n-button type="warning" text size="small" @click.stop="handleClose">
            <template #icon>
              <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-tag>
    </n-space>
    <template #suffix>
      <n-button type="info" dashed @click="show_param_drawer = true">
        <template #icon>
          <n-icon>
            <TextBulletListSquareEdit20Regular />
          </n-icon>
        </template>
        编辑
      </n-button>
    </template>
  </n-list-item>
  <SnapParamDrawer v-model:value="show_param_drawer" />
  </n-list>
</template>
<script lang="ts">
import { defineComponent, ref } from 'vue';
import { TextBulletListSquareEdit20Regular } from '@vicons/fluent';
import SnapParamDrawer from '@/components/Snapshot/SnapParamDrawer.vue';
import Command from '@/lib/Command';
import Snapshot from '@/lib/Snapshot';

export default defineComponent({
  name: 'SnapshotCreatePanel',
  components: {
    TextBulletListSquareEdit20Regular,
    SnapParamDrawer,
  },
  props: {
    command: {
      type: Command,
      require: true,
      default: Command.default(),
    },
  },
  setup(props: any) {
    const cmd = ref(props.command);
    const snap = ref(Snapshot.fromCmd(props.command));
    const check = ref(false);

    const show_param_drawer = ref(false);

    function handleClose() {
      console.log('handle');
    }
    return {
      show_param_drawer,
      check,
      cmd,
      snap,
      handleClose,
    };
  },
});
</script>
<style scoped>
</style>
