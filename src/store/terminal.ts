import { computed, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Term from '@/lib/Term';
import OmitSession from '../lib/OmitSession';
import sessionStore from './session';


export const useStore = defineStore('terminals', () => {
  // init 
  const session = sessionStore();

  // params
  const raw_terms = ref(new Map<string, Term>());
  const current_term_uid = ref('');
  
  // getter
  const current_term = computed( () => raw_terms.value.get(current_term_uid.value));

  // actions
  async function EXHIBIT_TREM(term_id?: string): Promise<Term> {
    console.log(raw_terms.value);
    const use_term_id = term_id ? term_id : current_term_uid.value;
    const current_term = raw_terms.value.get(use_term_id);
    if (current_term === undefined) {
      throw Error("Can not find Session");
    }
    current_term_uid.value = use_term_id;
    return current_term;
  }

  async function CREAT_TREM(sessionName: string): Promise<Term> {
    console.log(raw_terms.value);
    const session_info = session.FIND_SESSION_INFO(sessionName);
    if (session_info === undefined) throw Error("Can not find session info to connect");

    const term = new Term();
    current_term_uid.value = term.uid;
    await term.connect(session_info);
    raw_terms.value.set(term.uid, term);
    return term;
  }

  async function REMOVE_TERM(term_id: string): Promise<void> {

  }
  // async function CREAT_ONE(sessionName: string): Promise<Term> {
  //   console.log("create one: " + sessionName);
  //   const sess = await invoke<OmitSession>('read_session', { sessionName });
  //   const term = new Term(sess);
  //   current_term_uid.value = term.uid;
  //   raw_terms.value.set(term.uid, term);
  //   return term;
  // }
  return {
    current_term_uid,
    current_term,
    EXHIBIT_TREM,
    CREAT_TREM,
    REMOVE_TERM,
  };
  // state: () => ({
  //   raw_terms: ,
  //   current_term_uid: '',
  // }),
  // getters: {
  //   terms: state => state.raw_terms.values(),
  //   current_id: state => state.current_term_uid,
  //   current_term: state => {
  //     const in_map = state.raw_terms.get(state.current_term_uid);
  //     console.log(state.raw_terms);
  //     console.log(state.current_term_uid);
  //     // const term:Term = in_map ? in_map :  new Term();
  //     // state.current_term_uid = term.uid;
  //     // console.log(state.current_term_uid);
  //     // state.raw_terms.set(term.uid, term);
  //     return in_map;
  //   },
  // },
  // actions: {
  //   async CREAT_ONE(sessionName: string): Promise<Term> {
  //     console.log("create one: " + sessionName);
  //     const sess = await invoke<OmitSession>('read_session', { sessionName });
  //     const term = new Term(sess);
  //     this.current_term_uid = term.uid;
  //     this.raw_terms.set(term.uid, term);
  //     return term;
  //   },
  // },
});
export default useStore;
