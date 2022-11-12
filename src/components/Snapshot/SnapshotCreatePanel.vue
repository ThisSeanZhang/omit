<template>
<n-list style="margin-top: 0px;">
  <template #header>
    <n-thing style="margin-top: 10px;">
      <template #header>
        {{cmd.command_name}}
      </template>
        <!-- <template  #header-extra>
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
      </template> -->
    <template #description>
      {{cmd.description.get('zh-CN')}}
    </template>
    </n-thing>
  </template>
  <template #footer>
    <SnapshotExhibitCard class="command-exhibit" @reflash:snap="reflashSnap"
    @open:save="show_save_panel = true"
    :snapshot="snap" :exhibit_btn="SnapCardExhibitModel.CREATE_PANEL">
      <template #title>
        {{i18n.TRANSLATE('snap.title')}}
      </template>
    </SnapshotExhibitCard>
  </template>
  <n-list-item>
    <template #prefix>
      {{i18n.TRANSLATE('snap.option')}}
    </template>
    <template #suffix>
      <n-button type="info" dashed @click="show_option_drawer = true">
        <template #icon>
          <n-icon>
            <TextBulletListSquareEdit20Regular />
          </n-icon>
        </template>
        {{i18n.TRANSLATE('snap.edit')}}
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
        <n-ellipsis style="max-width: 200px">
        {{`${option.full_name} `}}=>{{` ${option.value}`}}
        </n-ellipsis>
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
      {{i18n.TRANSLATE('snap.param')}}
    </template>
    <n-space align="center" item-style="display: flex;">
      <n-button v-for="(param, index) in snap.param_value" :key="index"
        type="primary"
        icon-placement="right" @click="param.selected = !param.selected"
        :quaternary="!param.selected" :dashed="param.selected" >
        <n-ellipsis style="max-width: 200px">
        {{param.value}}
        </n-ellipsis>
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
        {{i18n.TRANSLATE('snap.edit')}}
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
    @reflash:snap="reflashSnap"
    v-model:value="show_save_panel" />
  </n-list>
</template>
<script setup lang="ts">
import {
  computed,
  ref,
  watch,
} from 'vue';
import {
  TextBulletListSquareEdit20Regular,
  DismissCircle16Filled,
} from '@vicons/fluent';
import SnapParamDrawer from '@/components/Snapshot/SnapParamDrawer.vue';
import SnapOptionDrawer from '@/components/Snapshot/SnapOptionDrawer.vue';
import SnapshotSavePanel from '@/components/Snapshot/SnapshotSavePanel.vue';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';
import { SnapExhibitModel, SnapCardExhibitModel } from '@/lib/Util';
import Command from '@/lib/Command';
import Snapshot from '@/lib/Snapshot';
import i18nStore from '@/store/i18n';
interface Props {
  command: Command
  edit_snap: Snapshot
}

const i18n = i18nStore();
const props = defineProps<Props>();
const cmd = computed(() => props.command);
console.log(props.edit_snap)
const snap = ref(props.edit_snap.clone());
watch(() => props.edit_snap, (newOne: Snapshot) => {
  snap.value = newOne.clone();
});

const show_param_drawer = ref(false);
const show_option_drawer = ref(false);
const show_save_panel = ref(false);

function reflashSnap() {
  snap.value = snap.value.clone();
  // console.log('reflashSnap');
}

</script>
<style lang="scss">
.command-exhibit {
  max-width: 100%;
  .n-thing-main {
    max-width: 100%;
  }
}
</style>
