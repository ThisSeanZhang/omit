<template>
    <div ref="terminal"></div>
</template>

<script lang="ts">
import 'xterm/css/xterm.css';
import { Vue } from 'vue-class-component';
import { Terminal } from 'xterm';
import { invoke } from '@tauri-apps/api/tauri';
import { Event } from '@tauri-apps/api/event';
import { getCurrent } from '@tauri-apps/api/window';
import { SerializeAddon } from 'xterm-addon-serialize';

export default class SH extends Vue {
  $refs!: {
    terminal: HTMLDivElement
  }

  created(): void {
    console.log('aaa');
  }

  mounted(): void {
    invoke('add_listen');
    // emit an event that are only visible to the current window
    const current = getCurrent();

    console.log(this.$refs.terminal);
    const term = new Terminal({
      cols: 129, // 9px
      rows: 33, // 17px
      // rendererType: 'dom'
    });
    const serializeAddon = new SerializeAddon();
    term.loadAddon(serializeAddon);
    term.onData(data => {
      current.emit('ssh-data-from-frontend', data);
    });
    current.listen('ssh-data-from-backend', (e: Event<{message: string}>) => {
      // console.log(e.payload.message);
      console.log(encodeURIComponent(e.payload.message));
      term.write(e.payload.message, () => {
        console.log(serializeAddon.serialize());
      });
    });
    // invoke('new_window');
    invoke('create_ssh', {
      SSHInfo: {
        ip: '10.1.1.90',
        port: 22,
        username: '',
        passwd: '',
      },
    });
    term.open(this.$refs.terminal);
  }
}
</script>
