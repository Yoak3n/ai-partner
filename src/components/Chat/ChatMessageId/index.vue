<script setup lang="ts">
import { onMounted, ref, PropType, nextTick, onBeforeUnmount } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { NSpin } from 'naive-ui';

import { MessageItem,Payload } from '../../../types';
import {registerNewListen, getUnlistenFnAndOff} from '../../../bus';
import emitter from '../../../bus';
import { useAppStore } from '../../../store';
import {throttle} from '../../../utils'
import MarkdownRender from '../../Markdown/index.vue'

let messageUpdate = ref<MessageItem>(
    {
        role: '',
        timestamp: 0,
        content: '',
        reasoning_content: ''
    }
)
let isThinking = ref(false)
const props = defineProps({
    message: {
        type: Object as PropType<MessageItem>,
        required: true
    },
})
const $AppStore = useAppStore()



onMounted(async () => {
    messageUpdate.value = {
        ...props.message,
        content: props.message.content || '',
        reasoning_content: props.message.reasoning_content || ''
    }

    if (messageUpdate.value.role == 'assistant-generating') {
        await nextTick()
        const unlistenFnData = await listen('stream-data', (event) => {
            const payload = event.payload as Payload
            if (payload.id!= props.message.timestamp) return
            messageUpdate.value = {
                ...messageUpdate.value,
                role: 'assistant',
                content: payload.message_type === 'content' 
                    ? (messageUpdate.value.content || '') + payload.data 
                    : messageUpdate.value.content,
                reasoning_content: payload.message_type === 'reasoning_content'
                    ? (messageUpdate.value.reasoning_content || '') + payload.data
                    : messageUpdate.value.reasoning_content
            }
            if (payload.message_type == 'reasoning_content') {
                isThinking.value = true
                $AppStore.setGenerating(true)
            } else if (payload.message_type == 'content') {
                isThinking.value = false
                $AppStore.setGenerating(true)
            } else {
                unlistenFnData()
                unlistenFnError()
                $AppStore.setGenerating(false)
                isThinking.value = false
            }
            nextTick(() => {
                throttleEmitScrollToBottom()
            })
        })
        const unlistenFnError = await listen('stream-error', (event) => {
            const payload = event.payload as Payload
            if (payload.id != props.message.timestamp) return
            messageUpdate.value.role = 'system'
            messageUpdate.value.content = payload.data
            unlistenFnError()
            unlistenFnData()
            $AppStore.setGenerating(false)
        })
        registerNewListen(messageUpdate.value.timestamp!,unlistenFnError)

    } else {
        messageUpdate.value!.content = props.message.content
        $AppStore.setGenerating(false)
    }
})
const throttleEmitScrollToBottom = throttle(() => {emitter.emit('scrollToBottom')},500)
onBeforeUnmount(() => {
    getUnlistenFnAndOff(messageUpdate.value.timestamp!)
})


</script>


<template>
    <div
        class="message"
        :class="messageUpdate.role"
        >
        <div class="message-item">
            <n-spin v-if="messageUpdate.role === 'assistant' || messageUpdate.role === 'system'" :show="isThinking" stroke="#4a90e2" size="large">
                <img class="bot-avatar" 
                src="../../../assets/bot.svg"/>
            </n-spin>
            <div class="output">
                <div class="thinking">
                    <div class="reasoning-text" v-if="messageUpdate.reasoning_content != ''&& messageUpdate.reasoning_content != null &&messageUpdate.reasoning_content != undefined">
                        {{messageUpdate?.reasoning_content }}
                    </div>
                </div>
                <div class="message-content" v-if="messageUpdate.content != ''">
                    <div class="message-text">
                        <MarkdownRender :source="messageUpdate.content" />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="less" scoped>
.message {
    display: flex;
    align-items: center;
    gap: 11px;

    .message-item {
        max-width: 75%;
        display: flex;
        align-items: flex-start;
        gap: 11px;
        .bot-avatar {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background-color: #4a90e2;
            box-shadow: 0 2px 8px rgba(74, 144, 226, 0.2);
        }
        .output {
            .thinking {
                .reasoning-text {
                    padding: 12px 16px;
                    position: relative;
                    font-size: 0.85rem;
                    white-space: pre-line;
                    color: #666;
                    background-color: #f5f7fa;
                    border-radius: 10px;
                    margin-bottom: 8px;
                }
            }
            .message-content {
                background-color: #edf4fc;
                box-shadow: 0 2px 8px rgba(74, 144, 226, 0.1);
                .message-text {
                    padding: 12px 16px;
                    position: relative;
                    font-size: 0.95rem;
                    white-space: normal;
                    color: #333;
                }
            }
        }
    }

    &.assistant,
    &.assistant-generating {
        .message-item {
            .message-content {
                border-radius: 12px 12px 12px 2px;
            }
        }
    }
    &.system {
        .message-item{
            .message-content{
                color: #e54d42;
                background-color: #fff2f0;
                border-radius: 12px 12px 12px 2px;
                border: 1px solid #ffccc7;
            }
            .bot-avatar{
                background-color: #e54d42;
            }
        }
    }
    &.user{
        .message-item{
            .message-content{
                background-color: #fff;
                border: 1px solid #e1eefb;
                border-radius: 12px 12px 2px 12px;
            }

        }
    }
}

.user {
    flex-direction: row-reverse;

    .message-item {
        flex-direction: row-reverse;
    }
}
</style>