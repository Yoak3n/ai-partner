<template>
    <div class="home-wrapper">
        <n-config-provider :theme-overrides="themeOverrides">
            <n-layout has-sider>
                <n-layout-sider
                    content-style="display: flex; flex-direction: column; justify-content: space-between;overflow: hidden;"
                    show-trigger :width="180" collapse-mode="width" bordered :collapsed-width="72" :collapsed="collapsed"
                    @collapse="collapsed = true" @expand="collapsed = false">
                    <n-menu v-model:value="activeKey" :collapsed="collapsed" :collapsed-width="72" :collapsed-icon-size="22"
                        :options="menuOptions" />
                    <div class="option-btn" style="display: flex;flex-direction: column;">
                        <n-button text 
                        @click="()=>{modalKey = 'about';showModal = !showModal}"
                        style="font-size: 48px;margin-bottom:2rem" 
                        size="large" 
                        class="setting"
                            >
                            <template #icon>
                                <n-icon>
                                    <ChatboxEllipsesOutline/>
                                </n-icon>
                            </template>
                        </n-button>
                        <n-button text style="font-size: 48px;margin-bottom:2rem" size="large" class="setting"
                            @click="()=>{modalKey = 'setting';showModal = !showModal}">
                            <template #icon>
                                <n-icon>
                                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
                                        viewBox="0 0 512 512">
                                        <circle cx="256" cy="256" r="48" fill="currentColor"></circle>
                                        <path
                                            d="M470.39 300l-.47-.38l-31.56-24.75a16.11 16.11 0 0 1-6.1-13.33v-11.56a16 16 0 0 1 6.11-13.22L469.92 212l.47-.38a26.68 26.68 0 0 0 5.9-34.06l-42.71-73.9a1.59 1.59 0 0 1-.13-.22A26.86 26.86 0 0 0 401 92.14l-.35.13l-37.1 14.93a15.94 15.94 0 0 1-14.47-1.29q-4.92-3.1-10-5.86a15.94 15.94 0 0 1-8.19-11.82l-5.59-39.59l-.12-.72A27.22 27.22 0 0 0 298.76 26h-85.52a26.92 26.92 0 0 0-26.45 22.39l-.09.56l-5.57 39.67a16 16 0 0 1-8.13 11.82a175.21 175.21 0 0 0-10 5.82a15.92 15.92 0 0 1-14.43 1.27l-37.13-15l-.35-.14a26.87 26.87 0 0 0-32.48 11.34l-.13.22l-42.77 73.95a26.71 26.71 0 0 0 5.9 34.1l.47.38l31.56 24.75a16.11 16.11 0 0 1 6.1 13.33v11.56a16 16 0 0 1-6.11 13.22L42.08 300l-.47.38a26.68 26.68 0 0 0-5.9 34.06l42.71 73.9a1.59 1.59 0 0 1 .13.22a26.86 26.86 0 0 0 32.45 11.3l.35-.13l37.07-14.93a15.94 15.94 0 0 1 14.47 1.29q4.92 3.11 10 5.86a15.94 15.94 0 0 1 8.19 11.82l5.56 39.59l.12.72A27.22 27.22 0 0 0 213.24 486h85.52a26.92 26.92 0 0 0 26.45-22.39l.09-.56l5.57-39.67a16 16 0 0 1 8.18-11.82c3.42-1.84 6.76-3.79 10-5.82a15.92 15.92 0 0 1 14.43-1.27l37.13 14.95l.35.14a26.85 26.85 0 0 0 32.48-11.34a2.53 2.53 0 0 1 .13-.22l42.71-73.89a26.7 26.7 0 0 0-5.89-34.11zm-134.48-40.24a80 80 0 1 1-83.66-83.67a80.21 80.21 0 0 1 83.66 83.67z"
                                            fill="currentColor"></path>
                                    </svg>
                                </n-icon>
                            </template>
                        </n-button>
                    </div>
                </n-layout-sider>
                <n-layout-content>
                    <router-view />
                </n-layout-content>

            </n-layout>
        </n-config-provider>
        <Modal v-model:show="showModal" :switch-callback="showModalFun" :modalKey="modalKey"/>
    </div>
</template>

<script lang="ts" setup>
import { ref, h, onMounted, onActivated, computed, onBeforeUnmount, watch } from 'vue'
import type { Component } from 'vue'
import { RouterLink, useRoute,useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

import {AddCircleOutline, ChatboxEllipsesOutline, TrashOutline} from '@vicons/ionicons5'
import { NLayout, NLayoutSider, NLayoutContent, NMenu, NIcon, NButton, NPopconfirm,NConfigProvider } from 'naive-ui'
import type { MenuOption } from 'naive-ui'

import { themeOverrides} from '../utils'
import Modal from '../components/Modal/index.vue'
import Message from '../components/Icon/Message.vue'
import emitter from '../bus'


let collapsed = ref<boolean>(true)
const historyChats = ref<Array<{ id: number, title: string }>>([])
const router = useRouter()

// 获取历史对话
const loadHistoryChats = async (id: number) => {
    // 更新菜单ui
    if (id && id != 0) {activeKey.value = `chat-${id}`}
    try {
        historyChats.value = await invoke('get_conversations')
    } catch (e) {
        console.error('获取历史对话失败:', e)
    }
}

let activeKey = ref<string>('')
const $route = useRoute()
const renderIcon = (icon: Component) => {return () => h(NIcon, null, { default: () => h(icon) })}
let showModal = ref<boolean>(false)
const showModalFun = (v:boolean) => showModal.value = v
let modalKey = ref('')
watch(
    () => $route.params.id,
    (newId) => {
        if (newId === 'new') {
            activeKey.value = 'NewChat'
        } else if (newId) {
            activeKey.value = `chat-${newId}`
        }
    },
    { immediate: true }
)
onMounted(() => {
    activeKey.value = $route.name?.toString() || ''
    loadHistoryChats(0)
    emitter.on('conversation-updated', (id)=>loadHistoryChats(id as number))
})
onBeforeUnmount(() => {
    emitter.off('conversation-updated')
})
onActivated(() => {
    activeKey.value = $route.name?.toString() || ''
})
const deleteConversation = async (id: number, event: MouseEvent) => {
    event.preventDefault() // 阻止路由跳转
    try {
        await invoke('delete_conversation', { conversationId: id })
        await loadHistoryChats(0) // 重新加载对话列表
        // 如果当前正在查看被删除的对话，则跳转到新建对话页面
        if ($route.params.id === id.toString()) {
            router.push('/chat/new')
        }
        emitter.emit('conversation-updated')
    } catch (e) {
        console.error('删除对话失败:', e)
    }
}
const handleNewChat = async (event: MouseEvent) => {
    event?.preventDefault()
    emitter.emit('reset-chat')
    if ($route.path !== '/chat/new') {
        await router.push('/chat/new')
    }
}
// 计算菜单选项
const menuOptions = computed<MenuOption[]>(() => [
    {
        label: () => h(
            'div',
            {
                onClick: (e: MouseEvent) => handleNewChat(e),
                style: 'width: 100%; cursor: pointer;'
            },
            { default: () => '新建对话' }
        ),
        key: 'NewChat',
        icon: renderIcon(AddCircleOutline),
        onClick: (e: MouseEvent) => handleNewChat(e)
    },
    {
        type: 'divider'
    },
    ...historyChats.value.map(chat => ({
        label: () => h('div', {
            style: 'display: flex; justify-content: space-between; align-items: center; width: 100%'
        }, [
            h(RouterLink, {
                to: `/chat/${chat.id}`,
                style: 'flex: 1; overflow: hidden; text-overflow: ellipsis;'
            }, { default: () => chat.title }),
            h(NPopconfirm, {
                onPositiveClick: (e: MouseEvent) => deleteConversation(chat.id, e),
                showIcon: false,
                positiveText: '确定',
                negativeText: '取消',
                placement:"right",
            }, {
                trigger: () => h(NButton, {
                    size: 'tiny',
                    quaternary: true,
                    style: 'margin-left: 8px;'
                }, {
                    icon: () => h(NIcon, null, {
                        default: () => h(TrashOutline)
                    })
                }),
                default: () => '确定要删除这个对话吗？'
            })
        ]),
        key: `chat-${chat.id}`,
        icon: renderIcon(Message)
    }))
])
</script>

<style lang="less">
.home-wrapper {
    height: 100%;
    .n-config-provider{
        .n-layout {
            height: 100%;
        }
    }
}
</style>