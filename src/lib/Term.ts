import { Terminal } from 'xterm';
import { SerializeAddon } from 'xterm-addon-serialize';
import { FitAddon } from 'xterm-addon-fit';
import { guid } from '@/lib/Util';
import { Event } from '@tauri-apps/api/event';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';

export default class Term {
  uid: string;
  term: Terminal;
  serializeAddon: SerializeAddon;
  fit: FitAddon;
  tauri_window: WebviewWindow;
  resizeObserver: ResizeObserver;

  constructor() {
    this.uid = guid();
    this.term = new Terminal({
      cols: 129, // 9px
      rows: 33, // 17px
      // rendererType: 'dom'
    });
    this.serializeAddon = new SerializeAddon();
    this.fit = new FitAddon();

    // emit an event that are only visible to the current window
    this.tauri_window = getCurrent();

    this.term.loadAddon(this.serializeAddon);
    this.term.loadAddon(this.fit);
    this.term.onData(data => {
      console.log(`send data${JSON.stringify(data)}`);
      this.tauri_window.emit('ssh-data-from-frontend', data);
    });
    this.tauri_window.listen('ssh-data-from-backend', (e: Event<{data: Uint8Array}>) => {
      this.term.write(e.payload.data, () => {
        console.log(this.serializeAddon.serialize());
      });
    });
    invoke('create_pty');
    this.resizeObserver = new ResizeObserver(e => {
      const notShrink = e.flatMap(value => [value.contentRect.width, value.contentRect.height])
        .every(value => value !== 0);
      if (notShrink) {
        this.fit.fit();
      }
      e.forEach(entry => console.log(`${entry.contentRect.width} x ${entry.contentRect.height}`));
      // console.log(e.length);
    });
    this.createListen();
  }

  createListen() {
    this.term.onResize((a:{cols: number, rows: number}) => {
      console.log(`on resize: ${JSON.stringify(a)}`);
      this.tauri_window.emit('ssh-resize-from-front', JSON.stringify({
        SizeChange: {
          width: a.cols,
          height: a.rows,
          width_px: null,
          height_px: null,
        },
      }));
    });
  }

  exhibitOn(element: HTMLDivElement) {
    this.term.open(element);
    // this.resizeObserver = new ResizeObserver(e => {
    //   const notShrink = e.flatMap(value => [value.contentRect.width, value.contentRect.height])
    //     .every(value => value !== 0);
    //   if (notShrink) {
    //     this.fit.fit();
    //   }
    //   e.forEach(entry => console.log(
    // `${entry.contentRect.width} x ${entry.contentRect.height}`));
    //   console.log(e.length);
    // });
    if (element.parentElement != null) {
      this.resizeObserver.observe(element.parentElement);
    }
    this.fit.fit();
  }

  dispose(element: HTMLDivElement) {
    if (element.parentElement != null) {
      this.resizeObserver.unobserve(element.parentElement);
    }
    // this.term.dispose();
  }
}
