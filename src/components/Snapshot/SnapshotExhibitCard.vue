<template>
<n-thing :class="btn_show.border ? 'card' : ''">
  <template #header>
    <n-ellipsis style="max-width: 250px">
    {{snap.title === '' ?'指令预览' : snap.title}}
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
          <n-button @click.stop="sendCmd(snapshot_str_arr)" type="warning" text size="small">
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
          <n-button @click.stop="removeSnap(snap.snap_id)" type="warning" text size="small">
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
<script lang="ts">
import {
  defineComponent,
  ref,
  computed,
  watch,
  PropType,
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
import {
  SnapExhibitModel,
  SnapCardExhibitModel,
} from '@/lib/Util';
import { useStore } from '@/store/snapshot';

interface BtnShow {
  border: Boolean,
  exhibit: Boolean,
  copy: Boolean,
  send: Boolean,
  revise: Boolean,
  delete: Boolean,
  more: Boolean,
  snap_create: Boolean,
}

export default defineComponent({
  name: 'SnapshotExhibitCard',
  components: {
    Copy20Regular,
    ArrowHookUpLeft28Regular,
    LineHorizontal120Filled,
    LineHorizontal520Filled,
    MoreCircle20Regular,
    Delete28Regular,
    PaperPlane,
    CameraAdd24Regular,
  },
  props: {
    snapshot: {
      type: Snapshot,
      require: true,
      default: null,
    },
    exhibit_btn: {
      type: Number as PropType<SnapCardExhibitModel>,
      default: () => SnapCardExhibitModel.EXHIBIT_ON_SIDE,
    },
  },
  setup(props: any) {
    const snap_store = useStore();
    const message = useMessage();
    const current = getCurrent();
    const snap = computed(() => props.snapshot);
    const exhibit_model = ref(SnapExhibitModel.ONELINE);
    const btn_show = ref({} as BtnShow);
    function changeBtnConfig(model: SnapCardExhibitModel):void {
      if (model === SnapCardExhibitModel.EXHIBIT_ON_SIDE) {
        btn_show.value = {
          border: true,
          exhibit: true,
          copy: true,
          send: true,
          revise: false,
          delete: false,
          more: true,
          snap_create: false,
        };
      } else if (model === SnapCardExhibitModel.MANAGER_PANEL) {
        btn_show.value = {
          border: true,
          exhibit: true,
          copy: true,
          send: false,
          revise: true,
          delete: true,
          more: false,
          snap_create: false,
        };
      } else if (model === SnapCardExhibitModel.CREATE_PANEL) {
        btn_show.value = {
          border: false,
          exhibit: true,
          copy: true,
          send: false,
          revise: false,
          delete: false,
          more: false,
          snap_create: true,
        };
      } else if (model === SnapCardExhibitModel.MORE) {
        btn_show.value = {
          border: false,
          exhibit: true,
          copy: true,
          send: true,
          revise: false,
          delete: false,
          more: false,
          snap_create: false,
        };
      } else if (model === SnapCardExhibitModel.SAVE) {
        btn_show.value = {
          border: true,
          exhibit: true,
          copy: true,
          send: false,
          revise: false,
          delete: false,
          more: false,
          snap_create: false,
        };
      }
    }
    changeBtnConfig(props.exhibit_btn);
    watch(() => props.exhibit_btn, newOne => changeBtnConfig(newOne));
    const snapshot_str_arr = computed(
      () => props.snapshot.dealCommandExhibit(exhibit_model.value),
    );

    function sendCmd(data: string) {
      current.emit('ssh-data-from-frontend', `${data}\r`);
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

    return {
      btn_show,
      SnapExhibitModel,
      SnapCardExhibitModel,
      exhibit_model,
      sendCmd,
      copyCmd,
      snap,
      snapshot_str_arr,
      removeSnap: snap_store.REMOVE_SNAPSHOTS,
    };
  },
});
</script>
<style scoped style="scss">
.card {
  margin: 5px;
  padding: 5px;
  border: 1px solid rgba(255, 255, 255, 0.24);
  border-radius: 5px;
}
</style>
