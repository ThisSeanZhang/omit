<template>
  <!-- <n-input type="text"
    v-model:value="cmd_query_key"
    placeholder="filter"
    clearable
  /> -->
  <n-select
      v-model:value="cmd_query_key"
      filterable
      placeholder=""
      :options="cmds"
      @update:value="selectValue"
    />
</template>
<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { useStore } from '@/store/repository';

export default defineComponent({
  name: 'CommandSearchBar',
  components: {
  },
  setup() {
    const store = useStore();
    const cmd_query_key = ref('');
    const raw_cmds = ref([]);
    const cmds = computed(() => (
      raw_cmds.value.map((cmd : any) => ({
        label: cmd.command_name,
        value: cmd.command_name,
      }))));

    invoke<string>('get_commands')
      .then(msg => {
        console.log(msg);
        raw_cmds.value = JSON.parse(msg)[0].commands;
      }).catch((msg:string) => console.log(msg));
    store.FETCH_REPOS().then(list => list.map(store.FETCH_REPO_CMDS));
    const repos = computed(() => store.repos);
    // ref(store.repos);
    function selectValue(value: any) {
      cmd_query_key.value = '';
      console.log(value);
    }
    return {
      cmds,
      cmd_query_key,
      selectValue,
      // repos,
    };
  },
});
</script>
