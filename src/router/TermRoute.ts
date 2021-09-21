import { RouteRecordRaw } from 'vue-router';
import SSH from '../views/SH.vue';

export default [
  {
    path: '/ssh',
    name: 'ssh',
    component: SSH,
  },
] as Array<RouteRecordRaw>;
