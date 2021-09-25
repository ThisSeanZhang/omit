<template>
  <div>
    {{sess}}
    <div ref="terminal"></div>
  </div>
</template>

<script lang="ts">
import 'xterm/css/xterm.css';
import { Vue } from 'vue-class-component';
import { Terminal } from 'xterm';
import { invoke } from '@tauri-apps/api/tauri';
import { Event } from '@tauri-apps/api/event';
import { getCurrent } from '@tauri-apps/api/window';
import { SerializeAddon } from 'xterm-addon-serialize';
import SSHInfo from '@/lib/SSInfo';

export default class SH extends Vue {
  $refs!: {
    terminal: HTMLDivElement
  }

  public sess: SSHInfo|undefined;

  created(): void {
    console.log(this.$route.params.sessionName);
  }

  connectSSH(sess: SSHInfo): void {
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
    current.listen('ssh-data-from-backend', (e: Event<{data: Uint8Array}>) => {
      // console.log(e.payload.message);
      // const SMCUP = '\u001b[?1049h'; // enable alternative screen
      // const CUP = '\u001b[H'; // move  cursor to top left
      // // term.write(`1${SMCUP}${CUP}2`, () => {
      // //   console.log(serializeAddon.serialize());
      // // });
      // const { message } = e.payload;
      // if (e.payload.message.indexOf(SMCUP)) {
      //   console.log(`contain SMCUP len ${e.payload.message.length}`);
      // //   message = e.payload.message.substring(SMCUP.length);
      // }
      // if (message.indexOf(CUP)) {
      //   console.log(`contain CUP len${CUP.length}
      // index of ${message.indexOf(CUP)} message ${message.length}`);
      //   // message = message.replace(CUP, '');
      // }
      term.write(e.payload.data, () => {
        console.log(serializeAddon.serialize());
      });
      // term.write(message, () => {
      //   console.log(serializeAddon.serialize());
      // });
    });
    // invoke('new_window');
    console.log(sess);
    invoke('create_ssh', {
      SSHInfo: {
        ...sess,
      },
    });
    term.open(this.$refs.terminal);
  }

  mounted(): void {
    const p = invoke<SSHInfo>('read_session', {
      sessionName: this.$route.params.sessionName,
    }).then(sess => {
      this.sess = sess;
      this.connectSSH(sess);
      console.log(sess);
    }).catch((msg: string) => {
      console.log(msg);
    });
    Promise.all([p]);
  }
}
</script>
<style lang="scss" scoped>
</style>
