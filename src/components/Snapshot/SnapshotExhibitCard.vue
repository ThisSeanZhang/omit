<template>
<n-thing :class="btn_show.border ? 'card' : ''">
  <template #header>
    <n-ellipsis style="max-width: 250px">
    {{title}}
    </n-ellipsis>
  </template>

  <template #header-extra>
    <n-space>
      <!-- line btn -->
      <n-popover
        v-if="btn_show.exhibit"
        trigger="hover" placement="top-end">
        <span>{{SnapExhibitModel.ONELINE === exhibit_model ? '单行模式' : '多行模式'}}</span>
        <template #trigger>
          <n-button
            @click.stop="exhibit_model = SnapExhibitModel.ONELINE === exhibit_model
            ? SnapExhibitModel.MULTLINE : SnapExhibitModel.ONELINE"
            type="warning" text size="small">
            <template #icon>
              <n-icon size="16">
                <LineHorizontal520Filled v-if="exhibit_model === SnapExhibitModel.ONELINE" />
                <LineHorizontal120Filled v-else />
              </n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <!-- send btn -->
      <n-popover
        v-if="btn_show.send" trigger="hover" placement="top-end">
        <span>发送</span>
        <template #trigger>
          <n-button @click.stop="sendCmd()" type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><PaperPlane /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <!-- copy btn -->
      <n-popover
        v-if="btn_show.copy" trigger="hover" placement="top-end">
        <span>复制</span>
        <template #trigger>
          <n-button @click.stop="copyCmd()" type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><Copy20Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <!-- revise btn -->
      <n-popover
        v-if="btn_show.revise" trigger="hover" placement="top-end">
        <span>重新编辑</span>
        <template #trigger>
          <n-button @click.stop="$emit('revise:snap', snap.clone())"
          type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><ArrowHookUpLeft28Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <!-- delete btn -->
      <n-popover
        v-if="btn_show.delete" trigger="hover" placement="top-end">
        <span>删除</span>
        <template #trigger>
          <n-button @click.stop="snap_store.REMOVE_SNAPSHOTS(snap.snap_id)" type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><Delete28Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <!-- more btn -->
      <n-popover
        v-if="btn_show.more" trigger="hover" placement="top-end">
        <span>更多</span>
        <template #trigger>
          <n-button @click.stop="$emit('open:more', snap)" type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><MoreCircle20Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <!-- more btn -->
      <n-popover
        v-if="btn_show.snap_create" trigger="hover" placement="top-end">
        <span>创建快照</span>
        <template #trigger>
          <n-button @click.stop="$emit('open:save')" type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><CameraAdd24Regular /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
    </n-space>
  </template>
  <div v-for="(line, index) in snapshot_str_arr" :key="index">
    {{line}}
  </div>
</n-thing>
</template>
<script setup lang="ts">
import {
  ref,
  watch,
  computed,
  withDefaults,
} from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { PaperPlane } from '@vicons/fa';
import {
  LineHorizontal120Filled,
  LineHorizontal520Filled,
  ArrowHookUpLeft28Regular,
  MoreCircle20Regular,
  Delete28Regular,
  Copy20Regular,
  CameraAdd24Regular,
} from '@vicons/fluent';
import { useMessage } from 'naive-ui';
import Snapshot from '@/lib/Snapshot';
import BtnShow from '@/lib/BtnShow';
import {
  SnapExhibitModel,
  SnapCardExhibitModel,
} from '@/lib/Util';
import { snapStore } from '@/store/snapshot';
import { useStore as termStore } from '@/store/terminal';


interface Props {
  snapshot: Snapshot
  exhibit_btn?: SnapCardExhibitModel
}
const props = withDefaults(defineProps<Props>(), {
  exhibit_btn: SnapCardExhibitModel.EXHIBIT_ON_SIDE
})

const snap_store = snapStore();
const term = termStore();
const message = useMessage();
const current = getCurrent();
const snap = computed(() => props.snapshot);
const exhibit_model = ref(SnapExhibitModel.ONELINE);

const btn_show = computed(() => BtnShow.useModelCreateShowConfig(props.exhibit_btn));

// watch(() => props.exhibit_btn, newOne => changeBtnConfig(newOne));
const snapshot_str_arr = computed(
  () => props.snapshot? props.snapshot.dealCommandExhibit(exhibit_model.value) : [] as string[],
);

const title = computed(() => snap.value === undefined || snap.value.title === '' ? '指令预览' : snap.value.title );

function sendCmd() {
  current.emit(`data-from-front_${term.current_term_uid}`, snapshot_str_arr.value.join('\r'));
}

function copyCmd() {
  navigator.clipboard
    .writeText(snapshot_str_arr.value.join('\r'))
    .then(() => {
      message.info('复制成功');
    }).catch(err => {
      message.info('复制失败', err);
    });
}
</script>
<style scoped style="scss">
.card {
  margin: 5px;
  padding: 5px;
  border: 1px solid rgba(255, 255, 255, 0.24);
  border-radius: 5px;
}
</style>
