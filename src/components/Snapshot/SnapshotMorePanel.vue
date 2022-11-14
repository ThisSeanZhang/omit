<template>
<n-drawer :show="value"
  @update:show="$emit('update:value', false)"
  to="#drawer-global"
  width="400px" placement="right">
  <n-drawer-content :title="i18n.TRANSLATE('snap.more')" closable>
<n-list style="margin-top: 0px;">
  <!-- <template #header>
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
  </template> -->
  <template #footer>
    <SnapshotExhibitCard :exhibit_btn="SnapCardExhibitModel.MORE" :snapshot="snapshot" />
  </template>
  <n-list-item>
    <template #prefix>
      <n-ellipsis style="max-width: 50px">
        {{i18n.TRANSLATE('snap.option')}}
      </n-ellipsis>
    </template>
    <!-- <template #suffix>
      <n-button type="info" dashed @click="show_option_drawer = true">
        <template #icon>
          <n-icon>
            <TextBulletListSquareEdit20Regular />
          </n-icon>
        </template>
        编辑
      </n-button>
    </template> -->
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
      <n-button v-for="(option, index) in snapshot.option_value" :key="index"
        type="primary"
        icon-placement="right" @click="option.selected = !option.selected"
        :quaternary="!option.selected" :dashed="option.selected" >
        <n-ellipsis style="max-width: 300px">
        {{`${option.full_name} `}}=>{{` ${option.value}`}}
        </n-ellipsis>
      </n-button>
    </n-space>
  </n-list-item>
  <n-list-item>
    <template #prefix>
      <n-ellipsis style="max-width: 50px">
        {{i18n.TRANSLATE('snap.param')}}
      </n-ellipsis>
    </template>
    <n-space align="center" item-style="display: flex;">
      <n-button v-for="(param, index) in snapshot.param_value" :key="index"
        type="primary"
        icon-placement="right" @click="param.selected = !param.selected"
        :quaternary="!param.selected" :dashed="param.selected" >
        <n-ellipsis style="max-width: 300px">
        {{param.value}}
        </n-ellipsis>
      </n-button>
    </n-space>
    <!-- <template #suffix>
      <n-button type="info" dashed @click="show_param_drawer = true">
        <template #icon>
          <n-icon>
            <TextBulletListSquareEdit20Regular />
          </n-icon>
        </template>
        编辑
      </n-button>
    </template> -->
  </n-list-item>
  <!-- <SnapOptionDrawer
    v-model:snap_options="snap.option_value"
    :cmd_options="cmd.options"
    v-model:value="show_option_drawer" />
  <SnapParamDrawer
    v-model:snap_params="snap.param_value"
    v-model:value="show_param_drawer" />
  <SnapshotSavePanel
    v-bind:snap="snap"
    @reflash:snap="reflashSnap"
    v-model:value="show_save_panel" /> -->
  </n-list>
  </n-drawer-content>
</n-drawer>
</template>

<script setup lang="ts">
import {
  ref,
  watch,
} from 'vue';
import {
  TextBulletListSquareEdit20Regular,
  ArrowUp16Filled,
  ArrowDown16Filled,
} from '@vicons/fluent';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';
import Snapshot from '@/lib/Snapshot';
import { SnapCardExhibitModel } from '@/lib/Util';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
interface Props {
  value: Boolean
  snap?: Snapshot
}
const props = withDefaults(defineProps<Props>(), {
  value: () => false,
  snap: () => new Snapshot()
})

const snapshot = ref(props.snap);
watch(() => props.snap, newOne => { snapshot.value = newOne; });

</script>
