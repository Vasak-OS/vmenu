<script lang="ts" setup>
import AppCard from '@/components/card/AppCard.vue';

defineProps({
  apps: {
    type: Array<any>,
    required: true
  }
});
</script>

<template>
  <div class="flex flex-wrap flex-col items-center" >
    <AppCard v-for="app in apps" :key="app.name" :app="app" />
  </div>
</template>
