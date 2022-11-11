import { Terminal } from 'xterm';
import { SerializeAddon } from 'xterm-addon-serialize';
import { FitAddon } from 'xterm-addon-fit';
import { guid } from '@/lib/Util';
import { Event } from '@tauri-apps/api/event';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import OmitSession from './OmitSession';


export default class Term {
  private _uid: string;
  private _errorMessage: string | undefined;
  private _session_info: OmitSession | undefined;
  term: Terminal;
  serializeAddon: SerializeAddon;
  fit: FitAddon;
  tauri_window: WebviewWindow;
  resizeObserver: ResizeObserver;
  private _data_emit_name: string;

  constructor() {
    this._uid = guid();
    this._data_emit_name = `data-from-front_${this._uid}`;
    this.term = new Terminal({
      allowProposedApi: true,
      cols: 129, // 9px
      rows: 33, // 17px
      // rendererType: 'dom'
      theme: {
        background: '#101014'
      }
    });
    this.serializeAddon = new SerializeAddon();
    this.fit = new FitAddon();

    // emit an event that are only visible to the current window
    this.tauri_window = getCurrent();

    this.term.loadAddon(this.serializeAddon);
    this.term.loadAddon(this.fit);
    this.term.onData(data => {
      // console.log(`send data${JSON.stringify(data)}`);
      this.tauri_window.emit(this._data_emit_name, data);
    });
    // this.tauri_window.listen(`action-from-backend_${this._uid}`, (e: Event<{data: Uint8Array}>) => {
    //   this.term.write(e.payload.data, () => {
    //     // console.log(this.serializeAddon.serialize());
    //   });
    // });
    this.tauri_window.listen(`action-from-backend_${this._uid}`, (e: Event<{ data: Uint8Array, action: string, message: string}>) => {
      // console.log(e.payload)
      switch (e.payload.action) {
        case 'Message': {
          this.term.write(e.payload.data, () => {
            // console.log(this.serializeAddon.serialize());
          });
          break;
        }
        case 'Eof': {
          this._errorMessage = "Connect close";
        }
      }
      
    });
    // invoke('create_pty', {
    //   SSHInfo: {
    //     ...sess,
    //   },
    // });
    this.resizeObserver = new ResizeObserver(e => {
      const notShrink = e.flatMap(value => [value.contentRect.width, value.contentRect.height])
        .every(value => value !== 0);
      if (notShrink) {
        this.fit.fit();
      }
      // e.forEach(entry => console.log(`${entry.contentRect.width} x ${entry.contentRect.height}`));
      // console.log(e.length);
    });
    this.createListen();
  }

  get uid(): string {
    return this._uid;
  }

  get errorMessage(): string | undefined {
    return this._errorMessage;
  }

  set errorMessage(message: string | undefined) {
    this._errorMessage = message;
  }

  async connect(sess: OmitSession): Promise<void> {
    this._session_info = sess;
    await this._connect();
  }

  async reconnect(): Promise<void> {
    await this._connect();
  }

  private async _connect() : Promise<void> {
    if (this._session_info === undefined) {
      this._errorMessage = "session info config is missing, can not connect"
    }
    try {
      this.term.clear();
      await invoke('create_pty', {
        SSHInfo: {
          uid: this._uid,
          ...this._session_info,
        },
      });
      this._errorMessage = undefined;
    } catch (error) {
      if (error instanceof Error) {
        this._errorMessage = error.message;
      } else {
        this._errorMessage = `${error}`;
      }
      throw error;
    }
  }

  createListen() {
    const action_name = `resize-from-front_${this._uid}`;
    this.term.onResize((a:{cols: number, rows: number}) => {
      // console.log(`on resize: ${JSON.stringify(a)}`);
      this.tauri_window.emit(action_name, JSON.stringify({
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

  is_error(): Boolean {
    return this._errorMessage !== undefined;
  }
}
