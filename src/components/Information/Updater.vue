<script setup lang="ts">
import type { PropType } from 'vue';
import {NTag} from 'naive-ui'
import { VersionComparation } from '../composables';
defineProps({
  version: {
    type: Object as PropType<VersionComparation>,
    default: true,
  },
});

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
        
        <div class="update-content">
          <div class="update-notes">
            <h3>更新内容</h3>
            <div class="notes-content" v-if="version.note">
              <div v-html="version.note.replace(/\n/g, '<br>')"></div>
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
    padding: 16px;
    max-width: 500px;
    
    .update-header {
      margin-bottom: 20px;
      
      h2 {
        margin: 0 0 12px 0;
        font-size: 20px;
        font-weight: 600;
      }
      
      .version-tags {
        display: flex;
        gap: 8px;
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