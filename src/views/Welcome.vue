<template>
<div>
  <div class="welcome-container">
    {{currentSSHInfo}}
    <input type="text" v-model="currentSSHInfo.name">
    <input type="text" v-model="currentSSHInfo.ip">
    <input type="text" v-model="currentSSHInfo.port">
    <input type="text" v-model="currentSSHInfo.username">
    <input type="text" v-model="currentSSHInfo.passwd">
    <input type="button" value="插入" @click="save">
    <div v-for="sess in omitSessions" v-bind:key="sess">
      <input type="button" :value="'连接' + sess" @click="connect(sess)">
    </div>
  </div>
</div>
</template>
<script lang="ts">
import { Vue } from 'vue-class-component';
import { invoke } from '@tauri-apps/api/tauri';
import SSHInfo from '@/lib/SSInfo';

export default class Welcome extends Vue {
  public myModel = {
    myDate: '123456',
  }

  public omitSessions = new Array<string>();

  public currentSSHInfo: SSHInfo = new SSHInfo();
  created(): void {
    this.flashSessions();
  }

  save(): void {
    invoke<string>('save_session', { sess: this.currentSSHInfo })
      .then(msg => {
        this.flashSessions();
        console.log(msg);
      }).catch((msg:string) => console.log(msg));
  }

  flashSessions(): void {
    invoke<string[]>('sessions').then(sess => {
      console.log(sess);
      this.omitSessions = sess;
    }).catch((e:string) => console.log(e));
  }
  connect(sessionName: string): void {
    this.$router.push({ name: 'ssh', params: { sessionName } });
  }
}
</script>
<style lang="scss" scoped>
.welcome-container {
  width: 100%;
  height: 100%;
}
</style>
