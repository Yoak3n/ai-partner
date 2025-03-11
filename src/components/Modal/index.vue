<template>

        <n-modal :show="props.show" @update:show="()=>props.switchCallback(!show)" >
            <n-card size="large">
                <n-config-provider :theme-overrides="modalThemeOverrides" v-if="props.modalKey==='setting'">
                <SettingView :switchCallback="props.switchCallback"  />
            </n-config-provider>
                <About :switchCallback="props.switchCallback" v-if="props.modalKey==='about'" />
            </n-card>
        </n-modal>

</template>

<script lang="ts" setup>
import { NModal,NCard,NConfigProvider } from 'naive-ui';
import SettingView from './Setting.vue'
import About from '../Information/About.vue'
import { modalThemeOverrides } from '../../utils';

const props = defineProps({
    switchCallback:{
        type:Function,
        required:true
    },
    show:{
        type:Boolean,
        default:false
    },
    modalKey:{
        type:String,
        default:''
    }
})
</script>

<style scoped lang="less">

</style>