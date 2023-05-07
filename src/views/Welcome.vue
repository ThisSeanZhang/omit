<template>
<div>
  <n-space vertical size="large">
    <n-layout has-sider position="absolute">
      <n-layout-sider width="33%">
        <n-space style="height: 100%; position: relative">
          <n-layout position="absolute">
            <n-layout-header position="absolute" style="padding: 5px">
              <n-input v-model:value="queryStr" type="text" :placeholder="i18n.TRANSLATE('session.filter')" clearable  />
            </n-layout-header>
            <n-layout-content
              style="top: 44px; padding: 5px;"
              position="absolute"
              :native-scrollbar="false"
            >
              <OmitSessionItem
                v-for="sess in exhibitSession"
                @choise="(e) => currentChoice = e"
                v-bind:key="sess.name" :session="sess"/>
            </n-layout-content>
          </n-layout>
        </n-space>
      </n-layout-sider>
        <n-layout-content>
          <n-space style="height: 100%;" align="center" >
            <OmitSessionForm :session="currentChoice" @edit_finish="editFinish" />
          </n-space>
        </n-layout-content>
    </n-layout>
  </n-space>
</div>
</template>
<script setup lang="ts">
import {
  computed,
  onMounted,
  ref,
} from 'vue';
import { useSessionStore } from '@/store/session';
import OmitSessionItem from '@/components/Session/OmitSessionItem.vue';
import OmitSessionForm from '@/components/Session/OmitSessionForm.vue';
import OmitSession from '@/lib/OmitSession';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
const sessionStore = useSessionStore();
const fetch_wait = sessionStore.FETCH_SESSIONS();
const queryStr = ref('');

const exhibitSession = computed(() => {
  let result = sessionStore.sessions;
  if (queryStr.value !== '') {
    result = result.filter(sess => sess.name.includes(queryStr.value))
  }
  return result;
});

const currentChoice = ref(new OmitSession({}));

function editFinish() {
  currentChoice.value = new OmitSession({});
}

onMounted(async () => {
  await fetch_wait;
});

</script>
<style lang="scss" scoped>
// .welcome-container {
//   width: 100%;
//   height: 100%;
// }
</style>
