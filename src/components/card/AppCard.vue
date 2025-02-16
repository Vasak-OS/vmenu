<script lang="ts" setup>
import { defineProps, ref, onMounted, computed } from 'vue';
import { getIcon, getImageType } from "@/common/icons";
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  app: {
    type: Object,
    required: true
  }
});

const appIcon = ref(props.app.icon);

const openApp = async (path: string) => {
  try {
    await invoke('open_app', { path });
    await $vsk.exit(); // Cierra el launcher después de abrir la app
  } catch (error) {
    console.error('Error al abrir la aplicación:', error);
  }
};

const getAppIcon = async () => {
  appIcon.value = await getIcon(props.app.icon);
};

const iconBase64 = computed(() => {
  return `data:${getImageType(appIcon.value)};base64,${appIcon.value}`;
});

onMounted(() => {
  getAppIcon();
});
</script>

<template>
  <button class="flex flex-row w-full p-2 items-center" @click="openApp(app.path)" :title="app.description">
    <img :src="iconBase64" class="img-fluid h-10" />
    <div class="col-10 app-card-info ps-2">
      {{ app.name }}
      <span class="text-muted" style="display: none">{{ app.description }}</span>
      <span style="display: none">{{ app.keywords }}</span>
    </div>
  </button>
</template>
