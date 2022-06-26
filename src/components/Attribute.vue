<script setup>
defineProps({
    type: String,
    value: String
});
</script>

<template>
    <div class="w-fit h-fit relative mt-[0.7rem]">
        <div class="w-fit h-fit absolute -translate-x-1 -translate-y-[0.7rem] right-0 text-sm px-1 bg-slate-800 ">
            {{ type }}
        </div>
        <div class="w-fit pt-1.5 px-2 text-lg border">
            {{ value }}
        </div>
    </div>
</template>

<style scoped>
</style>