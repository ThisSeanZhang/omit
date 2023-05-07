import { defineStore } from 'pinia';
import { ref, computed, ComputedRef } from 'vue';
const dict: any = {
  init: {
    welcome: {
      cn: '欢迎',
      en: 'Welcome'
    },
    welcomeDescription: {
      cn: '欢迎使用',
      en: 'Welcome to use'
    },
    storeLocation: {
      cn: '存放位置',
      en: 'Store location'
    },
    storeDescription: {
      cn: '指定运行时: 命令, 快照, 快捷输入. 存放位置',
      en: 'Specify runtime: command, snapshot, shortcut input. Storage location'
    },
    finish: {
      cn: '完成',
      en: 'Finish'
    },
    startConfigBtn: {
      cn: '开始配置',
      en: 'Start config'
    },
    confirmConfig: {
      cn: '确认配置',
      en: 'Confirm config'
    },
    dataLocation: {
      cn: '数据路径',
      en: 'data store location'
    },
    sessionLocation: {
      cn: '会话存放路径',
      en: 'Session store location'
    },
    snapLocation: {
      cn: '快照存放路径',
      en: 'Snapshot store location'
    },
    repoLocation: {
      cn: '公共仓库存放路径',
      en: 'Public Command location'
    },
    userRepoName: {
      cn: '私有仓库文件夹名',
      en: 'User Self Data location'
    },
    backBtn: {
      cn: '返回',
      en: 'Back'
    },
    confirmBtn: {
      cn: '确认',
      en: 'Confirm'
    }
  },
  session: {
    filter: {
      cn: '筛选会话',
      en: 'Filter The Session'
    },
    connectBtn: {
      cn: '连接',
      en: 'Connect it'
    },
    deleteConfirm: {
      cn: '确定删除吗',
      en: 'Confirm Delete'
    },
    name: {
      cn: '会话名称',
      en: 'Session Name'
    },
    ip: {
      cn: '主机名 / IP 地址',
      en: 'Host Name / IP Address'
    },
    port: {
      cn: '端口',
      en: 'Port'
    },
    username: {
      cn: '用户名',
      en: 'User Name'
    },
    passwd: {
      cn: '密码',
      en: 'Password'
    },
    save: {
      cn: '保存',
      en: 'Save'
    }
  },
  terminal: {
    tryAnother: {
      cn: '试个别的 ?',
      en: 'Try Another?'
    },
    tryAgain: {
      cn: '或者再试一次 ?',
      en: 'OR Try Again?'
    }
  },
  snap: {
    filter: {
      cn: '过滤快照',
      en: 'Filter Snapshot'
    },
    title: {
      cn: '指令预览',
      en: 'Command Preview'
    },
    singleLine: {
      cn: '单行模式',
      en: 'Single Line'
    },
    multipleLine: {
      cn: '多行模式',
      en: 'Multiple Line'
    },
    send: {
      cn: '发送',
      en: 'Send'
    },
    copy: {
      cn: '复制',
      en: 'Copy'
    },
    re_edit: {
      cn: '重新编辑',
      en: 'Re-Edit'
    },
    delete: {
      cn: '删除',
      en: 'Delete'
    },
    more: {
      cn: '更多',
      en: 'More'
    },
    create_snap: {
      cn: '创建快照',
      en: 'Create Snapshot'
    },
    empty: {
      cn: '空空如也',
      en: 'Empty'
    },
    emptyDescription: {
      cn: '在上面的选择框选个命令吧',
      en: 'Select a command in the checkbox above'
    },
    option: {
      cn: '可选项',
      en: 'Option'
    },
    optionEdit: {
      cn: '编辑可选项',
      en: 'Edit Option',
    },
    edit: {
      cn: '编辑',
      en: 'Edit',
    },
    param: {
      cn: '参数',
      en: 'Param',
    },
    paramEdit: {
      cn: '编辑参数',
      en: 'Edit Option',
    },
    paramAdd: {
      cn: '搞点参数',
      en: 'Let\'s go',
    },
    save_title: {
      cn: '保存快照',
      en: 'Save Snapshot',
    },
    snap_title: {
      cn: '快照名称',
      en: 'Snapshot Title',
    },
    save: {
      cn: '保存快照',
      en: 'Save Snapshot',
    }
  },
  param: {
    editParam: {
      cn: '选项编辑',
      en: 'Edit Params',
    },
    addParam: {
      cn: '搞点快捷按钮',
      en: 'Add Params',
    }
  }
}
export const i18nStore = defineStore('i18n', () => {

  const raw_laguage = ref('en');
  const language = computed(() => raw_laguage.value);

  function TRANSLATE(info: string): string {
    const arr = info.split('.');
    arr.push(raw_laguage.value);
    return arr.reduce((o, i) => {
      if (o) return o[i]
    }, dict);
  }

  function SET_LAN(lang: string) {
    raw_laguage.value = lang;
  }

  return {
    language,
    TRANSLATE,
    SET_LAN,
  };
})

export default i18nStore;