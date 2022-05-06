<template>
  <div style="height: 100%;">
    <div style="height: 100%; background-color: rgb(0,0,0);" ref="terminal"></div>
  </div>
</template>

<script lang="ts">
import 'xterm/css/xterm.css';
import { Terminal } from 'xterm';
import { invoke } from '@tauri-apps/api/tauri';
import { Event } from '@tauri-apps/api/event';
import { getCurrent } from '@tauri-apps/api/window';
import { SerializeAddon } from 'xterm-addon-serialize';
import { FitAddon } from 'xterm-addon-fit';
import {
  onMounted,
  ref,
} from 'vue';

export default {
  name: 'VTerminal',
  setup() {
    const terminal = ref(null as unknown as HTMLDivElement);

    function connectSSH(): void {
      invoke('add_listen');
      // emit an event that are only visible to the current window
      const current = getCurrent();

      console.log(terminal.value);
      const term = new Terminal({
        cols: 129, // 9px
        rows: 33, // 17px
        // rendererType: 'dom'
      });
      const serializeAddon = new SerializeAddon();
      const fit = new FitAddon();
      term.onResize((a:{cols: number, rows: number}) => {
        console.log(`on resize: ${JSON.stringify(a)}`);
        current.emit('ssh-resize-from-front', JSON.stringify({
          SizeChange: {
            width: a.cols,
            height: a.rows,
            width_px: null,
            height_px: null,
          },
        }));
      });
      term.loadAddon(serializeAddon);
      term.loadAddon(fit);
      term.onData(data => {
        console.log(`send data${JSON.stringify(data)}`);
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
      invoke('create_pty');
      if (terminal.value != null) {
        term.open(terminal.value);
      }
      // if (this.$refs.terminal.parentElement != null) {
      //   this.$refs.terminal.parentElement.onresize((_: UIEvent): void => {
      //     console.log('resize');
      //   });
      // }

      // resizeObser(this.$refs.terminal.parentElement, () => {
      //   fit.fit();
      // });
      const resizeObserver = new ResizeObserver(e => {
        const notShrink = e.flatMap(value => [value.contentRect.width, value.contentRect.height])
          .every(value => value !== 0);
        if (notShrink) {
          fit.fit();
        }
        e.forEach(entry => console.log(`${entry.contentRect.width} x ${entry.contentRect.height}`));
        console.log(e.length);
      });
      if (terminal.value != null && terminal.value.parentElement != null) {
        resizeObserver.observe(terminal.value.parentElement);
      }
      fit.fit();
    }

    onMounted(() => {
      connectSSH();
    });

    return {
      terminal,
    };
  },
};
</script>
<style lang="scss" scoped>
</style>
