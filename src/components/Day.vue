<script setup>
</script>

<template>
    <div id="days" class="flex flex-row justify-center gap-x-7 text-xl py-3">
        <div>yesterday</div>
        <div class="active">today</div>
        <div>tommorow</div>
  </div>
</template>

<style scoped>

#days div {
    opacity: 0.4;
    color: white;
}

.active {
    opacity: 1 !important;
}

</style>