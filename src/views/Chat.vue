<script lang="ts" setup>
import {nextTick, onBeforeUnmount, onMounted, ref,watch } from 'vue';
import { useRoute,useRouter} from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import {storeToRefs} from 'pinia'

import { NIcon } from 'naive-ui';
import {Reload,ReturnUpForwardOutline,Pause} from '@vicons/ionicons5'

import ChatBoard from '../components/Chat/ChatBoard/index.vue'

import type { MessageItem} from '../types'
import { addCopyButtons, throttle } from '../utils';
import {unListenAll} from '../bus'
import emitter from '../bus';
import { useApiStore ,useAppStore} from '../store';

// 添加对话相关的状态
const route = useRoute()
const router = useRouter()
const conversationId = ref<number | null>(null)
const conversationTitle = ref<string>('')
const defaultMessages:Array<MessageItem> = []
let messages = ref<MessageItem[]>(defaultMessages)
let input = ref<string>('')

const ts = ref(Date.now())
// 加载历史对话
const loadConversation = async (id: number) => {
  try {
    const msgs= await invoke('get_conversation_messages', { conversationId: id })
    if (Array.isArray(msgs)) {
      messages.value.splice(0, messages.value.length, ...(msgs as MessageItem[] ))
      nextTick(()=>addCopyButtons())
      conversationId.value = id
      throttleEmitScrollToBottom()
    } else {
      window.$message.error('加载对话返回格式错误')
      router.push('/chat/new')
    }
  } catch (e) {
    window.$message.error('加载对话失败:' + e)
    router.push('/chat/new')
  }
}

onMounted(async()=>{
  $ApiStore.getApifromConfig()
  emitter.on('reset-chat', resetHistory)
})
onBeforeUnmount(()=>{
  unListenAll()
  emitter.off('reset-chat', resetHistory)
})
// 监听路由参数变化
watch(
  () => route.params.id,
  async (newId) => {
    if (newId === 'new') {
      conversationId.value = null
      messages.value = []
      conversationTitle.value = ''
      input.value = ''
      $AppStore.setGenerating(false)
    } else if (newId) {
      await loadConversation(Number(newId))
      nextTick(()=>addCopyButtons())
    }
  },
  { immediate: true, flush: 'post'}
)

const setMessage = (mi:MessageItem) => {
  messages.value.push(mi);
  nextTick(() => {
    throttleEmitScrollToBottom()
  })
}

const submitUserMessage = async() => {
  if(input.value.trim() == '') return
  ts.value = Date.now()
  const m:MessageItem = {role:'user', content:input.value, timestamp:ts.value - 5,reasoning_content: ''}
  setMessage(m)
  const newMessage = {role:'assistant-generating', content:'', timestamp:ts.value, reasoning_content:''}
  setMessage(newMessage)
  // 如果是新对话，先创建对话
  if (!conversationId.value) {
    try {
      // 使用第一条用户消息的前20个字符作为对话标题
      const title = input.value.slice(0, 20) + (input.value.length > 20 ? '...' : '')
      conversationId.value = await invoke('create_conversation', { title })
      emitter.emit('conversation-updated',conversationId.value)
      history.replaceState(history.state, '', `${conversationId.value}`)
      // await router.replace(`/chat/${conversationId.value}`)
    } catch (e) {
      window.$message.error(`'创建对话失败: ${e}`,{duration: 5000})
      return
    }
  }
    // 保存消息
  try {
    await invoke('save_message', { 
      conversationId: conversationId.value,
      message: m
    })
    input.value = ''
    generateBotResponseStream()
  } catch (e) {
    window.$message.error(`保存消息失败:${e}`, {duration: 5000})
  }
}

const $AppStore = useAppStore()
const $ApiStore = useApiStore()
let {api} = storeToRefs($ApiStore)
let {generating} = storeToRefs($AppStore)
const isExpanded = ref(false)
const toggleExpand = () => isExpanded.value = !isExpanded.value
const generateBotResponseStream = async () => {
  const currentMessages:any = []
  messages.value.forEach((item) => {
    if (item.role !== 'assistant-generating'){
      currentMessages.push({role:item.role, content:item.content,reasoning_content:'',timestamp:item.timestamp})
    }
  })
  try {
    const res = await invoke('completions_stream', {
      id: ts.value,
      conversationId: conversationId.value,
      messages: currentMessages
    })
    let m = res as MessageItem
    m.timestamp = ts.value
    
    // 保存助手的回复
    await invoke('save_message', {
      conversationId: conversationId.value,
      message: m
    })
    updateHistoryStream(m)
  } catch (e) {
    window.$message.error(`生成回复失败:${e}`, {duration: 5000})
  }
}
const updateHistoryStream = (m: MessageItem) => {
  const index = messages.value.findIndex((item) =>item.timestamp == m.timestamp)
  if (index != -1){messages.value[index] = {...messages.value[index], content:m.content, reasoning_content:m.reasoning_content, role:m.role}
  }else{messages.value.push(m)}
  nextTick(async() => { 
    throttleEmitScrollToBottom()
    addCopyButtons()
  })
}

const resetHistory = async() => {
  messages.value.splice(0, messages.value.length)
  await nextTick()
  if (route.path !== '/chat/new') {
    history.replaceState(history.state, '', '/chat/new') // 如果不在新对话页面，则跳转
  }
  conversationId.value = null
  conversationTitle.value = ''
  input.value = ''
  $AppStore.setGenerating(false)
}

// 需要配合后端把消息的id和timestamp关联起来
// 还需要添加index字段，让针对同一个问题的回复可以关联之前的回复，出现在同一个位置
// const retryAskLastAnswer = async(conversationId:number,userMessageId:number) => {
// 目前只实现简化版本,即只针对最后一个用户消息，重新生成的回复会出现在之前的回复下面
const retryAskLastAnswer = async() => {
  try{
    const userMessages = messages.value.filter((item) => item.role == 'user')
    const lastUserMessage = userMessages.length > 0 ? userMessages[userMessages.length - 1] : null
    const msgs = await invoke('get_conversation_messages', { conversationId: conversationId.value }) 
    if (Array.isArray(msgs) && msgs.length > 0){
      let index= msgs.findIndex((item) => item.timestamp == lastUserMessage?.timestamp )
      if (index == -1) {
        window.$message.error('找不到对应的用户消息')
        return
      }else{
        // messages.value.splice(index+1, messages.value.length - (index + 1))
        ts.value = Date.now()
        const newMessage = {role:'assistant-generating', content:'', timestamp:ts.value,reasoning_content:''}
        setMessage(newMessage)
        await generateBotResponseStream()
      }
    }
  }catch(e){
    window.$message.error(`重试失败:${e}`, {duration: 5000})
  }

}


const throttleEmitScrollToBottom  = throttle(() =>emitter.emit('scrollToBottom'), 1000)
</script>
<template>
  <div class="chat-view">
    <ChatBoard :messages="messages" :model="api.model" :key="conversationId || 'new'"/>
    <form class="chat-input" 
    :class="{ 'expanded': isExpanded }"
    @submit="(e) =>{
      e.preventDefault()
      submitUserMessage()
    }">
      <div class="expand-button" @click="toggleExpand">
        <div class="expand-icon">
      </div></div>
      <textarea 
      placeholder="Type a message..." 
      v-model="input"  
      required 
      minlength="1" 
      @keydown="(e) => {
        if (e.key === 'Enter' && !e.shiftKey && !generating) {
          e.preventDefault()
          submitUserMessage()
        }
      }
      "/>
      <button type="button" class="reset-btn" @click="retryAskLastAnswer">
        <n-icon size="24">
          <Reload/>
        </n-icon>
      </button>
      <button type="submit" class="submit-btn" v-if="!generating">
        <n-icon size="24" :class="{ 'disabled-icon': !input.trim() }">
          <ReturnUpForwardOutline/>
        </n-icon>
      </button>
      <button type="button" class="pause-btn" v-else @click="(e)=>{
        e.preventDefault()
        invoke('pause_stream',{id:ts}).then(()=>$AppStore.setGenerating(false))
      }">
        <n-icon size="24">
          <Pause/>
        </n-icon>
      </button>
    </form>
  </div>

</template>
<style lang="less" scoped >
.chat-view {
  position: relative;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.chat-input {
  position: fixed;
  bottom: 10px;
  width: 60%;
  display: flex;
  padding: 8px;
  border-radius: 12px;
  background-color: #f5f7fa;
  box-sizing: border-box;
  transition: all 0.3s ease;
  z-index: 10;
  &.expanded {
    textarea {
      height: 150px;
    }
  }
  textarea {
    transition: height 0.3s ease;
    width: 90%;
    border-radius: 8px;
    background-color: #f5f7fa;
    padding: 15px;
    height: 50px;
    font-size: 1rem;
    box-sizing: border-box;
    overflow: auto;
    resize: none;
    scrollbar-width: none;
    -ms-overflow-style: none;
    position: relative;
    border: none;
    color: #333;
    &::placeholder {
      color: #999;
    }
    &:focus {
      outline: none;
    }
  }
  .expand-button {
    position: absolute;
    top: -16px;
    left: 50%;
    transform: translateX(-50%);
    width: 26px;
    height: 16px;
    background-color: #f5f7fa;
    border: 1px solid #e1e1e1;
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    opacity: 0.6;
    transition: opacity 0.2s;

    &:hover {
      opacity: 1;
    }

    .expand-icon {
      width: 10px;
      height: 2px;
      background-color: #4a90e2;
      position: relative;

      &::before {
        content: '';
        position: absolute;
        left: 0;
        top: -3px;
        width: 100%;
        height: 2px;
        background-color: #4a90e2;
      }
    }
  }
  button {
    height: 50px;
    border-radius: 8px;
    padding: 10px 15px;
    color: #fff;
    background-color: #4a90e2;
    cursor: pointer;
    border: none;
    transition: all 0.2s ease;
    margin-left: 8px;
    .disabled-icon {
      opacity: 0.4;
      transform: scale(0.9);
      filter: grayscale(0.5);
      transition: all 0.3s ease;
    }
    &.reset-btn {
      background-color: #f5f7fa;
      color: #4a90e2;
      border: 1px solid #e1e1e1;
      
      &:hover {
        background-color: #e1eefb;
        color: #357abd;
      }
    }

    &.submit-btn {
      &:hover {
        background-color: #357abd;
      }
    }

    &.pause-btn {
      background-color: #fff;
      color: #666;
      border: 1px solid #e1e1e1;
      
      &:hover {
        background-color: #f5f7fa;
        color: #4a90e2;
      }
    }
  }

  &:valid button {
    &.reset-btn {
      background-color: #fff;
      color: #4a90e2;
      border: 1px solid #e1e1e1;
      
      &:hover {
        background-color: #e1eefb;
        color: #357abd;
      }
    }
    
    &.submit-btn {
      background-color: #4a90e2;
      
      &:hover {
        background-color: #357abd;
      }
    }
  }

  &:invalid button.submit-btn {
    background-color: #f5f7fa;
    border: 1px solid #e1e1e1;
    cursor: default;
    color: #4a90e2;
    .disabled-icon {
      opacity: 0.65;
      transform: scale(0.95);
      filter: grayscale(0.3);
    }
    &:hover {
      background-color: #f5f7fa;
      .disabled-icon {
        opacity: 0.75;
        transform: scale(0.97);
      }
    }
  }

  &:focus-within {
    outline: 1.5px solid #4a90e2;
    background-color: #fff;
    box-shadow: 0 2px 12px rgba(74, 144, 226, 0.1);
    bottom: 21px;
  }

  @media screen and (max-width: 768px) {
    width: 90%;    // 移动端适配
    bottom: 10px;
    &:focus-within {
      bottom: 11px;
    }
    button {
      padding: 5px 10px;
    }
  }
}
</style>