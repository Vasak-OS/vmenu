<script lang="ts" setup>
import { computed } from 'vue';
import AppButton from '@/components/button/AppButton.vue';

const props = defineProps({
  apps: {
    type: Array,
    required: true
  },
  filter: {
    type: String,
    required: true
  }
});

const appsFiltred = computed((): Array<any> => {
  return props.apps.filter(
    (app: any) =>
      app.name.toLowerCase().includes(props.filter) || app.description.toLowerCase().includes(props.filter)
  );
});
</script>

<template>
  <div class="hydriam-search-results">
    <AppButton v-for="app in appsFiltred" :key="app.name" :app="app" />
  </div>
</template>
