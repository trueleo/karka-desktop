<script setup>
import Attribute from './Attribute.vue';
defineProps({
    actions: Array,
    attributes: Array
})
</script>

<template>
<div class="flex justify-start flex-wrap content-start gap-2 border p-2"> 
    <div v-for="action in actions" class="border px-3 py-2"> 
        {{action}}
    </div>
    <div class="h-0.5 w-full bg-white my-4"></div>
    <Attribute v-for="attribute in attributes" v-bind="attribute" />
</div>
</template>

<style scoped></style>