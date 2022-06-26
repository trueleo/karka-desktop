<script setup>
const props = defineProps({
    action: String,
    argument: String,
    summary: String
});

const hasMargin = computed(()=>({
    hasmargin: props.summary
}))

import { invoke } from '@tauri-apps/api/tauri';
import { computed } from 'vue';
</script>

<template>
    <div class="relative" :class="hasMargin">
        <p class=" bg-slate-800 px-1 w-fit \
        absolute right-0 top-0 \
        text-sm \
        -translate-y-[0.7rem] -translate-x-2 \
        ">
        {{ summary }}
        </p>
        <div
            class="rounded-sm w-full h-fit \
            flex justify-start items-center gap-4 \
            border  \
            px-3 py-2"
        >
            <span id="action" class="w-fit h-fit text-center ">{{ action }}</span>
            <span id="right" class="w-fit h-fit">{{ argument }}</span>
        </div>
    </div>
</template>

<style scoped>
#action {
    flex-basis: 6rem;
}

.hasmargin {
    margin-top: 0.7rem;
}

</style>