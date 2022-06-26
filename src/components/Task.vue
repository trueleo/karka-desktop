<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import Action from './Action.vue';
import Attribute from './Attribute.vue';

defineProps({
    actions: {type: Array, required: true},
    attributes: {type: Array, required: true}
})

</script>

<template>
    <div class="h-fit w-auto border-4 border-dashed rounded-lg border-slate-900 grid grid-flow-row-dense gap-6 grid-cols-5 grid-rows-1 px-4 py-3">
        <div class="col-span-3 flex flex-col justify-start gap-y-2">
            <Action v-for="action in actions" v-bind="action" />
        </div> 
        <div class="col-span-2 flex flex-wrap justify-start content-start gap-x-3 gap-y-1"> 
            <Attribute v-for="attribute in attributes" v-bind="attribute" />
        </div> 
    </div>
</template>

<style scoped>
</style>