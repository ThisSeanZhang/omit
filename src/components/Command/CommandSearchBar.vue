<script lang="ts" setup>
import {
  ref,
  computed,
  defineEmits
} from 'vue';
// import { invoke } from '@tauri-apps/api/tauri';
import useCommandStore from '@/store/command';
import Command from '@/lib/Command'
import { Add } from '@vicons/carbon'

const emit = defineEmits<{
  (event: 'selectCmd', cmd: Command): void
}>()

const commandStore = useCommandStore();
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
const cmds = computed(() => commandStore.commands.map(e => e.toSelectOption())
  // repos.value
  //   .map(e => e.toSelectGroup())
  //   .filter(group => group.children !== undefined && group.children.length > 0)
);
function selectValue(value: string) {
  console.log(value);
  cmd_query_key.value = '';
  const cmd = commandStore.FIND_COMMAND_BY_ID(value)
  if (cmd !== undefined) {
    emit('selectCmd', cmd);
  }
}

</script>
<template>
  <n-input-group>
    <n-button ghost >
      <n-icon>
        <Add />
      </n-icon>
    </n-button>
    <n-select
      v-model:value="cmd_query_key"
      filterable
      placeholder=""
      :options="cmds"
      @update:value="selectValue"
    />
  </n-input-group>
</template>
