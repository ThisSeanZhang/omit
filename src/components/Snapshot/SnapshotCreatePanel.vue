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
    <CommandSearchBar v-on:selectCmd="handleUpdateCmd" />
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
      </template>
    <template #description>
      {{cmd.description.get('zh-CN')}}
    </template>
    </n-thing>
  </template>
  <template #footer>
    <n-thing>
      <template #header>
        指令预览
      </template>
      <template  #header-extra>
        <n-space>
          <n-button v-if="display_model === SnapExhibitModel.ONELINE"
            @click="display_model = SnapExhibitModel.MULTLINE"
            type="warning" text size="small">
            <template #icon>
              <n-icon size="20"><LineHorizontal520Regular /></n-icon>
            </template>
          </n-button>
          <n-button v-else
            @click="display_model = SnapExhibitModel.ONELINE"
            type="warning" text size="medium">
            <template #icon>
              <n-icon size="20"><LineHorizontal120Filled /></n-icon>
            </template>
          </n-button>
          <n-button type="warning" text size="medium"
            @click="copyCmd()">
            <template #icon>
              <n-icon size="20"><Copy20Regular /></n-icon>
            </template>
          </n-button>
          <n-button type="warning" text size="medium"
            @click="show_save_panel = true">
            <template #icon>
              <n-icon size="20"><CameraAdd24Regular /></n-icon>
            </template>
          </n-button>
        </n-space>
      </template>
      <div v-for="(line, index) in command_str" :key="index">{{line}}</div>
    </n-thing>
  </template>
  <n-list-item>
    <template #prefix>
      可选项
    </template>
    <template #suffix>
      <n-button type="info" dashed @click="show_option_drawer = true">
        <template #icon>
          <n-icon>
            <TextBulletListSquareEdit20Regular />
          </n-icon>
        </template>
        编辑
      </n-button>
    </template>
    <n-space align="center" item-style="display: flex;">
      <!-- <n-tag v-for="(option, index) in snap.option_value"
        :key="index" checkable v-model:checked="option.selected" >
        {{option.full_name}}
        <template #avatar>
          <n-button type="warning" text size="small"
            @click.stop="snap.option_value.splice(index, 1)">
            <template #icon>
              <n-icon size="16"><TextBulletListSquareEdit20Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-tag> -->
      <n-button v-for="(option, index) in snap.option_value" :key="index"
        type="primary"
        icon-placement="right" @click="option.selected = !option.selected"
        :quaternary="!option.selected" :dashed="option.selected" >
        {{`${option.full_name} `}}=>{{` ${option.value}`}}
        <template #icon >
          <n-button v-show="!option.selected" text size="small"
            @click.stop="snap.option_value.splice(index, 1)">
            <template #icon>
              <n-icon size="16"><DismissCircle16Filled /></n-icon>
            </template>
          </n-button>
        </template>
      </n-button>
    </n-space>
  </n-list-item>
  <n-list-item>
    <template #prefix>
      参数
    </template>
    <n-space align="center" item-style="display: flex;">
      <n-button v-for="(param, index) in snap.param_value" :key="index"
        type="primary"
        icon-placement="right" @click="param.selected = !param.selected"
        :quaternary="!param.selected" :dashed="param.selected" >
        {{param.value}}
        <template #icon >
          <n-button v-show="!param.selected" text size="small"
            @click.stop="snap.param_value.splice(index, 1)">
            <template #icon>
              <n-icon size="16"><DismissCircle16Filled /></n-icon>
            </template>
          </n-button>
        </template>
      </n-button>
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
  <SnapOptionDrawer
    v-model:snap_options="snap.option_value"
    :cmd_options="cmd.options"
    v-model:value="show_option_drawer" />
  <SnapParamDrawer
    v-model:snap_params="snap.param_value"
    v-model:value="show_param_drawer" />
  <SnapshotSavePanel
    v-bind:snap="snap"
    v-model:value="show_save_panel" />
  </n-list>
</template>
<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import {
  TextBulletListSquareEdit20Regular,
  DismissCircle16Filled,
  LineHorizontal120Filled,
  LineHorizontal520Regular,
  Copy20Regular,
  CameraAdd24Regular,
} from '@vicons/fluent';
import { useMessage } from 'naive-ui';
import { invoke } from '@tauri-apps/api/tauri';
import CommandSearchBar from '@/components/Command/CommandSearchBar.vue';
import SnapParamDrawer from '@/components/Snapshot/SnapParamDrawer.vue';
import SnapOptionDrawer from '@/components/Snapshot/SnapOptionDrawer.vue';
import SnapshotSavePanel from '@/components/Snapshot/SnapshotSavePanel.vue';
import { SnapExhibitModel } from '@/lib/Util';
import Command from '@/lib/Command';
import Snapshot from '@/lib/Snapshot';

export default defineComponent({
  name: 'SnapshotCreatePanel',
  components: {
    SnapshotSavePanel,
    DismissCircle16Filled,
    TextBulletListSquareEdit20Regular,
    CommandSearchBar,
    SnapParamDrawer,
    SnapOptionDrawer,
    LineHorizontal120Filled,
    LineHorizontal520Regular,
    Copy20Regular,
    CameraAdd24Regular,
  },
  props: {
    command: {
      type: Command,
      require: true,
      default: Command.default(),
    },
  },
  setup(props: any) {
    const message = useMessage();
    const cmd = ref(props.command);
    const snap = ref(Snapshot.fromCmd(props.command));
    const check = ref(false);
    const display_model = ref(SnapExhibitModel.ONELINE);
    const command_str = computed(() => snap.value.dealCommandExhibit(display_model.value));

    const show_param_drawer = ref(false);
    const show_option_drawer = ref(false);
    const show_save_panel = ref(false);

    function handleClose() {
      console.log('handle');
    }

    function copyCmd() {
      navigator.clipboard
        .writeText(command_str.value.join('\r'))
        .then(() => {
          message.info('复制成功');
        }).catch(err => {
          message.info('复制失败', err);
        });
    }
    function handleUpdateCmd(updateCmd) {
      cmd.value = updateCmd;
      snap.value = Snapshot.fromCmd(updateCmd);
    }
    return {
      show_save_panel,
      handleUpdateCmd,
      copyCmd,
      SnapExhibitModel,
      display_model,
      command_str,
      show_param_drawer,
      show_option_drawer,
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
