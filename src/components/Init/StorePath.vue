<template>
<n-layout>
  <n-form
    ref="formRef"
    :label-width="150"
    size="medium"
    label-placement="left"
  >
    <n-form-item :label="i18n.TRANSLATE('init.repoLocation')" >
      <n-input disabled="true" v-model:value="setting.base_path" />
    </n-form-item>
    <n-form-item :label="i18n.TRANSLATE('init.sessionLocation')" >
      <n-input v-model:value="setting.sessions_path" />
    </n-form-item>
    <n-form-item :label="i18n.TRANSLATE('init.snapLocation')" >
      <n-input disabled="true" v-model:value="setting.snapshots_path" />
    </n-form-item>

    <!-- <n-form-item :label="i18n.TRANSLATE('init.repoLocation')" >
      <n-input disabled="true" v-model:value="setting.repos_folder" />
    </n-form-item>
    <n-form-item :label="i18n.TRANSLATE('init.userRepoName')" >
      <n-input disabled="true" v-model:value="setting.user_repo_name" />
    </n-form-item> -->
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
            @click="emit_setting"
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
import { useSettingStore } from '@/store/setting';
import i18nStore from '@/store/i18n';
import { Setting } from '@/lib/setting';

const i18n = i18nStore();
const settingStore = useSettingStore();
const props = defineProps({
  setting: Setting,
})

const emit = defineEmits<{
  (e: 'finish'): void
  (e: 'confirm-setting', setting: Setting): void
  (e: 'back'): void
}>()

const setting = ref<Setting>(new Setting(props.setting ?? {}));

// const sessionsFolder = ref(props.setting?.sessions_folder);
// const reposFolder = ref(props.setting?.repos_folder);
// const userRepoName = ref(props.setting?.user_repo_name);
function emit_setting() {
  emit('confirm-setting', setting.value);
  emit('finish');
}
onMounted(async () => {
  // setting.value = store.getConfigClone;
});
</script>
<style scoped>
</style>