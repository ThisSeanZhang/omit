<template>
<n-thing class="card">
  <template #header>{{snap.title}}</template>

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
  Copy20Regular,
} from '@vicons/fluent';
import { useMessage } from 'naive-ui';
import Snapshot from '@/lib/Snapshot';
import {
  SnapExhibitModel,
  SnapCardExhibitModel,
  dealCommandExhibit,
} from '@/lib/Util';

export default defineComponent({
  name: 'SnapshotExhibitCard',
  components: {
    Copy20Regular,
    LineHorizontal120Filled,
    LineHorizontal520Filled,
    PaperPlane,
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
    const message = useMessage();
    const current = getCurrent();
    const snap = ref(props.snapshot);
    const exhibit_model = ref(SnapExhibitModel.ONELINE);
    const btn_show = ref({});
    function changeBtnConfig(model: SnapCardExhibitModel):void {
      if (model === SnapCardExhibitModel.EXHIBIT_ON_SIDE) {
        btn_show.value = {
          exhibit: true,
          copy: true,
          send: true,
          restore: true,
        };
      } else if (model === SnapCardExhibitModel.SAVE) {
        btn_show.value = {
          exhibit: true,
          copy: true,
          send: false,
          restore: false,
        };
      }
    }
    changeBtnConfig(props.exhibit_btn);
    watch(() => props.exhibit_btn, newOne => changeBtnConfig(newOne));
    const snapshot_str_arr = computed(
      () => dealCommandExhibit(props.snapshot, exhibit_model.value),
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
