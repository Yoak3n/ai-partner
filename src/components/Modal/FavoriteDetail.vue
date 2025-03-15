<script lang="ts" setup>
import type{ PropType } from 'vue';
import { FavoriteMessage } from '../../types';
import MarkdownRender from '../Markdown/index.vue'
defineProps({
    message: {
        type: Object as PropType<FavoriteMessage>,
        required: true
    }
})
const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
    });
};

</script>


<template>
    <div class="message-detail">
        <div class="detail-item">
          <span class="detail-label">时间:</span>
          <span class="detail-value">{{ formatTime(message.message_id) }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">模型:</span>
          <span class="detail-value">{{ message.model }}</span>
        </div>
        <div class="detail-item detail-content">
          <span class="detail-label">内容:</span>
          <div class="detail-value content">
            <MarkdownRender :source="message.content"/>
        </div>
        </div>
      </div>
</template>

<style scoped lang="less">
.message-detail {
  padding: 10px;
  
  .detail-item {
    margin-bottom: 16px;
    
    .detail-label {
      font-weight: 500;
      display: block;
      margin-bottom: 8px;
      color: #666;
    }
    
    .detail-value {
      &.content {
        line-height: 1.6;
        background-color: #f9f9f9;
        padding: 12px;
        border-radius: 6px;
        max-height: 300px;
        overflow-y: auto;
        
        &.reasoning {
          background-color: #f0f7ff;
          border-left: 3px solid #1890ff;
        }
      }
    }
  }
}
</style>