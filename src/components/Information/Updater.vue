<script setup lang="ts">
import {  ref } from 'vue';
import { invoke,Channel } from '@tauri-apps/api/core';

type DownloadEvent =
  | {
      event: 'Started';
      data: {
        contentLength: number;
      };
    }
  | {
      event: 'Progress';
      data: {
        chunkLength: number;
      };
    }
  | {
      event: 'Finished';
      data: {
        downloadId: number;
      };
    };
const progress = ref(0);
const totalSize = ref(0);
const downloading = ref(false);
const finished = ref(false);
async function installUpdate() {
    downloading.value = true;
    progress.value = 0;

    const onEvent= new Channel<DownloadEvent>();
    onEvent.onmessage=(message)=>{
        console.log(message);
        
        if(message.event==='Started'){
            downloading.value=true;
            totalSize.value=message.data.contentLength;
            console.log("started,total",totalSize.value);
            
        }else if(message.event==='Progress'){
            downloading.value=true;
            progress.value+=message.data.chunkLength;
            console.log("progress",progress.value);
        }else if(message.event==='Finished'){
            finished.value=true;
        }
    }



    try {
        // 调用后端函数，传递回调函数
        await invoke('install_update', {
            onEvent
        });
    } catch (error) {
        console.error('安装更新失败:', error);
        downloading.value = false;
    }
}

</script>

<template>
    <button @click="installUpdate" :disabled="downloading">安装更新</button>
    <div v-if="downloading && totalSize > 0">
        下载进度: {{ Math.round((progress / totalSize) * 100) }}%
    </div>
    <div v-if="finished">更新下载完成，请重启应用以应用更新</div>
</template>