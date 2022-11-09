<template>
<n-layout size="large" content-style="padding: 20px;">
  <n-layou>
    <n-steps :current="(current as number)">
      <n-step
        title="欢迎"
        description="欢迎使用"
      />
      <n-step
        title="存放位置"
        description="指定运行时: 命令, 快照, 快捷输入. 存放位置"
      />
      <n-step
        title="完成"
        description="开始使用"
      />
    </n-steps>
  </n-layou>
  <n-layout >
    <n-result v-show="current === 1"
      title="欢迎使用"
    >
    <template #icon>
      <n-icon size="40">
        😀
      </n-icon>
    </template>
      <template #footer>
        <n-button @click="nextStep">开始配置</n-button>
      </template>
    </n-result>
    <StorePathVue v-if="current === 2"
      :config ="config"
      @confirm-config="(c) => config = c" 
      @finish="nextStep" @back="comeBack">
    </StorePathVue>
    <!-- 三 -->
    <n-thing v-show="current === 3">
      <template >
        确认配置
      </template>
      <n-list hoverable>
        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              会话存放路径:
            </n-ellipsis>
          </template>
          {{config?.sessions_folder}}
        </n-list-item>

        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              公共仓库存放路径:
            </n-ellipsis>
          </template>
          {{config?.repos_folder}}
        </n-list-item>

        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              私有仓库文件夹名:
            </n-ellipsis>
          </template>
          {{config?.user_repo_name}}
        </n-list-item>
      </n-list>
      <template #action>
        <n-space>
          <n-button size="small" @click="comeBack">
            返回
          </n-button>
          <n-button size="small" @click="confirmConfig">
            确认
          </n-button>
        </n-space>
      </template>
    </n-thing>
  </n-layout>
  <!-- <n-layout-header>颐和园路</n-layout-header>
  <n-layout-content content-style="padding: 24px;">
    {{homeDirPath}}
  </n-layout-content> -->
</n-layout>
</template>
<script setup lang="ts">
import {
  defineComponent,
  ref,
  onMounted,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import StorePathVue from '@/components/Init/StorePath.vue';
import { useStore, Config } from '@/store/config';

const store = useStore();
const router = useRouter();
const current = ref<number>(1);
const config = ref<Config>();
// const currentStatus = ref<string>('process');
  
onMounted(async () => {
  let has_setting = await store.CHECK_SETTING_FILE();
  if (has_setting) {
    router.push({ name: 'Welcome'});
  }
  config.value = store.getConfigClone;
});

async function confirmConfig() {
  if (config.value != undefined) {
    await store.REWRITE_CONFIG(config.value);
    router.push({ name: 'Welcome'});
  }
}

function nextStep() {
  current.value += 1;
}

function comeBack() {
  current.value -= 1;
}
</script>
<style scoped>
</style>