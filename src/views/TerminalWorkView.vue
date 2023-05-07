<template>
  <n-layout style="height: 100%;" has-sider sider-placement="right">
    <n-layout-content content-style="padding-right: 20px;">
      <Terminal></Terminal>
    </n-layout-content>
    <n-layout-sider
      show-trigger="bar"
      collapsed-width=0
      width="400">
      <div style="position: relative;height: calc(100%);">
        <n-layout position="absolute">
          <n-layout-header style="height: 34px; padding-right: 10px;">
            <n-input type="text"
              v-model:value="query_key"
              :placeholder="i18n.TRANSLATE('snap.filter')"
              clearable
            />
          </n-layout-header>
          <n-layout-content position="absolute"
            style="top: 34px; padding-right: 5px;">
            <!-- <SnapshotPanel :filter="query_key" /> -->
            <SnapshotExhibitCard
            v-for="snap in snapshots"
            @open:more="(snap) => { more_snap = snap; show_more = true; }"
            :key="snap.command_name"
            :snapshot="snap"/>
            <SnapshotMorePanel
            v-bind:snap="more_snap"
            v-model:value="show_more"/>
          </n-layout-content>
          <!-- <n-layout-footer position="absolute" style="height: 44px; padding: 5px">
            <n-space justify="end">
              <n-popover v-if="exhibit_terminal" placement="top-end" trigger="hover">
                <template #trigger>
                  <n-button type="info" @click.stop="exhibit_terminal = !exhibit_terminal" dashed>
                    <template #icon>
                      <n-icon><CameraAdd24Regular /></n-icon>
                    </template>
                  </n-button>
                </template>
                <span>增加快照</span>
              </n-popover>
              <n-popover v-else placement="top-end" trigger="hover">
                <template #trigger>
                  <n-button type="info" @click.stop="exhibit_terminal = !exhibit_terminal" dashed>
                    <template #icon>
                      <n-icon><WindowConsole20Regular /></n-icon>
                    </template>
                  </n-button>
                </template>
                <span>返回终端</span>
              </n-popover>
            </n-space>
          </n-layout-footer> -->
        </n-layout>
      </div>
    </n-layout-sider>
  </n-layout>
</template>

<script setup lang="ts">
import {
  computed,
  ref,
} from 'vue';
// import { CameraAdd24Regular, WindowConsole20Regular } from '@vicons/fluent';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';
import SnapshotMorePanel from '@/components/Snapshot/SnapshotMorePanel.vue';
import { snapStore } from '@/store/snapshot';
import Terminal from './Terminal.vue';
import Snapshot from '@/lib/Snapshot';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
const storage = snapStore();
const query_key = ref('');
const show_more = ref(false);
const more_snap = ref<Snapshot>();
// const exhibit_terminal = ref(true);
const exhibit_terminal = ref(false);
storage.FETCH_SNAPSHOTS();
const snapshots = computed(() => storage.snapshots.filter(snap => (query_key.value === '' ? true : snap.title.includes(query_key.value))));

</script>
