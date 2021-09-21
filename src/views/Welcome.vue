<template>
<div class="welcome-container">
  {{currentSSHInfo}}
  <input type="text" v-model="currentSSHInfo.ip">
  <input type="text" v-model="currentSSHInfo.port">
  <input type="text" v-model="currentSSHInfo.username">
  <input type="text" v-model="currentSSHInfo.passwd">
  <input type="button" value="连接" @click="connect">
</div>
</template>
<script lang="ts">
import { Vue } from 'vue-class-component';
import SSHInfo from '@/lib/SSInfo';

export default class Welcome extends Vue {
  public myModel = {
    myDate: '123456',
  }

  public currentSSHInfo: SSHInfo = new SSHInfo();

  connect() {
    this.$router.push('/ssh');
  }
}
</script>
<style lang="scss" scoped>
.welcome-container {
  width: 100%;
  height: 100%;
}
</style>
