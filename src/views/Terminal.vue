<template>
  <div style="height: 100%;overflow: hidden;">
    <div style="height: 100%; background-color: rgb(0,0,0);" ref="terminal_dom"></div>
  </div>
</template>
<script lang="ts">
import 'xterm/css/xterm.css';
import {
  computed,
  defineComponent,
  onBeforeUnmount,
  onMounted,
  ref,
  watch,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useStore } from '@/store/terminal';
import Term from '@/lib/Term';

export default defineComponent({
  name: 'Terminal',
  setup() {
    const route = useRoute();
    const terminalStore = useStore();
    const terminal_dom = ref<HTMLDivElement>();
    const term_id = terminalStore.CREAT_ONE(route.params.sessionName as string);
    console.log(` create in VUE${term_id}`);
    const terminal = ref<Term>();

    onMounted(async () =>  {
      terminal.value = await term_id;
      console.log('onMounted');
      console.log(terminal_dom.value);
      terminal.value.exhibitOn(terminal_dom.value as HTMLDivElement);
      watch(() => terminalStore.current_term, (newOne, oldOne) => {
        oldOne?.dispose(terminal_dom.value as HTMLDivElement);
        if (terminal_dom.value !== undefined) {
          newOne?.exhibitOn(terminal_dom.value as HTMLDivElement);
        }
      });
    });

    onBeforeUnmount(() => {
      console.log('onBeforeUnmount');
      terminal.value?.dispose(terminal_dom.value as HTMLDivElement);
    });

    return {
      terminal_dom,
    };
  },
});
</script>
