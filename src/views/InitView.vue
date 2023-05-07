<template>
<n-layout style="height: 100%;" size="large" content-style="padding: 20px;">
  <n-layou>
    <n-steps :current="(current as number)">
      <n-step
        :title="i18n.TRANSLATE('init.welcome')"
        :description="i18n.TRANSLATE('init.welcomeDescription')"
      />
      <n-step
      :title="i18n.TRANSLATE('init.storeLocation')"
      :description="i18n.TRANSLATE('init.storeDescription')"
      />
      <n-step
      :title="i18n.TRANSLATE('init.finish')"
      />
    </n-steps>
  </n-layou>
  <n-layout >
    <n-result v-show="current === 1"
    :title="i18n.TRANSLATE('init.welcomeDescription')"
    >
    <template #icon>
      <n-icon size="40">
        😀
      </n-icon>
    </template>
      <template #footer>
        <n-button @click="nextStep">{{i18n.TRANSLATE('init.startSettingBtn')}}</n-button>
      </template>
    </n-result>
    <StorePathVue v-if="current === 2"
      :setting ="setting"
      @confirm-setting="(c) => setting = c" 
      @finish="nextStep" @back="comeBack">
    </StorePathVue>
    <!-- 三 -->
    <n-thing v-show="current === 3">
      <template >
        {{i18n.TRANSLATE('init.confirmSetting')}}
      </template>
      <n-list hoverable>
        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              {{i18n.TRANSLATE('init.dataLocation')}}:
            </n-ellipsis>
          </template>
          {{setting.data_base_path}}
        </n-list-item>

        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              {{i18n.TRANSLATE('init.sessionLocation')}}:
            </n-ellipsis>
          </template>
          {{setting.sessions_path}}
        </n-list-item>

        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              {{i18n.TRANSLATE('init.snapLocation')}}:
            </n-ellipsis>
          </template>
          {{setting.snapshots_path}}
        </n-list-item>

        <!-- <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              {{i18n.TRANSLATE('init.repoLocation')}}:
            </n-ellipsis>
          </template>
          {{setting?.repos_folder}}
        </n-list-item>

        <n-list-item>
          <template #prefix>
            <n-ellipsis style="max-width: 240px">
              {{i18n.TRANSLATE('init.userRepoName')}}:
            </n-ellipsis>
          </template>
          {{setting?.user_repo_name}}
        </n-list-item> -->
      </n-list>
      <template #action>
        <n-space>
          <n-button size="small" @click="comeBack">
            {{i18n.TRANSLATE('init.backBtn')}}
          </n-button>
          <n-button size="small" @click="confirmSetting">
            {{i18n.TRANSLATE('init.confirmBtn')}}
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
  ref,
  onMounted,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import StorePathVue from '@/components/Init/StorePath.vue';
import { useSettingStore } from '@/store/setting';
import { Setting } from '@/lib/setting';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
const settingStore = useSettingStore();
const router = useRouter();
const current = ref<number>(1);
const setting = ref<Setting>(new Setting(settingStore.setting));
// const currentStatus = ref<string>('process');


async function confirmSetting() {
  try {
    if (setting.value != undefined) {
    await settingStore.SAVE_SETTING(setting.value);
    router.push({ name: 'Welcome'});
  }
  } catch (error) {
    // TODO need send a message to user
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