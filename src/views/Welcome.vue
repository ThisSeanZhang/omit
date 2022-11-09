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
              <OmitSessionItem
                v-for="sess in exhibitSession"
                @choise="(e) => currentChoice = e"
                v-bind:key="sess.name" :session="sess"/>
            </n-layout-content>
          </n-layout>
        </n-space>
      </n-layout-sider>
        <n-layout-content
          style="flex: 2"
          :native-scrollbar='false'
        >
        <OmitSessionForm :session="currentChoice" @edit_finish="editFinish" />
        </n-layout-content>
    </n-layout>
  </n-space>
</div>
</template>
<script lang="ts">
import {
  computed,
  defineComponent,
  onMounted,
  ref,
} from 'vue';
import { sessionStore } from '@/store/session';
import OmitSessionItem from '@/components/Session/OmitSessionItem.vue';
import OmitSessionForm from '@/components/Session/OmitSessionForm.vue';
import OmitSession from '@/lib/OmitSession';

export default defineComponent({
  name: 'Welcome',
  components: {
    OmitSessionItem,
    OmitSessionForm,
  },
  setup() {
    const store = sessionStore();
    const fetch_wait = store.FETCH_SESSIONS();
    const queryStr = ref('');

    const exhibitSession = computed(() => {
      let result = store.sessions;
      if (queryStr.value !== '') {
        result = result.filter(sess => sess.name.includes(queryStr.value))
      }
      return result;
    });

    const currentChoice = ref(new OmitSession());
    
    function editFinish() {
      currentChoice.value = new OmitSession();
    }
    onMounted(async () => {
      await fetch_wait;
    });

    return {
      queryStr,
      editFinish,
      currentChoice,
      exhibitSession,
    };
  }
});
</script>
<style lang="scss" scoped>
// .welcome-container {
//   width: 100%;
//   height: 100%;
// }
</style>
