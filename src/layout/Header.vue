<script lang="ts" setup>
import { computed, toRefs,ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { exit } from '@tauri-apps/plugin-process';

import { useAppStore,type CloseAction } from '../store';

const heightString  = computed(() => `${heigth.value}px`)
const props = defineProps({
    heigth:{
        type: Number,
        default: 30
    }
})
const {heigth} = toRefs(props)
const appStore = useAppStore()
const showCloseDialog = ref(false)
const rememberChoice = ref(false)

const minimize = () => {
    let w = getCurrentWindow()
    w.minimize()
}
const toggleFullscreen = async() => {
    let w = getCurrentWindow()
    const isFullscreen = await w.isFullscreen()
    w.setFullscreen(!isFullscreen)
}
const hideToTray = () => {
    let w = getCurrentWindow()
    showCloseDialog.value = false
    w.hide()
}
const handleCloseAction = (action: CloseAction) => {
    if (rememberChoice.value) {
        appStore.setCloseAction(action)
    }
    if (action === 'hide') {
        hideToTray()
    } else {
        exit(0)
    }
}
const close = () => {
    if (appStore.closeAction=== 'ask'){
        showCloseDialog.value = true
    }else if (appStore.closeAction === 'hide') {
        hideToTray()
    }else if (appStore.closeAction === 'exit') {
        exit(0)
    }
}
const cancelClose = () => {
    showCloseDialog.value = false
}


</script>
<template>
    <div class="header" data-tauri-drag-region>
        <div class="title">
            <!-- <img src="../../src-tauri/icons/icon.png" alt="tauri" width="20" height="20"> -->
        </div>
        <div class="option">
            <button @click="minimize">
                <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
            </button>
            <button @click="toggleFullscreen"> 
                <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" /></button>
            <button @click="close">
                <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </button>
        </div>
    </div>
    <div v-if="showCloseDialog" class="close-dialog-overlay">
        <div class="close-dialog">
            <h3>关闭确认</h3>
            <p>您想要如何关闭应用程序？</p>
            <div class="dialog-buttons">
                <button @click="handleCloseAction('hide')">隐藏到托盘</button>
                <button @click="handleCloseAction('exit')" class="exit-button">退出程序</button>
                <button @click="cancelClose" class="cancel-button">取消</button>
            </div>
            <div class="remember-choice">
                <input type="checkbox" id="remember-choice" v-model="rememberChoice">
                <label for="remember-choice">记住我的选择</label>
            </div>
        </div>
    </div>
</template>


<style scoped lang="less">
.header {
    position: fixed;
    top: 0;
    width: 100%;
    height: v-bind(heightString);
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #fff;
    background-color: #fff;
    box-shadow: 1px 0px 5px 0px rgba(3, 3, 3, 0.3);
    z-index: 9999;
}

.title {
    width: 30px;
    padding-left: .5rem;
    img{
        padding-top: 5px;
        line-height: 30px;
        background: transparent;
    }
}

.option {
    height: 100%;
    display: flex;
    button {
        cursor: pointer;
        background-color: transparent;
        border-radius: 5px;
        height: 100%;
        width: 30px;
        padding: auto;
        &:hover{
            background-color: #eee;
        }
        img{
            line-height: 30px;
            background: transparent;
        }
    }
}
.close-dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 10000;
}

.close-dialog {
    background-color: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    width: 300px;
    text-align: center;
    
    h3 {
        margin-top: 0;
        color: #333;
    }
    
    p {
        margin-bottom: 20px;
        color: #666;
    }
}

.dialog-buttons {
    display: flex;
    justify-content: center;
    gap: 10px;
    margin-bottom: 15px;
    
    button {
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        background-color: #f0f0f0;
        color: #333;
        
        &:hover {
            background-color: #e0e0e0;
        }
        
        &.exit-button {
            background-color: #ff4d4f;
            color: white;
            
            &:hover {
                background-color: #ff7875;
            }
        }
        
        &.cancel-button {
            background-color: #1890ff;
            color: white;
            
            &:hover {
                background-color: #40a9ff;
            }
        }
    }
}

.remember-choice {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    margin-top: 10px;
    
    input[type="checkbox"] {
        cursor: pointer;
    }
    
    label {
        cursor: pointer;
        color: #666;
        font-size: 14px;
    }
}
</style>
