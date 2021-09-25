<template>
  <div class="omit_form">
    <n-form
      label-placement="top"
      :label-width="80"
      :model="omitSession"
      ref="formRef"
    >
      <n-grid :cols="24" :x-gap="24">
        <n-form-item-gi :span="24" label="会话标签" path="session.name">
          <n-input placeholder="会话标签" v-model:value="omitSession.name" />
        </n-form-item-gi>
        <n-form-item-gi :span="20" label="IP" path="session.ip">
          <n-input placeholder="IP地址" v-model:value="omitSession.ip" />
        </n-form-item-gi>
        <n-form-item-gi :span="4" label="端口号" path="session.port">
          <n-input placeholder="端口号" v-model:value="omitSession.port" />
        </n-form-item-gi>
        <n-form-item-gi :span="24" label="用户名" path="session.username">
          <n-input placeholder="用户名" v-model:value="omitSession.username" />
        </n-form-item-gi>
        <n-form-item-gi :span="24" label="密码" path="session.passwd">
          <n-input
            placeholder="密码"
            type="password"
            show-password-on="mousedown"
            v-model:value="omitSession.passwd"
          />
        </n-form-item-gi>
        <n-gi :span="24">
          <div style="display: flex; justify-content: flex-end;">
            <n-button @click="save" round type="primary" >保存</n-button>
          </div>
        </n-gi>
      </n-grid>
    </n-form>
  </div>
</template>
<script lang="ts">
import { Vue } from 'vue-class-component';
import { invoke } from '@tauri-apps/api/tauri';
import OmitSession from '@/lib/OmitSession';

export default class OmitSessions extends Vue {
  public omitSession: OmitSession = new OmitSession();

  save(): void {
    invoke<string>('save_session', { sess: this.omitSession })
      .then(msg => {
        this.$emit('session_save_done');
        console.log(msg);
      }).catch((msg:string) => console.log(msg));
  }
}
</script>
<style lang="scss" scoped>
.omit_form {
  padding: 20px;
}
</style>
