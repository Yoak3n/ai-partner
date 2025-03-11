<script setup lang="ts">
import { defineComponent, h, onMounted, onUnmounted } from 'vue'
import {
  NDialogProvider,
  NLoadingBarProvider,
  NMessageProvider,
  NNotificationProvider,
  NModalProvider,
  useDialog,
  useLoadingBar,
  useMessage,
  useNotification,
} from 'naive-ui'

function registerNaiveTools() {
  window.$loadingBar = useLoadingBar()
  window.$dialog = useDialog()
  window.$message = useMessage()
  window.$notification = useNotification()
}
const NaiveProviderContent = defineComponent({
  name: 'NaiveProviderContent',
  setup() {
    registerNaiveTools()
    const disableRightClick = (e: MouseEvent) => {
      e.preventDefault();
    };

    // 在组件挂载时添加事件监听器
    onMounted(async() => {
      document.addEventListener('contextmenu', disableRightClick);
    });

    // 在组件卸载时移除事件监听器
    onUnmounted(() => {
      document.removeEventListener('contextmenu', disableRightClick);
    });
    },
  render() {
    return h('div')
  },
})
</script>

<template>
  <NLoadingBarProvider>
    <NDialogProvider>
      <NNotificationProvider>
        <NModalProvider>
        <NMessageProvider placement="bottom">
          <slot />
          <NaiveProviderContent />
        </NMessageProvider>
        </NModalProvider>
      </NNotificationProvider>
    </NDialogProvider>
  </NLoadingBarProvider>
</template>