<script setup lang="ts">
import { computed, ref, type PropType } from 'vue';
import {NTag,NProgress} from 'naive-ui'
import { VersionComparation } from '../composables';
import MarkdownRender from '../Markdown/index.vue';
import emitter from '../../bus';
const downloading = ref(false);
const props = defineProps({
  version: {
    type: Object as PropType<VersionComparation>,
    default: true,
  },
  content:{
    type:Number,
    default:0
  },
  downloaded:{
    type:Number,
    default:0
  }
});
const percent = computed(()=>{
  return props.content == 0 ? 0: Math.floor(props.downloaded/props.content)*100
})

</script>

<template>
    <div class="update-dialog">
        <div class="update-header">
          <div class="version-info">
            <h2>发现新版本</h2>
            <div class="version-tags">
              <n-tag type="warning" size="small">当前版本: {{ version.current_version }}</n-tag>
              <n-tag type="success" size="small">新版本: {{ version.version }}</n-tag>
            </div>
          </div>
        </div>
        <div class="update-action">

          <div class="update-progress">
            <n-progress type="circle" :percentage="percent" :stroke-width="12" :color="{ stops: ['white', '#18a058'] }" >
              <div class="indicator">
                <button class="update-button" @click="()=>{downloading = true;emitter.emit('install_update')}">
                  {{ !downloading ? '下载' : percent == 100 ? '安装' :'下载中...'}}
                </button>
              </div>
            </n-progress>
          </div>
        </div>
        <div class="update-content">
          <div class="update-notes">
            <h3>更新内容</h3>
            <div class="notes-content" v-if="version.note">
              <markdown-render :source="version.note"/>
            </div>
            <div class="notes-content empty" v-else>
              暂无更新说明
            </div>
          </div>
        </div>
        
      </div>
</template>

<style scoped lang="less">
.update-dialog {
    padding: 10px;
    max-width: 500px;
    
    .update-header {
      margin-bottom: 20px;
      
      h2 {
        margin: 0 0 12px 0;
        font-size: 20px;
        font-weight: 600;
      }
    }
    .update-action{
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      .indicator{
        width: 90px;
        height: 90px;
        border-radius: 50%;
        cursor: pointer;
        transition: transform 0.3s ease;
        &:hover {
          transform: scale(1.05);
        }
        .update-button{
          background-color: #fff;
          width: 100%;
          height: 100%;
          margin: 0 auto;
          border-radius: 50%;
          cursor: pointer;
          font-weight: 500;
          font-size: 16px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
          &.hover {
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
          }
        }
      }
    }
    .update-content {
      margin-bottom: 24px;
      
      .update-notes {
        h3 {
          font-size: 16px;
          margin: 0 0 12px 0;
          font-weight: 500;
        }
        
        .notes-content {
          background-color: #f9f9f9;
          border-radius: 6px;
          padding: 12px;
          max-height: 200px;
          overflow-y: auto;
          line-height: 1.6;
          font-size: 14px;
          white-space: pre-wrap;
          
          &.empty {
            color: #999;
            font-style: italic;
          }
        }
      }
    }
  }
</style>