<template>
  <div style="height: 100%;overflow: hidden;">
    <div style="height: 100%; background-color: rgb(0,0,0);" ref="terminal_dom"></div>
  </div>
</template>
<script lang="ts">
import {
  computed,
  defineComponent,
  onBeforeUnmount,
  onMounted,
  ref,
  watch,
} from 'vue';
import { useStore } from '@/store/terminal';

export default defineComponent({
  name: 'Terminal',
  setup() {
    const terminalStore = useStore();
    const terminal_dom = ref(null as unknown as HTMLDivElement);
    const terminal = computed(() => terminalStore.current_term);

    onMounted(() => {
      console.log('onMounted');
      console.log(terminal_dom.value);
      terminal.value.exhibitOn(terminal_dom.value);
      watch(() => terminalStore.current_term, (newOne, oldOne) => {
        oldOne.dispose(terminal_dom.value);
        if (terminal_dom.value !== undefined) {
          newOne.exhibitOn(terminal_dom.value);
        }
      });
    });

    onBeforeUnmount(() => {
      console.log('onBeforeUnmount');
      terminal.value.dispose(terminal_dom.value);
    });

    return {
      terminal_dom,
    };
  },
});
</script>
