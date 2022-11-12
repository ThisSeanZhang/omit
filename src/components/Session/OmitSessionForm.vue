<template>
  <div class="omit_form">
    <n-form
      label-placement="top"
      :label-width="80"
      :model="omitSession"
      ref="formRef"
    >
      <n-grid :cols="24" :x-gap="24">
        <n-form-item-gi :span="24" :label="i18n.TRANSLATE('session.name')" path="session.name">
          <n-input :placeholder="i18n.TRANSLATE('session.name')" v-model:value="omitSession.name" />
        </n-form-item-gi>
        <n-form-item-gi :span="20" :label="i18n.TRANSLATE('session.ip')" path="session.ip">
          <n-input :placeholder="i18n.TRANSLATE('session.ip')" v-model:value="omitSession.ip" />
        </n-form-item-gi>
        <n-form-item-gi :span="4" :label="i18n.TRANSLATE('session.port')" path="session.port">
          <n-input-number :show-button="false"
            :placeholder="i18n.TRANSLATE('session.port')"
            v-model:value="omitSession.port"
          />
        </n-form-item-gi>
        <n-form-item-gi :span="24" :label="i18n.TRANSLATE('session.username')" path="session.username">
          <n-input :placeholder="i18n.TRANSLATE('session.username')" v-model:value="omitSession.username" />
        </n-form-item-gi>
        <n-form-item-gi :span="24" :label="i18n.TRANSLATE('session.passwd')" path="session.passwd">
          <n-input
            :placeholder="i18n.TRANSLATE('session.passwd')"
            type="password"
            show-password-on="mousedown"
            v-model:value="omitSession.passwd"
          />
        </n-form-item-gi>
        <n-gi :span="24">
          <div style="display: flex; justify-content: flex-end;">
            <n-button @click="save" round type="primary" >{{i18n.TRANSLATE('session.save')}}</n-button>
          </div>
        </n-gi>
      </n-grid>
    </n-form>
  </div>
</template>
<script setup lang="ts">
import {
  defineComponent,
  ref,
  watch,
} from 'vue';
import { sessionStore } from '@/store/session';
import { useRouter, useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/tauri';
import OmitSession from '@/lib/OmitSession';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
const store = sessionStore();
const props = defineProps({
  session: OmitSession,
})
const emit = defineEmits<{
  (e: 'edit_finish'): void
}>()

const omitSession = ref<OmitSession>(new OmitSession());
watch(() => props.session, (_) => {
  if (props.session !== undefined)
  omitSession.value = props.session;
})
function save(): void {
  store.SAVE_SESSION(omitSession.value);
  emit('edit_finish');
}
</script>
<style lang="scss" scoped>
.omit_form {
  padding: 20px;
}
</style>
