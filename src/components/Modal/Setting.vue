<template>
    <div class="setting-wrapper">
        <n-form size="large" 
        label-placement="left" 
        label-width="100px" 
        v-model="model"
        @submit="saveSetting"
        :style="{ width: '600px', textAlign: 'start' }"
            label-align="left">
            <n-divider >API</n-divider>
            <n-form-item label="API_base">
                <n-input placeholder="请输入API请求地址" v-model:value="model!.api.url" />
            </n-form-item>
            <n-form-item label="API_key">
                <n-input placeholder="请输入API请求key" type="password" show-password-on="click" v-model:value="model!.api.key" :input-props="{autocomplete:'off'}" >
                    <template #password-visible-icon>
                        <n-icon>
                            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
                                viewBox="0 0 16 16">
                                <g fill="none">
                                    <path
                                        d="M11.284 11.99l2.862 2.864a.5.5 0 0 0 .708-.708l-13-13a.5.5 0 1 0-.708.708l2.566 2.565l-2.169 1.926C1.211 6.62 1 7.035 1 7.5v2A2.5 2.5 0 0 0 3.5 12h1A2.5 2.5 0 0 0 7 9.5V9h1.293l.716.716a2.5 2.5 0 0 0 2.275 2.275zM5.293 6H3.437l.983-.873l.873.873zm8.48 5.652L9.095 6.974A1.5 1.5 0 0 1 10.5 6h2.063L10.31 4H9.5a.5.5 0 1 1 0-1h1a.5.5 0 0 1 .332.126l3.625 3.219c.332.275.543.69.543 1.155v2c0 .916-.492 1.716-1.227 2.152zM5.213 3.09L6.12 4H6.5a.5.5 0 1 0 0-1h-1a.5.5 0 0 0-.288.091z"
                                        fill="currentColor"></path>
                                </g>
                            </svg>
                        </n-icon>
                    </template>
                    <template #password-invisible-icon>
                        <n-icon>
                            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
                                viewBox="0 0 28 28">
                                <g fill="none">
                                    <path
                                        d="M9.32 7.5c-.328 0-.643.13-.877.36L5.768 10.5h4.482c1.348 0 2.47.97 2.705 2.25h2.09a2.75 2.75 0 0 1 2.705-2.25h4.482l-2.675-2.64a1.25 1.25 0 0 0-.878-.36h-1.43a.75.75 0 1 1 0-1.5h1.43a2.75 2.75 0 0 1 1.932.793l4.42 4.361c.104.089.201.185.291.288l.025.024l-.002.002c.408.48.655 1.102.655 1.782v4A3.75 3.75 0 0 1 22.25 21h-3.5A3.75 3.75 0 0 1 15 17.25v-3h-2v3A3.75 3.75 0 0 1 9.25 21h-3.5A3.75 3.75 0 0 1 2 17.25v-4c0-.68.247-1.302.655-1.782l-.002-.002l.027-.026c.089-.102.185-.197.288-.284l4.421-4.363A2.75 2.75 0 0 1 9.321 6h1.429a.75.75 0 0 1 0 1.5H9.32z"
                                        fill="currentColor"></path>
                                </g>
                            </svg>
                        </n-icon>
                    </template>
                </n-input>
            </n-form-item>
            <n-form-item label="model">
                <n-select
                    v-model:value="model!.api.model" 
                    :options="modelOptions"
                    placeholder="请选择或输入模型名称"
                    filterable
                    tag
                >
                </n-select>
                <n-button 
                    secondary 
                    type="info" 
                    @click="$ApiStore.clearModelHistory()"
                    :disabled="!modelHistory.length"
                >
                    清除历史
                </n-button>
            </n-form-item>
            <n-divider>快捷键</n-divider>
            <n-form-item label="快捷对话框" >
                <n-input placeholder="按下按键设置快捷键" 
                v-model:value="model!.hotkey.dialog" 
                @keydown="(e)=>{
                    keyDown(e,updateHotKeyDialog)
                }" 
                @focus="()=>{
                    DialogHotkeyChanged = true
                    unregister(model!.hotkey.dialog)
                    model!.hotkey.dialog = '';
                }"
                />
            </n-form-item>                
            <n-form-item>
                <n-button type="primary" class="confirm-btn" style="margin: 0 auto;width:20%" @click="saveSetting">确定</n-button>
            </n-form-item>
        </n-form>
    </div>
</template>

<script lang="ts" setup>
import { ref,onMounted, computed } from 'vue';
import { NForm, NFormItem, NInput, NButton, NIcon,NDivider,NSelect } from 'naive-ui';
import type { AppSetting } from '../../types';
import { storeToRefs } from 'pinia'
import { useApiStore } from '../../store';
import {updateAllSetting,querySetting} from '../../api/db'
import { keyDown } from '../composables/recognize_key';
import { invoke } from '@tauri-apps/api/core';
import {isRegistered, unregister} from '@tauri-apps/plugin-global-shortcut'
const $ApiStore = useApiStore();
const { modelHistory } = storeToRefs($ApiStore)
const modelOptions = computed(() => {
    return modelHistory.value.map(m => ({
        label: m,
        value: m
    }));
})
let DialogHotkeyChanged = ref(false)
const updateHotKeyDialog = (value:string)=>{
    model.value!.hotkey.dialog = value;
}

let model = ref<AppSetting|null>({
    api:{
        url: '', key: '', model: ''
    },
    hotkey:{
        dialog:''
    }
}
);
onMounted(async()=>{
    const s = await querySetting();
    if(s){
        model.value = s;
    }
    $ApiStore.initModelHistory();
})
const props = defineProps({
    switchCallback: {
        type: Function,
        default: () => {}
    }
})

const registerHotKey = async(key:string)=>{
    if(!DialogHotkeyChanged.value){return}
    let res = await isRegistered(key)
    if(res){
        window.$message.error('快捷键已被占用') 
    }else{
        await invoke('register_shortcut_by_frontend',{name:'dialog',shortcut:key})
    }
}

const saveSetting = async()=>{
    if(model.value == null) return;
    const s:AppSetting = model.value;
    if (s.api.model) {
        $ApiStore.addModelToHistory(s.api.model);
    }
    await registerHotKey(s.hotkey.dialog)
    await updateAllSetting(s)
    $ApiStore.getApifromConfig();
    window.$message.success('保存成功');
    props.switchCallback(false);
}
onMounted(()=>saveSetting)
</script>

<style scoped lang="less">
.setting-wrapper {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: start;
    margin: 5rem 0 0 0;
    .confirm-btn {
        margin: 0 auto;
        width: 120px;
        height: 40px;
        font-size: 15px;
        font-weight: 500;
        border-radius: 20px;
        transition: all 0.3s ease;
        
        &:hover {
            transform: translateY(-2px);
            box-shadow: 0 2px 8px rgba(74, 144, 226, 0.2);
        }
        
        &:active {
            transform: translateY(0);
        }
    }
}
@media screen and (max-width: 768px) {
    .setting-wrapper{
      margin:5rem 0 ;
    }
  }
</style>