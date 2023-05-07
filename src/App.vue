<template>
  <n-config-provider :theme="theme" style="height: 100%;">
  <n-message-provider>
    <!-- <n-space vertical size="large" > -->
        <n-layout-header>
          <HeadBar class="title" />
        </n-layout-header>
  
        <n-layout id="drawer-global" class="contents">
          <router-view/>
        </n-layout>
  
        <n-layout-footer style="height: 36px;"><footer-bar></footer-bar></n-layout-footer>
    <!-- </n-space> -->
  </n-message-provider>
  </n-config-provider>
  </template>
<script setup lang="ts">
import {
  ref,
  onMounted,
} from 'vue';
import { darkTheme as theme } from 'naive-ui';
import { useRouter, useRoute } from 'vue-router';
// import { invoke } from '@tauri-apps/api/tauri';
import HeadBar from '@/views/HeadBar.vue';
import FooterBar from '@/views/FooterBar.vue';
import { useSettingStore } from '@/store/setting';

const router = useRouter();
const settingStore = useSettingStore();

onMounted(async() => {
  try {
    await settingStore.FETCH_SYSTEM_SETTING();
  } catch {
    router.push({ name: 'InitView'});
  }
})
</script>

