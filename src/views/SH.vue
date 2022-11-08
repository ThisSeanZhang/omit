<template>
  <div>
    <div style="height: 100%; background-color: rgb(0,0,0);" ref="terminal"></div>
  </div>
</template>

<script lang="ts">
// import 'xterm/css/xterm.css';
// import {
//   defineComponent,
//   ref,
//   computed,
// } from 'vue';
// import { Terminal } from 'xterm';
// import { invoke } from '@tauri-apps/api/tauri';
// import { Event } from '@tauri-apps/api/event';
// import { getCurrent } from '@tauri-apps/api/window';
// import { SerializeAddon } from 'xterm-addon-serialize';
// import { FitAddon } from 'xterm-addon-fit';
// import SSHInfo from '@/lib/SSInfo';

// export default defineComponent({
// setup() {
//   const sess = ref(null as null|SSHInfo);

//   console.log(this.$route.params.sessionName);

//   function connectSSH(sess: SSHInfo): void {
//     invoke('add_listen');
//     // emit an event that are only visible to the current window
//     const current = getCurrent();

//     console.log(this.$refs.terminal);
//     const term = new Terminal({
//       cols: 129, // 9px
//       rows: 33, // 17px
//       // rendererType: 'dom'
//     });
//     const serializeAddon = new SerializeAddon();
//     const fit = new FitAddon();
//     term.onResize((a:{cols: number, rows: number}) => {
//       console.log(`on resize: ${JSON.stringify(a)}`);
//       current.emit('ssh-resize-from-front', JSON.stringify({
//         SizeChange: {
//           width: a.cols,
//           height: a.rows,
//           width_px: null,
//           height_px: null,
//         },
//       }));
//     });
//     term.loadAddon(serializeAddon);
//     term.loadAddon(fit);
//     term.onData(data => {
//       current.emit('ssh-data-from-frontend', data);
//     });
//     current.listen('ssh-data-from-backend', (e: Event<{data: Uint8Array}>) => {
//       // console.log(e.payload.message);
//       // const SMCUP = '\u001b[?1049h'; // enable alternative screen
//       // const CUP = '\u001b[H'; // move  cursor to top left
//       // // term.write(`1${SMCUP}${CUP}2`, () => {
//       // //   console.log(serializeAddon.serialize());
//       // // });
//       // const { message } = e.payload;
//       // if (e.payload.message.indexOf(SMCUP)) {
//       //   console.log(`contain SMCUP len ${e.payload.message.length}`);
//       // //   message = e.payload.message.substring(SMCUP.length);
//       // }
//       // if (message.indexOf(CUP)) {
//       //   console.log(`contain CUP len${CUP.length}
//       // index of ${message.indexOf(CUP)} message ${message.length}`);
//       //   // message = message.replace(CUP, '');
//       // }
//       term.write(e.payload.data, () => {
//         console.log(serializeAddon.serialize());
//       });
//       // term.write(message, () => {
//       //   console.log(serializeAddon.serialize());
//       // });
//     });
//     // invoke('new_window');
//     console.log(sess);
//     invoke('create_ssh', {
//       SSHInfo: {
//         ...sess,
//       },
//     });
//     term.open(this.$refs.terminal);
//     // if (this.$refs.terminal.parentElement != null) {
//     //   this.$refs.terminal.parentElement.onresize((_: UIEvent): void => {
//     //     console.log('resize');
//     //   });
//     // }

//     // resizeObser(this.$refs.terminal.parentElement, () => {
//     //   fit.fit();
//     // });
//     const resizeObserver = new ResizeObserver(e => {
//       fit.fit();
//     });
//     if (this.$refs.terminal.parentElement != null) {
//       resizeObserver.observe(this.$refs.terminal.parentElement);
//     }
//     fit.fit();
//   }

//   function mounted(): void {
//     const p = invoke<SSHInfo>('read_session', {
//       sessionName: this.$route.params.sessionName,
//     }).then(sess => {
//       this.sess = sess;
//       this.connectSSH(sess);
//       console.log(sess);
//     }).catch((msg: string) => {
//       console.log(msg);
//     });
//     Promise.all([p]);
//   }
// }
// });
</script>
<style lang="scss" scoped>
</style>
