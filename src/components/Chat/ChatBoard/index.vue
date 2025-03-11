<script lang="ts" setup>
import { nextTick,  onMounted, ref,onUnmounted,PropType} from 'vue'
import ChatMessageId from '../ChatMessageId/index.vue'
import type { MessageItem } from '../../../types/index'
import emitter from '../../../bus';
let chatBody = ref<HTMLInputElement | null>(null)
// 添加用户滚动状态变量
const userScrolling = ref(false);
const scrollTimeout = ref<number | null>(null);

// 处理用户滚动事件
const handleScroll = () => {
  if (!chatBody.value) return;
  const isAtBottom = chatBody.value.scrollHeight - chatBody.value.scrollTop <= chatBody.value.clientHeight + 150;
  
  // 如果不在底部，设置用户正在滚动状态
  if (!isAtBottom) {
    userScrolling.value = true;
    // 清除之前的定时器
    if (scrollTimeout.value !== null) {
      clearTimeout(scrollTimeout.value);
    }
    
    // 设置新的定时器，5秒后恢复自动滚动
    scrollTimeout.value = window.setTimeout(() => {
      userScrolling.value = false;
      scrollTimeout.value = null;
    }, 5000);
  } else {
    // 如果已经在底部，则认为不需要滚动
    userScrolling.value = false;
    if (scrollTimeout.value !== null) {
      clearTimeout(scrollTimeout.value);
      scrollTimeout.value = null;
    }
  }
};
onMounted(() => {
  if (chatBody.value) {
    chatBody.value.addEventListener('scroll', handleScroll, { passive: true });
  }
  nextTick(() => {
    // 后面再加个滚动到底部的按钮
    if (chatBody.value) {
      chatBody.value.scrollTo({ top: chatBody.value.scrollHeight });
    }
  });
  emitter.on('scrollToBottom', ()=>{
    if (!userScrolling.value) {
      scrollToBottom();
    }
})
});
onUnmounted(() => {
  if (chatBody.value) {
    chatBody.value.removeEventListener('scroll', handleScroll);
  }
  if (scrollTimeout.value !== null) {
    clearTimeout(scrollTimeout.value);
  }
  emitter.off('scrollToBottom')
})

// const messageList = computed(() => props.messages || [])

const scrollToBottom = () => {
  nextTick(() => {
    if (chatBody.value) {
      chatBody.value.scrollTo({ top: chatBody.value.scrollHeight, behavior: 'smooth' });
    }
  });
};
defineProps(
  {
    messages: {
      type: Array as PropType<MessageItem[]>,
      required: true
    },
    model:{
      type:String,
      default:''
    }
  }
)

</script>
<template>
  <div class="chat-board" ref="chatBody">
    <h1>{{model}}</h1>
    <div class="chat-body">
      <ChatMessageId v-for="message in messages" :message="message" 
      :key="`${message.timestamp}-${message.role}`"></ChatMessageId>
    </div>
  </div>
</template>


<style lang="less" scoped>
.chat-board {
  width: 100%;
  height: calc(100vh - 150px);
  margin-bottom: 100px;
  position: relative;

  scrollbar-width: thin;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #ddd3f9 transparent;

  .chat-body {
    display: flex;
    flex-direction: column;
    padding: 0 50px;
    gap: 20px;
  }
  @media screen and (max-width: 768px) {
    .chat-body{
      padding:0 ;
    }
  }
}
</style>