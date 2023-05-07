import { watch, ref, computed } from 'vue';
import { defineStore } from 'pinia';
import Command from '@/lib/Command';

const useCommandStore = defineStore('commands', () => {
  const _commands = ref<Command[]>([]);
  
  const commands = computed(() => _commands.value);

  function FIND_COMMAND_BY_ID(id: string): Command | undefined {
    return _commands.value.find(e => e.command_id === id)
  }
  return {
    commands,
    FIND_COMMAND_BY_ID,
  };
});

export default useCommandStore;