<script lang="ts" setup>
import { nextTick, onMounted, onUnmounted, ref } from 'vue'
import { NIcon } from 'naive-ui'
import { InfiniteSharp } from '@vicons/ionicons5'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { UnlistenFn, listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

import { debounce, throttle } from '../../../utils'
import { MessageItem, Payload } from '../../../types'
import emitter from '../../../bus'
import { registerNewListen,getUnlistenFnAndOff } from '../../../bus'
import MarkdownRender from '../../../components/Markdown/index.vue'

const messageContainer = ref<HTMLElement|null>(null);
const userScrolling = ref(false);
const scrollTimeout = ref<number | null>(null);
const handleScroll = () => {
  if (!messageContainer.value) return;
  
  // 检测是否滚动到底部
  const isAtBottom = messageContainer.value.scrollHeight - messageContainer.value.scrollTop <= messageContainer.value.clientHeight + 50;
  
  // 如果不在底部，设置用户正在滚动状态
  if (!isAtBottom) {
    userScrolling.value = true;
    
    // 清除之前的定时器
    if (scrollTimeout.value !== null) {
      clearTimeout(scrollTimeout.value);
    }
    
    // 设置新的定时器，3秒后恢复自动滚动
    scrollTimeout.value = window.setTimeout(() => {
      userScrolling.value = false;
      scrollTimeout.value = null;
    }, 3000);
  } else {
    // 如果已经在底部，重置用户滚动状态
    userScrolling.value = false;
    if (scrollTimeout.value !== null) {
      clearTimeout(scrollTimeout.value);
      scrollTimeout.value = null;
    }
  }
};
const scrollToBottom = () => {
  if (messageContainer.value) {
    if (!userScrolling.value){
      messageContainer.value.scrollTop = messageContainer.value.scrollHeight;
    }
  }
};
const throttleEmitScrollToBottom = throttle(() => scrollToBottom(), 500)
let message = ref('')
let dragging = ref(false)
const submittMessage = () => {
  makeMessage(message.value)
  recircle()
}
let timeId = ref(0);
const defaultMessage = [{ role: 'assistant', content: '', reasoning_content: '' }]
let messageUpdate = ref<MessageItem>(defaultMessage[0])
const generating = ref(false)
const makeMessage = async (question: string) => {
  timeId.value = Date.now()
  const messageItems: Array<MessageItem> = [{ role: 'user', content: question, reasoning_content: '', timestamp: timeId.value - 5 },]
  setTimeout(()=>message.value = '',1000)
  const unlistenFnData = await listen('stream-data', (event) => {
    const payload = event.payload as Payload
    if (payload.id != timeId.value) return
    generating.value = true
    if (payload.message_type == 'reasoning_content') {
      messageUpdate.value!.reasoning_content += payload.data
    } else if (payload.message_type == 'content') {
      messageUpdate.value!.content += payload.data
    } else {
      unlistenFnData()
      generating.value = false
    }
    nextTick(() => {
      throttleEmitScrollToBottom()
    })
  })
  registerNewListen(timeId.value, unlistenFnData)
  invoke('completions_stream', { messages: messageItems, id: timeId.value }).catch((e) => {
    window.$message.error(e.toString(), { duration: 3000 })
  })
}
const switchState = debounce(() => { dragging.value = false }, 300)
let unlisten: UnlistenFn;
onMounted(async () => {
  emitter.emit('message-cleared')
  unlisten = await getCurrentWindow().listen('tauri://move', () => {
    switchState()
  })
  if (messageContainer.value) {
    messageContainer.value.addEventListener('scroll', handleScroll, { passive: true });
  }
})
onUnmounted(() => {
  unlisten()
})
const recircle = () => {
  emitter.emit('message-cleared')
  getUnlistenFnAndOff(timeId.value)
  generating.value = false
  timeId.value = 0
  setTimeout(() => {
    messageUpdate.value = { ...defaultMessage[0], content: '', reasoning_content: '',timestamp:timeId.value }
    emitter.emit('message-cleared')
  }, 300)
}

</script>


<template>
  <div class="dialog">
    <div class="dialog-bg"></div>
    <div class="dialog-header" data-tauri-drag-region :class="dragging ? 'dragging' : ''" @mousedown="dragging = true">
    </div>


    <div class="dialog-content">
      <textarea class="dialog-textarea" :placeholder="generating ? '生成中...' : '输入你的问题'" rows="4" cols="50" v-model="message"
        @keydown="(e) => {
          if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); submittMessage() }
        }">
      </textarea>
      <button class="btn" @click="recircle">
        <n-icon>
          <InfiniteSharp />
        </n-icon>
      </button>
    </div>
    <div class="message-container" 
      ref="messageContainer"
      v-show="messageUpdate.reasoning_content !== '' || messageUpdate.content !== '' ">
      <div class="output">
        <div class="message-item"  v-if="messageUpdate.reasoning_content !== ''">
          <div class="reasoning">{{ messageUpdate.reasoning_content }}</div>
        </div>
        <div class="message-item" >
          <div class="content" v-if="messageUpdate.content !== ''">
            <MarkdownRender :source="messageUpdate.content" />
          </div>
        </div>
      </div>
    </div>
  </div>

</template>


<style lang="less">
body {
  background-color: transparent;
}

.message-container {
  flex: 1;
  overflow-y: auto;
  height: 100%;
  margin: 15px 0;
  padding: 0 15px;
  max-height: calc(800px - 100px); 
  scroll-behavior: smooth; 
}

.message-container::-webkit-scrollbar {
  width: 6px;
}

message-container::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}

message-container::-webkit-scrollbar-track {
  background-color: transparent;
}
.message-item {
  margin-bottom: 20px;
  animation: fadeIn 0.3s ease-in-out;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: -15px;
    width: 3px;
    height: 0;
    background: linear-gradient(to bottom, transparent, rgba(77, 107, 254, 0.7), transparent);
    animation: lineGrow 0.5s ease-out 0.2s forwards;
  }
}

@keyframes lineGrow {
  0% {
    height: 0;
    opacity: 0;
  }
  100% {
    height: 100%;
    opacity: 1;
  }
}
.output {
  line-height: 1.6;
  font-size: 15px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 1;
}
.reasoning {
  padding: 12px 16px;
  color: rgba(255, 255, 255, 0.85);
  background: linear-gradient(135deg, rgba(80, 80, 120, 0.5), rgba(60, 60, 100, 0.4));
  border-radius: 12px;
  margin-bottom: 10px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1), inset 0 1px 2px rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.08);
  font-size: 14px;
  backdrop-filter: blur(5px);
  transition: all 0.3s ease;
  
  &:hover {
    box-shadow: 0 6px 20px rgba(60, 60, 100, 0.2), inset 0 1px 3px rgba(255, 255, 255, 0.15);
    transform: translateY(-2px);
  }
}
.content {
  padding: 16px 20px;
  background: linear-gradient(135deg, rgba(111, 66, 193, 0.5), rgba(87, 54, 163, 0.4));
  border-radius: 16px;
  color: #fff;
  font-size: 15.5px;
  box-shadow: 0 6px 20px rgba(111, 66, 193, 0.2), inset 0 1px 3px rgba(255, 255, 255, 0.15);
  border: 1px solid rgba(111, 66, 193, 0.25);
  line-height: 1.7;
  letter-spacing: 0.3px;
  backdrop-filter: blur(8px);
  transition: all 0.3s ease;
  
  &:hover {
    box-shadow: 0 8px 25px rgba(111, 66, 193, 0.25), inset 0 1px 5px rgba(255, 255, 255, 0.2);
    transform: translateY(-2px);
  }
}

// 修改左侧线条颜色，使其与新配色方案协调
.message-item {
  &::before {
    background: linear-gradient(to bottom, transparent, rgba(111, 66, 193, 0.7), transparent);
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
.dialog-bg {
  position: fixed;
  height: 100%;
  width: 100%;
  user-select: none;
  filter: blur(10px);
  z-index: -1;
}
.dialog-header {
  width: 50%;
  margin: 0 auto;
  height: 8px;
  backdrop-filter: blur(10px);
  border-radius: 10px;
  background-color: transparent;
  cursor: pointer;
  transition: all .1s ease-out;

  &:hover,
  &.dragging {
    background-color: rgba(200, 200, 200, .7);
  }
}
.dialog-content {
  padding: 10px;
  backdrop-filter: blur(10px);
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: 20px;
  background-color: rgba(77, 107, 254, .5);
  height: 50px;

  .btn {
    height: 50px;
    border-radius: 5px 10px 10px 5px;
    background: #4d6bfe;
    /* 主色不透明版本 */
    color: white;
    font-weight: 500;
    font-size: 18px;
    border: none;
    cursor: pointer;
    transition: all .1s ease-out;

    i {
      transform: rotateZ(90deg);
    }

    &:hover {
      background: #3a55cc;
      transform: translateY(-1px);
    }
  }
}
.dialog {
  display: flex;
  height: 100%;
  flex-direction: column;
  box-sizing: border-box;
}
.dialog-textarea {
  width: 100%;
  height: 100%;
  border: none;
  padding: 10px;
  box-sizing: border-box;
  resize: none;
  font-size: large;
  background-color: transparent;
  color: #fff;

  &::placeholder {
    color: rgba(255, 255, 255, 0.8);
    font-style: italic;
  }
}
</style>