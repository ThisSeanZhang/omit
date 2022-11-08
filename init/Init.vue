<template>
<div>
  initPage
</div>
</template>
<script lang="ts">
import { darkTheme } from 'naive-ui';
// import { invoke } from '@tauri-apps/api/tauri';

import {
  defineComponent,
  ref,
} from 'vue';
export default defineComponent({
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

