<template>
  <div class="session_card" @click="reviewOmitSessionConfig">
    <div>{{value}}</div>
    <div class="card_btns">
      <n-button class="card_btn" text @click.stop="connect">
        <n-icon>
         <PlugDisconnected20Regular />
        </n-icon>
      </n-button>

      <n-popconfirm
        @positive-click="clickOK"
        @negative-click="clickNo"
        positive-text="NO!!!" negative-text="YES"
      >
        <!-- <template #icon>
          <n-icon color="red">
            <hand-icon />
          </n-icon>
        </template> -->
        <template #trigger>
          <n-button class="card_btn" @click.stop text>
            <n-icon>
            <Delete20Regular />
            </n-icon>
          </n-button>
        </template>
        确定删除吗
      </n-popconfirm>

    </div>
  </div>
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
  computed,
} from 'vue';
import { useRouter, useRoute } from 'vue-router';
import {
  Delete20Regular,
  PlugDisconnected20Regular,
} from '@vicons/fluent';
// import SSHInfo from '@/lib/SSInfo';

export default defineComponent({
  name: 'OmitSessions',
  components: {
    Delete20Regular,
    PlugDisconnected20Regular,
  },
  props: {
    value: {
      type: String,
      require: true,
      default: () => '',
    },
  },
  setup(props: { value: string; }) {
    const router = useRouter();
    function connect(): void {
      router.push({ name: 'TerminalWorkView', params: { sessionName: props.value } });
    }

    function reviewOmitSessionConfig():void {
      console.log(`want to push session: ${props.value}`);
      router.push({ name: 'Welcome', params: { sessionName: props.value } });
    }

    function clickOK(): void {
      console.log('ok');
    }

    function clickNo(): void {
      console.log('no');
    }
    return {
      connect,
      reviewOmitSessionConfig,
      clickOK,
      clickNo,
    };
  },
});
</script>

<style lang="scss" scoped>
.session_card{
  border-radius: 3px;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 10px 10px;
  margin: 10px 5px;
}
.session_card:hover {
  box-shadow: inset 0px 0px 11px 3px rgba(255, 255, 255, 0.1);
  // box-shadow: 0px 0px 11px 3px rgba(255, 255, 255, 0.1);
}
.card_btn {
  font-size: 20px;
  padding: 3px;
}
.card_btns {
  flex-direction: row;
  display: flex;
  align-items: center;
}
</style>
