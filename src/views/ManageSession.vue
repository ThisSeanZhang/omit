<template>
<div>
  <n-space vertical size="large">
    <n-layout has-sider position="absolute">
      <n-layout-sider width="33%">
        <n-space style="height: 100%; position: relative">
          <n-layout position="absolute">
            <n-layout-header position="absolute" style="padding: 5px">
              <n-input v-model:value="queryStr" type="text" placeholder="筛选会话" clearable  />
            </n-layout-header>
            <n-layout-content
              style="top: 44px; padding: 5px;"
              position="absolute"
              :native-scrollbar="false"
            >
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <!-- <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/>
              <OmitSession
                v-for="sess in omitSessions"
                v-bind:key="sess" :value="sess"/> -->
            </n-layout-content>
          </n-layout>
        </n-space>
      </n-layout-sider>
        <n-layout-content
          style="flex: 2"
          :native-scrollbar='false'
        >
        <OmitSessionForm v-on:session_save_done="flashSessions" />
        </n-layout-content>
    </n-layout>
  </n-space>
</div>
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
  computed,
} from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
export default defineComponent({
  setup() {
    const queryStr = ref('');
    const omitSessions = ref([] as Array<string>);
    function flashSessions(): void {
      invoke<string[]>('sessions').then(sess => {
        console.log(sess);
        omitSessions.value = sess;
      }).catch((e:string) => console.log(e));
    }
    return {
      queryStr,
      omitSessions,
      flashSessions,
    };
  }

});
</script>
<style scoped>

/* // .welcome-container {
//   width: 100%;
//   height: 100%;
// } */
</style>
