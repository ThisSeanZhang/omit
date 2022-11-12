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
  },
  session: {
    filter: {
      cn: '筛选会话'
    },
    connectBtn: {
      cn: '连接'
    },
    deleteConfirm: {
      cn: '确定删除吗'
    },
    name: {
      cn: '会话名称'
    },
    ip: {
      cn: '地址'
    },
    port: {
      cn: '端口'
    },
    username: {
      cn: '用户名'
    },
    passwd: {
      cn: '密码'
    },
    save: {
      cn: '保存'
    }
  },
  terminal: {
    tryAnother: {
      cn: '试个别的 ?'
    },
    tryAgain: {
      cn: '或者再试一次 ?'
    }
  },
  snap: {
    title: {
      cn: '指令预览'
    },
    singleLine: {},
    multipleLine: {},
    send: {},
    copy: {},
    re_edit: {

    },
    delete: {},
    more: {},
    create_snap: {},
    
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