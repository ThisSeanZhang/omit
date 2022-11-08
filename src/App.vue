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
<script lang="ts">
import { darkTheme } from 'naive-ui';
// import { invoke } from '@tauri-apps/api/tauri';
import HeadBar from '@/views/HeadBar.vue';
import FooterBar from '@/views/FooterBar.vue';

import {
  defineComponent,
  ref,
} from 'vue';
export default defineComponent({
  components: {
    HeadBar,
    FooterBar,
  },
  setup() {
      //   invoke<string>('current_path')
  //     .then(value => {
  //       this.message = value;
  //     });
    return {
      theme: darkTheme,
    };
  }
});
</script>

