<template>
    <div class="home-wrapper">
        <n-config-provider style="height: 100%;" :theme-overrides="themeOverrides">
            <n-layout has-sider>
                <n-layout-sider
                    content-style="display: flex; flex-direction: column; justify-content: space-between;overflow: hidden;"
                    show-trigger :width="180" collapse-mode="width" bordered :collapsed-width="72"
                    :collapsed="collapsed" @collapse="collapsed = true" @expand="collapsed = false">
                    <n-menu v-model:value="activeKey" :collapsed="collapsed" :collapsed-width="72"
                        :collapsed-icon-size="22" :options="menuOptions" />
                    <div class="option-btn" style="display: flex;flex-direction: column;">
                        <n-button text @click="router.push('/favorite')"
                            style="font-size: 48px;margin-bottom:2rem" size="large" class="setting">
                            <template #icon>
                                <n-icon color="#e54d42">
                                    <Heart />
                                </n-icon>
                            </template>
                        </n-button>
                        <n-button text @click="() => { modalKey = 'about'; showModal = !showModal }"
                            style="font-size: 48px;margin-bottom:2rem" size="large" class="setting">
                            <template #icon>
                                <n-icon>
                                    <ChatboxEllipsesOutline />
                                </n-icon>
                            </template>
                        </n-button>
                        <n-button text style="font-size: 48px;margin-bottom:2rem" size="large" class="setting"
                            @click="() => { modalKey = 'setting'; showModal = !showModal }">
                            <template #icon>
                                <n-icon>
                                    <Settings />
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
        <Modal v-model:show="showModal" :switch-callback="showModalFun" :modalKey="modalKey" />
    </div>
</template>

<script lang="ts" setup>
import { ref, h, onMounted, onActivated, computed, onBeforeUnmount, watch } from 'vue'
import type { Component } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

import { AddCircleOutline, ChatboxEllipsesOutline, TrashOutline, Settings,Heart } from '@vicons/ionicons5'
import { NLayout, NLayoutSider, NLayoutContent, NMenu, NIcon, NButton, NPopconfirm, NConfigProvider, type MenuOption } from 'naive-ui'

import { themeOverrides } from '../utils'
import Modal from '../components/Modal/index.vue'
import Message from '../components/Icon/Message.vue'
import emitter from '../bus'


let collapsed = ref<boolean>(true)
const historyChats = ref<Array<{ id: number, title: string }>>([])
const router = useRouter()

// 获取历史对话
const loadHistoryChats = async (id: number) => {
    // 更新菜单ui
    if (id && id != 0) { activeKey.value = `chat-${id}` }
    try {
        historyChats.value = await invoke('get_conversations')
    } catch (e) {
        console.error('获取历史对话失败:', e)
    }
}

let activeKey = ref<string>('')
const $route = useRoute()
const renderIcon = (icon: Component) => { return () => h(NIcon, null, { default: () => h(icon) }) }
let showModal = ref<boolean>(false)
const showModalFun = (v: boolean) => showModal.value = v
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
    emitter.on('conversation-updated', (id) => loadHistoryChats(id as number))
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
                placement: "right",
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

    .n-config-provider {
        .n-layout {
            height: 100%;
        }
    }
}
</style>