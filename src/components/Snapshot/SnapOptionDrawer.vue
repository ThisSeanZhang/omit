<template>
<n-drawer v-model:show="active" :width="502" :placement="placement">
  <n-drawer-content title="斯通纳">
    《斯通纳》是美国作家约翰·威廉姆斯在 1965 年出版的小说。
  </n-drawer-content>
</n-drawer>
</template>
<script lang="ts">
import { defineComponent, ref } from 'vue'

export default defineComponent({
  name: 'SnapOptionDrawer',
  setup() {
   return {}; 
  },
});
</script>
