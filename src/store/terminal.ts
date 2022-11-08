import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Term from '@/lib/Term';
import OmitSession from '../lib/OmitSession';

export const useStore = defineStore('terminals', {
  state: () => ({
    raw_terms: new Map<string, Term>(),
    current_term_uid: '',
  }),
  getters: {
    terms: state => state.raw_terms.values(),
    current_id: state => state.current_term_uid,
    current_term: state => {
      const in_map = state.raw_terms.get(state.current_term_uid);
      console.log(state.raw_terms);
      console.log(state.current_term_uid);
      // const term:Term = in_map ? in_map :  new Term();
      // state.current_term_uid = term.uid;
      // console.log(state.current_term_uid);
      // state.raw_terms.set(term.uid, term);
      return in_map;
    },
  },
  actions: {
    async CREAT_ONE(sessionName: string): Promise<Term> {
      console.log("create one: " + sessionName);
      const sess = await invoke<OmitSession>('read_session', { sessionName });
      const term = new Term(sess);
      this.current_term_uid = term.uid;
      this.raw_terms.set(term.uid, term);
      return term;
    },
  },
});
export default useStore;
