<template>
  <n-layout has-sider>
        <n-layout-sider
      show-trigger="bar"
      collapsed-width=0
      width="50%">
    </n-layout-sider>
      <n-layout-content content-style="padding: 24px;">
      <n-list>
<n-thing>
    <template #header>
      命令管理
    </template>
    <template #header-extra>
      <n-button circle size="small">
        <template #icon>
          <cash-icon />
        </template>
      </n-button>
      <n-button circle size="small">
        <template #icon>
          <cash-icon />
        </template>
      </n-button>
    </template>
  </n-thing>
      </n-list>
        {{repos}}
        {{repos}}
        {{repos}}
        {{repos}}
        {{repos}}
      <n-list v-for="(repo, repo_index) in repos" :key="repo_index">
        <template #header>
          {{repo.name}}
        </template>
        <n-list-item v-for="(cmd, cmd_index) in repo.commands" :key="cmd_index">
          <template #prefix>
            {{cmd.command_name}}
          </template>
          {{cmd.description.get('zh-CN')}}
          <template #suffix>
            <n-button>Show</n-button>
            <n-button>Edit</n-button>
          </template>
        </n-list-item>
      </n-list>
    </n-layout-content>
  </n-layout>
</template>
<script lang="ts">
import {
  computed,
  defineComponent,
} from 'vue';
import { repoStore } from '@/store/repository';

export default defineComponent({
  name: 'CommandManageView',
  setup() {
    const repos_store = repoStore();
    const repos = computed(() => repos_store.repos);
    return {
      repos,
    };
  },
});
</script>
