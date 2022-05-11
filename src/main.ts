import { createApp } from 'vue';
import {
  // create naive ui
  create,
  // component
  NButton,
  NButtonGroup,
  NConfigProvider,
  NMessageProvider,
  NSpace,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NLayoutFooter,
  NLayoutSider,
  NIcon,
  NCard,
  NInput,
  NInputNumber,
  NInputGroup,
  NInputGroupLabel,
  NSelect,
  NH2,
  NForm,
  NFormItem,
  NFormItemGi,
  NGrid,
  NGi,
  NPopconfirm,
  NThing,
  NPopover,
  NScrollbar,
  NList,
  NListItem,
  NTag,
  NAvatar,
  NDrawer,
  NDrawerContent,
  NModal,
  NEllipsis,
  NResult,
} from 'naive-ui';
import App from './App.vue';
import router from './router';
import store from './store';
// 通用字体
import 'vfonts/Lato.css';
// 等宽字体
import 'vfonts/FiraCode.css';

const naive = create({
  components: [
    NButton,
    NButtonGroup,
    NConfigProvider,
    NMessageProvider,
    NSpace,
    NLayout,
    NLayoutHeader,
    NLayoutContent,
    NLayoutFooter,
    NLayoutSider,
    NIcon,
    NCard,
    NInput,
    NInputNumber,
    NInputGroup,
    NInputGroupLabel,
    NSelect,
    NH2,
    NForm,
    NFormItem,
    NFormItemGi,
    NGrid,
    NGi,
    NPopconfirm,
    NThing,
    NPopover,
    NScrollbar,
    NList,
    NListItem,
    NTag,
    NAvatar,
    NDrawer,
    NDrawerContent,
    NModal,
    NEllipsis,
    NResult,
  ],
});

createApp(App)
  .use(store)
  .use(naive)
  .use(router)
  .mount('#app');
