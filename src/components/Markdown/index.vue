<template>
    <div style="render" v-html="displayHtml" />
</template>

<script lang="ts" setup>
import MarkdownIt from 'markdown-it'
import MarkdownItHighlightjs from 'markdown-it-highlightjs'
import { computed,toRefs } from 'vue'
// import {addCopyButtons} from '../../utils/index'
const markdown=  new MarkdownIt().use(MarkdownItHighlightjs)
const defaultRender = markdown.renderer.rules.link_open || function(tokens:any, idx:any, options:any, _:any, self:any) {
  return self.renderToken(tokens, idx, options);
};

markdown.renderer.rules.link_open = function (tokens:any, idx:any, options:any, env:any, self:any) {
  // 添加 target="_blank" 属性
  const aIndex = tokens[idx].attrIndex('target');
  if (aIndex < 0) {
    tokens[idx].attrPush(['target', '_blank']); // 添加新属性
  } else {
    tokens[idx].attrs[aIndex][1] = '_blank';    // 替换现有属性值
  }

  // 添加 rel="noopener noreferrer" 以增强安全性
  const relIndex = tokens[idx].attrIndex('rel');
  if (relIndex < 0) {
    tokens[idx].attrPush(['rel', 'noopener noreferrer']);
  } else {
    tokens[idx].attrs[relIndex][1] = 'noopener noreferrer';
  }

  // 调用默认的渲染方法
  return defaultRender(tokens, idx, options, env, self);
};
const props = defineProps({
    source:{
        type:String,
        default:''
    }
})
let { source } = toRefs(props)
const displayHtml = computed(()=>{
    try{
        const m = markdown.render(source.value)
        return m
    }catch(err){
        console.log(err)
    }
})
// onUpdated(()=>{
//     addCopyButtons()
// })

</script>

<style lang="less" >
div{
    margin: 0;
    padding: 0;
    height: auto;

}

p{
    margin-block-start: 0;
    margin-block-end: 0;
}
pre code {
    display: block;
    padding: 15px;
    background-color: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.5;
    color: #333;
    
}


.hljs-keyword {
    color: #d73a49; /* 红色 */
    font-weight: bold;
}

/* 字符串 */
.hljs-string {
    color: #032f62; /* 蓝色 */
}

/* 注释 */
.hljs-comment {
    color: #6a737d; /* 灰色 */
    font-style: italic;
}

/* 数字 */
.hljs-number {
    color: #005cc5; /* 蓝色 */
}

/* 函数名 */
.hljs-title {
    color: #6f42c1; /* 紫色 */
}

/* 标签名 */
.hljs-tag {
    color: #22863a; /* 绿色 */
}

/* 属性名 */
.hljs-attr {
    color: #e36209; /* 橙色 */
}

/* 其他 */
.hljs-built_in,
.hljs-selector-class,
.hljs-selector-id {
    color: #6f42c1; /* 紫色 */
}


</style>