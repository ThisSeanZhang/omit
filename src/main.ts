import { createApp } from 'vue';
import {
  // create naive ui
  create,
  // component
  NButton,
  NConfigProvider,
  NSpace,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NLayoutFooter,
  NLayoutSider,
  NIcon,
  NCard,
  NInput,
  NH2,
  NForm,
  NFormItem,
  NFormItemGi,
  NGrid,
  NGi,
  NPopconfirm,
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
    NConfigProvider,
    NSpace,
    NLayout,
    NLayoutHeader,
    NLayoutContent,
    NLayoutFooter,
    NLayoutSider,
    NIcon,
    NCard,
    NInput,
    NH2,
    NForm,
    NFormItem,
    NFormItemGi,
    NGrid,
    NGi,
    NPopconfirm,
  ],
});

createApp(App).use(naive).use(store).use(router)
  .mount('#app');
