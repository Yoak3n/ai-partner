<script lang="ts" setup>
import { ref,onMounted, onBeforeUnmount } from 'vue'
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window'
import type {Window }from '@tauri-apps/api/window'
import DialogItem from '../components/Chat/ChatDialog/index.vue'
import emitter from '../bus'
import { debounce } from '../utils'

const dialogWrapper = ref<HTMLElement | null>(null)
let appWindow :Window;
const debouncedResizeWindow = debounce(async (height) => {
  const currentSize = await appWindow.innerSize();
  const targetHeight = Math.min(Math.max(height, 100), 800);
  if (Math.abs(currentSize.height - targetHeight) > 50) {
    const steps = 10; // 动画步数
    const stepDelay = 10; // 每步延迟(ms)
    const heightDiff = targetHeight - currentSize.height;
    const stepHeight = heightDiff / steps;
    
    for (let i = 1; i <= steps; i++) {
      const newHeight = currentSize.height + stepHeight * i;
      await appWindow.setSize(new PhysicalSize(800, Math.round(newHeight)));
      await new Promise(resolve => setTimeout(resolve, stepDelay));
    }
  }else{
    await appWindow.setSize(new PhysicalSize(800, Math.min(Math.max(height, 100), 800)))
  }
}, 100) // 100ms的防抖时间
onMounted(async() => {
  appWindow = getCurrentWindow()
  if (dialogWrapper.value) {
    resizeObserver.observe(dialogWrapper.value)
  }
  emitter.on('message-cleared', () => {
    debouncedResizeWindow(100);
  })
})
const resizeObserver = new ResizeObserver(async (entries) => {
  for (const entry of entries) {
    const dialogContent = entry.target.querySelector('.dialog-content')
    const messageContainer = entry.target.querySelector('.message-container')
    if (!dialogContent || !messageContainer) return
    const contentHeight = dialogContent.getBoundingClientRect().height
    const messageHeight = messageContainer.scrollHeight
    const headerHeight = 28  // 头部高度
    const totalHeight = messageHeight + contentHeight + headerHeight + 10
    debouncedResizeWindow(totalHeight) 
    
  }
})
onBeforeUnmount(async() => {
  resizeObserver.disconnect()
  debouncedResizeWindow(100)
  emitter.off('message-cleared')
})



</script>
<template>
    <div class="dialog-wrapper" ref="dialogWrapper" :key="'dialog-wrapper'">
        <DialogItem />
    </div>
</template>
<style>
html {
  overflow-y: hidden;
}
body{
    height: 100%;
}
.dialog-wrapper{
    overflow-y: hidden;
    transition: height 0.3s ease-out;
}
</style>