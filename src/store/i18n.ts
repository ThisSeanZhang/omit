import { defineStore } from 'pinia';
import { ref } from 'vue';
const dict: any = {
  init: {
    welcome: {
      cn: '欢迎',
      en: 'Welcome'
    },
    welcomeDescription: {
      cn: '欢迎使用'
    },
    storeLocation: {
      cn: '存放位置'
    },
    storeDescription: {
      cn: '指定运行时: 命令, 快照, 快捷输入. 存放位置'
    },
    finish: {
      cn: '完成'
    },
    startConfigBtn: {
      cn: '开始配置'
    },
    confirmConfig: {
      cn: '确认配置'
    },
    sessionLocation: {
      cn: '会话存放路径'
    },
    repoLocation: {
      cn: '公共仓库存放路径'
    },
    userRepoName: {
      cn: '私有仓库文件夹名'
    },
    backBtn: {
      cn: '返回'
    },
    confirmBtn: {
      cn: '确认'
    }
  }
}
export const i18nStore = defineStore('i18n', () => {

  const laguage = ref('cn');

  function TRANSLATE(info: string): string {
    const arr = info.split('.');
    arr.push(laguage.value);
    return arr.reduce((o, i) => {
      if (o) return o[i]
    }, dict);
  }

  return {
    TRANSLATE
  };
})

export default i18nStore;