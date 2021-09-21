<template>
  <Title/>
  path:{{message}}
  <router-view />
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
import { invoke } from '@tauri-apps/api/tauri';
import Title from '@/views/Title.vue';

@Options({
  components: {
    Title,
  },
})
export default class App extends Vue {
  public message = '';

  created():void {
    invoke<string>('current_path')
      .then(value => {
        this.message = value;
      });
  }
}
</script>
<style lang="scss">
html, body , #app {
  margin: 0px;
  // overflow: hidden;
  width: 100%;
  height: 100%;
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
