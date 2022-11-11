<template>
<div v-show="showTerminal" style="height: 100%;overflow: hidden;">
  <div  style="height: 100%; " ref="terminal_dom">
    <n-space style="height: 100%" justify="center" align="center" v-show="spin">
      <n-spin size="small" />
    </n-space>
  </div>
</div>
<n-space style="height: 100%" justify="center" align="center" v-if="!showTerminal">
  <n-empty style="height: 100%;overflow: hidden;"  :description="errorMessge" >
    <template #icon>
      <n-icon>
        <LinkDismiss20Filled />
      </n-icon>
    </template>
    <template #extra>
      <n-space justify="center">
        <n-button size="small" @click="goToSessionPage">
          换一个
        </n-button>
        <n-button size="small" @click="connectAgain">
          重试
        </n-button>
      </n-space>
    </template>
  </n-empty>
</n-space>
</template>
<script setup lang="ts">
import 'xterm/css/xterm.css';
import {
  computed,
  defineComponent,
  onBeforeUnmount,
  onMounted,
  ref,
  watch,
  WatchStopHandle,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useStore } from '@/store/terminal';
import Term from '@/lib/Term';
import { LinkDismiss20Filled } from '@vicons/fluent';

const route = useRoute();
const router = useRouter();
const terminalStore = useStore();
const terminal_dom = ref<HTMLDivElement>();

const errorMessge = ref<string>();
const showTerminal = computed(() => errorMessge.value === undefined);

const termPromise = route.params.sessionName === undefined
  ? terminalStore.EXHIBIT_TREM(route.params.termId as string)
  : terminalStore.CREAT_TREM(route.params.sessionName as string);

const terminal = ref<Term>();

let spin = ref(true);

function resolveAfter2Seconds() {
  console.log("starting slow promise");
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve("slow");
      console.log("slow promise is done");
    }, 10000);
  });
}
onMounted(async () =>  {
  // console.log('onMounted');
  errorMessge.value == undefined;
  spin.value = true;
  // await resolveAfter2Seconds();
  try {
    terminal.value = await termPromise;
    // console.log(terminal_dom.value);
    terminal.value.exhibitOn(terminal_dom.value as HTMLDivElement);
  } catch (error) {
    if (error instanceof Error) {
      errorMessge.value = error.message;
      // router.go(-1);
    } else {
      errorMessge.value = `${error}`;
      console.log(error);
    }
  }
  spin.value = false;
  // await resolveAfter2Seconds();
  // watch(() => terminalStore.current_term, (newOne, oldOne) => {
  //   oldOne?.dispose(terminal_dom.value as HTMLDivElement);
  //   if (terminal_dom.value !== undefined) {
  //     newOne?.exhibitOn(terminal_dom.value as HTMLDivElement);
  //   }
  // });
});

onBeforeUnmount(() => {
  // console.log('onBeforeUnmount');
  terminal.value?.dispose(terminal_dom.value as HTMLDivElement);
  spin.value = true;
});

function goToSessionPage() {
  // TODO need change to replace
  router.push({ name: 'Welcome'});
}

async function connectAgain() {
  if (terminal.value === undefined) return;
  spin.value = true;
  await terminal.value.reconnect();
  terminal.value.exhibitOn(terminal_dom.value as HTMLDivElement);
  spin.value = false;
}
</script>
<style scoped>
.n-spin-container {
  height: 100%;
}


.n-spin-container > .n-spin-content {
  height: 100%;
}
</style>