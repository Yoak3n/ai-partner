<template>
  <div class="app-about" :class="{ 'dark-mode': darkMode }">
    <n-tabs type="segment">
      <n-tab-pane name="chap1" tab="使用说明">
        <div>
          安装后开始对话前需要先
          <b>配置api相关信息:</b>
          <br />
          在设置中找到api配置，填入api地址、api密钥以及调用的模型名即可开始对话。
        </div>
        <n-divider></n-divider>
        <div>
          快速（临时）对话框:
          <br />
          回车键发送消息，循环按钮可停止并清空当前对话
          <br />
          对话框顶可拖动
          <br />

        </div>
      </n-tab-pane>
      <n-tab-pane name="chap3" tab="版本信息" display-directive="show:lazy">
        <header class="brand-header">
          <img src="../../../src-tauri/icons/icon.png" alt="App Logo" class="app-logo" />
          <div class="title-group">
            <h1>{{ appInfo.name }}</h1>
            <p class="version-info">
              Version {{ appInfo.version }}
            </p>
          </div>
        </header>

        <!-- 基础信息卡片 -->
        <div class="info-card">
          <h2><i class="icon-info"></i> 基本信息</h2>
          <div class="info-grid">
            <div class="info-item">
              <label>项目地址：</label>
              <span><a href="https://github.com/Yoak3n/ai-partner" target="_blank"
                  rel="noopener noreferrer">ai-partner</a></span>
            </div>
            <div class="info-item">
              <label>编译日期：</label>
              <span>{{ appInfo.buildDate }}</span>
            </div>
            <div class="info-item">
              <label>开发框架：</label>
              <span>
                <a href="https://tauri.app/" target="_blank" rel="noopener noreferrer">
                  <img src="../../assets/tauri.svg" alt="" height="15">
                  tauri {{ appInfo.tauriVersion }}
                </a>
                /
                <a href="https://vuejs.org/" target="_blank" rel="noopener noreferrer">
                  <img src="../../assets/vue.svg" alt="" height="15">
                  Vue {{ version }}
                </a>
              </span>
            </div>
          </div>
        </div>
        <div class="info-card">
          <h2><i class="icon-info"></i>开发者信息</h2>
          <div class="info-grid">

            <div class="info-item">
              <label>
                <n-avatar src="https://avatars.githubusercontent.com/u/120039624?v=4">
                </n-avatar>
              </label>
              <span>
                <a href="https://github.com/Yoak3n" target="_blank" rel="noopener noreferrer">Yoake</a>
              </span>
            </div>
            <div class="info-item">
              <label>网站：</label>
              <span><a href="https://www.yoake.top" target="_blank" rel="noopener noreferrer">Mou1ght</a></span>
            </div>
            <div>
              一个喜欢摸鱼偷闲时写点东西的人
            </div>
          </div>
        </div>
        <!-- 操作按钮组 -->
        <div class="action-buttons">
          <n-badge dot :show="needUpdate">
            <button class="btn" @click="getLatesetVersion">
              <i class="icon-update"></i> 检查更新
            </button>
          </n-badge>
          <button class="btn" @click="openLogsFolder">
            <i class="icon-folder"></i> 打开所在目录
          </button>
          <button class="btn">
            <i class="icon-copy"></i> 复制诊断信息
          </button>
        </div>
      </n-tab-pane>
    </n-tabs>

  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, version, onBeforeUnmount, h } from 'vue';
import { NTabs, NTabPane, NDivider, NAvatar, NBadge } from 'naive-ui';
import type { MessageReactive } from 'naive-ui'
import { invoke, Channel } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell'

import { AppInfo, getAppInfo, VersionComparation, DownloadEvent } from '../composables';
import emitter from '../../bus';
import Updater from './Updater.vue';

const darkMode = ref(false);
let needUpdate = ref(false);
let messageReactive: MessageReactive | null = null
const content_size = ref(0)
const downloaded_size = ref(0)
let appInfo = reactive<AppInfo>({ version: '', name: '', buildDate: '', buildNumber: '', configPath: '', logo: '', tauriVersion: '', });

onMounted(async () => {
  const newInfo = await getAppInfo()
  Object.assign(appInfo, newInfo)
  const res = await fetchUpdate()
  if (res != null && res.version != res.current_version) {
    needUpdate.value = true
    createUpdateDialog(res)
  }
});
onBeforeUnmount(() => removeMessage());

const openLogsFolder = async () => {
  const pwd = await invoke('get_app_install_path')
  open(pwd as string)
}
const fetchUpdate = async (): Promise<VersionComparation | null> => {
  try {
    const res: VersionComparation = await invoke('fetch_update')
    return res
  } catch (e) { return null }

}
async function install_update() {
  const onEvent = new Channel<DownloadEvent>();
  onEvent.onmessage = (res) => {
    if (res.event == 'Started') {
      content_size.value = res.data.contentLength
    } else if (res.event == 'Progress') {
      downloaded_size.value += res.data.chunkLength
    } else {
      downloaded_size.value = content_size.value
      window.$message.success('更新成功')
    }
  }
  try {
    await invoke('install_update', { onEvent });
  } catch (error) {
    window.$message.error('安装更新失败:' + error);
  }
}

const createUpdateDialog = async (version: VersionComparation) => {
  emitter.on('install_update', install_update)
  window.$dialog.info({
    content: () => h(Updater, { version: version, content: content_size.value, downloaded: downloaded_size.value }),
    showIcon: false,
    closable: false,
    autoFocus: false,
    onClose: () => {
      emitter.off('install_update', install_update)
    },
  });
}

const getLatesetVersion = async () => {
  if (!messageReactive) { messageReactive = window.$message.loading('正在检查更新...', { duration: 0 }); }
  const res = await fetchUpdate()
  removeMessage()
  if (res != null && res.current_version != res.version) {
    needUpdate.value = true
    createUpdateDialog(res)
  } else {
    window.$message.info('当前已是最新版本')
  }
}

const removeMessage = () => {
  if (messageReactive) {
    messageReactive.destroy();
    messageReactive = null;
  }
};


</script>

<style lang="less">
.app-about {
  max-width: 800px;
  margin: 0 auto;
  background: var(--bg-color);
  color: var(--text-color);
  max-height: 80%;
}

.brand-header {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 0rem;
}

.app-logo {
  width: 64px;
  height: 64px;
}

.title-group h1 {
  margin: 0;
  font-size: 1.8rem;
}

.version-info {
  color: var(--secondary-text);
  margin: 0.25rem 0 0;
}

.info-cards {
  display: grid;
  gap: 1.5rem;
}

.info-card {
  background: var(--card-bg);
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  margin-top: 1rem;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-grid {
  display: grid;
  gap: 1rem;
}

.status-item progress {
  width: 100%;
  height: 6px;
}

.dependency-list {
  display: grid;
  gap: 0.5rem;
}

.dependency-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.5rem;
  border-radius: 4px;
  background: var(--item-bg);
}

.action-buttons {
  margin-top: 2rem;
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--button-bg);
  color: var(--button-text);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* 深色模式变量 */
.dark-mode {
  --bg-color: #1a1a1a;
  --text-color: #ffffff;
  --secondary-text: #a0a0a0;
  --card-bg: #2d2d2d;
  --item-bg: #3a3a3a;
  --border-color: #4a4a4a;
  --button-bg: #3a3a3a;
}

.update-dialog-action {
  .n-button {
    &.n-button--default-type {
      color: #000;
      --n-border-hover: none !important;

    }

    &.n-button--info-type {
      &:hover {
        color: #fff;
      }

    }
  }
}
</style>