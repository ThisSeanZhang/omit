import { RouteRecordRaw } from 'vue-router';
import Manage from '../views/ManageSession.vue';

export default [
  {
    path: '/cmd',
    name: 'cmd',
    component: Manage,
  },
] as Array<RouteRecordRaw>;
