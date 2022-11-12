<template>
<n-layout>
  <n-form
    ref="formRef"
    :label-width="150"
    size="medium"
    label-placement="left"
  >
    <n-form-item :label="i18n.TRANSLATE('init.sessionLocation')" >
      <n-input v-model:value="config.sessions_folder" />
    </n-form-item>
    <n-form-item :label="i18n.TRANSLATE('init.repoLocation')" >
      <n-input v-model:value="config.repos_folder" />
    </n-form-item>
    <n-form-item :label="i18n.TRANSLATE('init.userRepoName')" >
      <n-input v-model:value="config.user_repo_name" />
    </n-form-item>
    <n-row :gutter="[0, 24]">
      <n-col :span="24">
        <div style="display: flex; justify-content: space-between;">
          <n-button
          size="small" 
            @click="emit('back')"
          >
          {{i18n.TRANSLATE('init.backBtn')}}
          </n-button>
          <n-button
          size="small" 
            @click="emit_config"
          >
          {{i18n.TRANSLATE('init.confirmBtn')}}
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
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
const store = useStore();
const props = defineProps({
  config: Config,
})
const emit = defineEmits<{
  (e: 'finish'): void
  (e: 'confirm-config', config: Config): void
  (e: 'back'): void
}>()

const config = ref<Config>(new Config(props.config === undefined ? {} : props.config));

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