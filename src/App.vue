<template>
  <Title class="title" />
  <router-view class="app_contents" />
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
// import { invoke } from '@tauri-apps/api/tauri';
import Title from '@/views/Title.vue';

@Options({
  components: {
    Title,
  },
})
export default class App extends Vue {
  // public message = '';

  // created():void {
  //   invoke<string>('current_path')
  //     .then(value => {
  //       this.message = value;
  //     });
  // }
}
</script>
<style lang="scss">
#app {
  background-color: rgba(91, 190, 195, 0.5);
}
html, body , #app {
  // background: inherit;
  margin: 0px;
  overflow: hidden;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
.title {
  height: 30px;
}
.app_contents {
  flex: 1;
  box-shadow: 0 0 1rem 0 rgba(0, 0, 0, .2);
  background-color: rgba(255, 255, 255, .15);
  backdrop-filter: blur(5px);
}
/*控制整个滚动条*/
::-webkit-scrollbar {
    background-color: lightgray;
    width: 10px;
    height: 10px;
    background-clip: padding-box;
}
/*滚动条两端方向按钮*/
::-webkit-scrollbar-button {
    background-color: pink;
}
/*滚动条中间滑动部分*/
::-webkit-scrollbar-thumb {
    background-color: blue;
    border-radius: 5px;
}
/*滚动条右下角区域*/
::-webkit-scrollbar-corner {
    background-color: red;
}
</style>
