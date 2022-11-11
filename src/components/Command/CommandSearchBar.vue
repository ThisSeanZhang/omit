<template>
  <n-select
      v-model:value="cmd_query_key"
      filterable
      placeholder=""
      :options="cmds"
      @update:value="selectValue"
    />
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
  computed,
  watch,
} from 'vue';
// import { invoke } from '@tauri-apps/api/tauri';
import { repoStore } from '@/store/repository';
// import Command from '@/lib/Command';

export default defineComponent({
  name: 'CommandSearchBar',
  // emits: ['selectCmd'],
  components: {
  },
  setup(props, { emit }) {
    const store = repoStore();
    const cmd_query_key = ref('');
    const raw_cmds = ref([]);
    // const cmds = computed(() => (
    //   raw_cmds.value.map((cmd : any) => ({
    //     label: cmd.command_name,
    //     value: cmd.command_name,
    //   }))));

    // invoke<string>('get_commands')
    //   .then(msg => {
    //     console.log(msg);
    //     raw_cmds.value = JSON.parse(msg)[0].commands;
    //   }).catch((msg:string) => console.log(msg));
    store.FETCH_REPOS();
    const repos = computed(() => store.repos);
    console.log(repos.value);
    // const cmds = computed(() => {
    //   console.log(repos.value);
    //   const result = [] as any[];
    //   repos.value.flatMap(e => e.commands.forEach(cmd => {
    //     result.push({
    //       label: cmd.getLabel(),
    //       value: cmd.command_name,
    //     });
    //   }));
    //   console.log(result);
    //   return result;
    // });
    const cmds = computed(() => 
      repos.value
        .map(e => e.toSelectGroup())
        .filter(group => group.children !== undefined && group.children.length > 0)
    );
    function selectValue(value: string) {
      console.log(value);
      cmd_query_key.value = '';
      const cmd = store.FIND_CMD_USE(value);
      emit('selectCmd', cmd);
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
