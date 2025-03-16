<script lang="ts" setup>
import type{ PropType } from 'vue';

import { NEllipsis } from 'naive-ui';

import { formatTime } from '../../utils';
import { FavoriteMessage } from '../../types';
import MarkdownRender from '../Markdown/index.vue'
defineProps({
    message: {
        type: Object as PropType<FavoriteMessage>,
        required: true
    }
})


</script>


<template>
    <div class="message-detail">
        <div class="detail-item">
          <span class="detail-label">时间:</span>
          <span class="detail-value">{{ formatTime(message.message_id) }}</span>
        </div>
        <div class="detail-item detail-content">
          <span class="detail-label">内容:</span>
          <div class="detail-value content">
            <div class="reasoning-section" v-if="message.reasoning_content != null && message.reasoning_content != ''">
              <span class="detail-label">思考过程:</span>
              <div class="reasoning-content">
                <n-ellipsis style="max-width: 95%" line-clamp="2" expand-trigger="click" :tooltip="false">
                  <MarkdownRender :source="message.reasoning_content"/>
                </n-ellipsis>
    
              </div>
            </div>
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
        max-height: 500px;
        overflow-y: auto;
        font-size: 16px;
        .reasoning-section {
          border-left: 3px solid #1890ff;
          border-radius: 6px;
          padding: 12px;
          font-size: 14px;
          background-color: #fafafa;
          .reasoning-content{
            font-size: 14px;
            color: #595959;
            line-height: 1.5;
            background-color: #f0f7ff;
            padding: 10px;
            border-radius: 4px;
          }
        }
      }
    }
  }
}
</style>