import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import TerminalWorkView from '@/views/TerminalWorkView.vue';
import SnapshotManageView from '@/views/SnapshotManageView.vue';
import CommandCreateView from '@/views/CommandCreateView.vue';
import CommandManageView from '@/views/CommandManageView.vue';
import InitView from '@/views/InitView.vue';
import Welcome from '@/views/Welcome.vue';
// import TermRoute from './TermRoute';
import CmdRoute from './CmdRoute';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/terminal/:sessionName?',
    name: 'TerminalWorkView',
    component: TerminalWorkView,
  }, {
    path: '/snap-manage',
    name: 'SnapshotManageView',
    component: SnapshotManageView,
  }, {
    // path: '/cmd-manage',
    path: '/CommandCreateView',
    name: 'CommandCreateView',
    component: CommandCreateView,
  }, {
    path: '/CommandManageView',
    name: 'CommandManageView',
    component: CommandManageView,
  },
  {
    path: '/InitView',
    name: 'InitView',
    component: InitView,
  },
  {
    path: '/:sessionName?',
    name: 'Welcome',
    component: Welcome,
  },
];

// routes.push(...TermRoute);
routes.push(...CmdRoute);

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
