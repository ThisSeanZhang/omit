<template>
<n-layout>
  <n-form
    ref="formRef"
    :label-width="150"
    size="medium"
    label-placement="left"
  >
    <n-form-item label="会话存放路径" >
      <n-input v-model:value="config.sessions_folder" placeholder="输入路径" />
    </n-form-item>
    <n-form-item label="公共仓库存放路径" >
      <n-input v-model:value="config.repos_folder" placeholder="输入路径" />
    </n-form-item>
    <n-form-item label="私有仓库文件夹名" >
      <n-input v-model:value="config.user_repo_name" placeholder="输入路径" />
    </n-form-item>
    <n-row :gutter="[0, 24]">
      <n-col :span="24">
        <div style="display: flex; justify-content: space-between;">
          <n-button
          size="small" 
            @click="emit('back')"
          >
            返回
          </n-button>
          <n-button
          size="small" 
            @click="emit_config"
          >
            确认
          </n-button>
        </div>
      </n-col>
    </n-row>
  </n-form>
</n-layout>
</template>
<script setup lang="ts">
import {
  ref,
  onMounted,
} from 'vue';
import { useStore, Config } from '@/store/config';
const store = useStore();
const props = defineProps({
  config: Config,
})
const emit = defineEmits<{
  (e: 'finish'): void
  (e: 'confirm-config', config: Config): void
  (e: 'back'): void
}>()

const config = ref<Config>(new Config(props.config?.repos_folder, props.config?.sessions_folder, props.config?.user_repo_name));

// const sessionsFolder = ref(props.config?.sessions_folder);
// const reposFolder = ref(props.config?.repos_folder);
// const userRepoName = ref(props.config?.user_repo_name);
function emit_config() {
  emit('confirm-config', config.value);
  emit('finish');
}
onMounted(async () => {
  // config.value = store.getConfigClone;
});
</script>
<style scoped>
</style>