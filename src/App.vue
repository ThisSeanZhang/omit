<template>
  <n-config-provider :theme="theme" style="height: 100%;">
  <n-message-provider>
    <!-- <n-space vertical size="large" > -->
        <n-layout-header>
          <HeadBar class="title" />
        </n-layout-header>
  
        <!-- :native-scrollbar="false" -->
        <router-view id="drawer-global" class="contents"/>
        <!-- <n-layout position="absolute" style="top: 30px; bottom: 30px;">
          <router-view />
        </n-layout> -->
        <!-- <n-layout-content
          position="absolute"
          style="top: 30px; bottom: 30px;"
           >
          <router-view/>
        </n-layout-content> -->
  
        <n-layout-footer style="height: 36px;"><footer-bar></footer-bar></n-layout-footer>
    <!-- </n-space> -->
  </n-message-provider>
  </n-config-provider>
  </template>
<script setup lang="ts">
import {
  defineComponent,
  ref,
} from 'vue';
import { darkTheme as theme } from 'naive-ui';
import { useRouter, useRoute } from 'vue-router';
// import { invoke } from '@tauri-apps/api/tauri';
import HeadBar from '@/views/HeadBar.vue';
import FooterBar from '@/views/FooterBar.vue';
import { useStore as configStore } from '@/store/config';

const router = useRouter();
const config = configStore();

if (config.needToInit) {
    router.push({ name: 'InitView'});
}
</script>

