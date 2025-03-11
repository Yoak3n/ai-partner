<script lang="ts" setup>
import Header from './Header.vue'
import Body from './Body.vue'
import { useRoute } from 'vue-router';
let heigth = 30
const $route = useRoute()


const checkIsDialog = ():boolean=>{
    if ($route.name === 'Dialog'){
        return false
    }else{
        return true
    }
}
</script>

<template>
    <div class="container">
        <Header :heigth="heigth" v-if="checkIsDialog()"/>
        <Body :top="checkIsDialog()? heigth:0"/>
    </div>
</template>