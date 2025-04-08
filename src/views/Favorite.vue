<template>
  <div class="favorite-wrapper">
    <div class="favorite-header">
      <h1>我的收藏</h1>
    </div>
    
    <div v-if="favoriteMessages.length === 0" class="empty-state">
      <n-empty description="暂无收藏消息" />
    </div>
    
    <div v-else class="favorite-list">
      <div v-for="(group, date) in groupedMessages" :key="date" class="message-group">
        <n-divider>{{ date }}</n-divider>
        <div class="favorite-item" v-for="message in group" :key="message.id">
          <div class="message-content">
            <div class="message-header">
              <span class="message-time">{{ formatTimeOnly(message.message_id) }}</span>
              <span class="message-model">{{ message.model }}</span>
            </div>
            <div class="message-text">
              <n-ellipsis line-clamp="1" style="max-width: 90%;" :tooltip="false">
                {{ message.content }}
              </n-ellipsis>
            </div>
          </div>
          <div class="message-actions">
            <n-button size="small" @click="viewDetail(message)">查看详情</n-button>
            <n-button size="small" type="error" @click="removeFavorite(message.message_id)">移除</n-button>
          </div>
          </div>

      </div>
    </div>
    
    <div class="pagination-container"  v-if="totalPages > 1">
      <n-pagination 
        v-model:page="currentPage" 
        :page-count="totalPages" 
        :page-size="pageSize"
        @update:page="handlePageChange" 
        class="pagination" 
        :show-size-picker="false"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted,h, onBeforeUnmount, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {listen, UnlistenFn} from '@tauri-apps/api/event'
import { NEmpty, NButton, NPagination,NDivider,NEllipsis } from 'naive-ui';

import { FavoriteMessage } from '../types';
import { formatTimeOnly,debounce } from '../utils';
import FavoriteDetail from '../components/Modal/FavoriteDetail.vue';


const favoriteMessages = ref<FavoriteMessage[]>([]);
const currentPage = ref(1);
const pageSize = ref(10);
const totalPages = ref(1);
const groupedMessages = ref<Record<string, FavoriteMessage[]>>({});
let unlistenRefresh:UnlistenFn;
onMounted(() => {
  loadFavoriteMessages();
  window.addEventListener('resize', debouncedHandleResize);
  listen('refresh_favorite',()=>{
    loadFavoriteMessages();
  }).then((fn)=>unlistenRefresh = fn)
});
onBeforeUnmount(() => {
  window.removeEventListener('resize', debouncedHandleResize);
  if(unlistenRefresh){
    unlistenRefresh();
  }
});

const handleResize = () => {
  const oldPageSize = pageSize.value;
  calculatePageSize();
  if (oldPageSize !== pageSize.value) {
    groupMessagesByDate();
  }
};
const debouncedHandleResize = debounce(handleResize, 200);
const calculatePageSize = () => {
  const viewportHeight = window.innerHeight;
  const itemHeight = 120; // 假设平均高度为120px
  const availableHeight = viewportHeight - 200;
  const calculatedSize = Math.max(5, Math.floor(availableHeight / itemHeight));
  
  const oldPageSize = pageSize.value;
  const oldCurrentPage = currentPage.value;

  pageSize.value = calculatedSize;

  if (favoriteMessages.value.length > 0) {
    totalPages.value = Math.ceil(favoriteMessages.value.length / pageSize.value);
    if (currentPage.value > totalPages.value) {
      currentPage.value = totalPages.value;
    }
    if (oldPageSize !== pageSize.value || oldCurrentPage !== currentPage.value) {
      groupMessagesByDate();
    }
  }
};
const currentPageMessages = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return favoriteMessages.value.slice(start, end);
});
const loadFavoriteMessages = async() => {
    try{
        const messages:FavoriteMessage[] = await invoke('get_favorites')
        favoriteMessages.value = messages;
        totalPages.value = Math.ceil(messages.length / pageSize.value);
        calculatePageSize();
        groupMessagesByDate();
    }catch(e){
        window.$message.error('获取收藏消息失败:'+ e);
    }
};


const viewDetail = (msg: FavoriteMessage) => {
  window.$modal.create({
    title: '消息详情',
    content: () => h(FavoriteDetail, { message: msg ,style:"max-height:80%"}),
    contentStyle:{
      maxHeight:'80%',
      overflow:'auto'
    },
    preset:'card',
    maskClosable: true,
  })
};

const removeFavorite = (message_id: number) => {
  try {
    invoke('remove_favorite', { messageId:message_id }).then(()=>{
      favoriteMessages.value = favoriteMessages.value.filter(msg => msg.message_id !== message_id);
      window.$message.success('已取消收藏',{duration:2000})
      totalPages.value = Math.ceil(favoriteMessages.value.length / pageSize.value);
      if (currentPageMessages.value.length === 0 && currentPage.value > 1) {
        currentPage.value--;
      }
      groupMessagesByDate();
    });
  } catch (e) {
    window.$message.error('移除收藏失败:'+ e);
    return;
  }

};
const groupMessagesByDate = () => {
  const grouped: Record<string, FavoriteMessage[]> = {};
  
  currentPageMessages.value.forEach(msg => {
    const dateStr = formatDate(msg.message_id);
    if (!grouped[dateStr]) {
      grouped[dateStr] = [];
    }
    grouped[dateStr].push(msg);
  });
  
  // 对每个日期组内的消息按时间排序（从新到旧）
  Object.keys(grouped).forEach(date => {
    grouped[date].sort((a, b) => b.message_id - a.message_id);
  });
  
  // 对日期键进行排序（从新到旧）
  groupedMessages.value = Object.keys(grouped)
    .sort((a, b) => {
      const dateA = new Date(a.replace('年', '/').replace('月', '/').replace('日', ''));
      const dateB = new Date(b.replace('年', '/').replace('月', '/').replace('日', ''));
      return dateB.getTime() - dateA.getTime();
    })
    .reduce((obj, key) => {
      obj[key] = grouped[key];
      return obj;
    }, {} as Record<string, FavoriteMessage[]>);
};

const formatDate = (timestamp: number): string => {
  const date = new Date(timestamp);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  });
};

const handlePageChange = (page: number) => {
  currentPage.value = page;
  window.scrollTo({ top: 0, behavior: 'smooth' });
  groupMessagesByDate();
};
</script>

<style scoped lang="less">
.favorite-wrapper {
  padding: 20px;
  box-sizing: border-box;
  margin: 0 auto;
  height: 100%;
  .favorite-header {
    margin-bottom: 24px;
    
    h1 {
      font-size: 24px;
      font-weight: 600;
      color: #333;
    }
  }
  
  .empty-state {
    margin-top: 60px;
    text-align: center;
  }
  
  .favorite-list {
    .favorite-item {
      padding: 16px;
      border-radius: 8px;
      background-color: #fff;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
      margin-bottom: 16px;
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      
      .message-content {
        flex: 1;
        
        .message-header {
          display: flex;
          align-items: center;
          margin-bottom: 8px;
          
          .message-time {
            color: #666;
            font-size: 14px;
            margin-right: 12px;
          }
          
          .message-model {
            background-color: #f0f0f0;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 12px;
          }
        }
        
        .message-text {
          color: #333;
          line-height: 1.5;
          max-height: 100px;
          overflow: hidden;
          text-overflow: ellipsis;
          display: -webkit-box;
          -webkit-line-clamp: 3;
          -webkit-box-orient: vertical;
        }
      }
      
      .message-actions {
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin-left: 16px;
      }
    }
  }
  
  .pagination-container {
    margin-top: 24px;
    width: 100%;
    height: 50px; 
    position: fixed;
    bottom: 20px;
    z-index: 10;
    .pagination {
      display: flex;
      justify-content: center;
      width: 90%;
      background-color: rgba(255, 255, 255, 0.9);
      padding: 10px 0;
      border-radius: 8px;
      box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.05);
    }
  }
  

}
</style>