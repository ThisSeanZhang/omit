import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Term from '@/lib/Term';

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
      const term:Term = in_map === undefined ? new Term() : in_map;
      state.current_term_uid = term.uid;
      state.raw_terms.set(term.uid, term);
      return term;
    },
  },
  actions: {
    CREAT_ONE() {
      const term = new Term();
      this.current_term_uid = term.uid;
      this.raw_terms.set(term.uid, term);
    },
  },
});
export default useStore;
