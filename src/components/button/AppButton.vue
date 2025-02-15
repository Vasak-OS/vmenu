<script lang="ts" setup>
import { defineProps, ref, onMounted, computed } from 'vue';
import { getIcon, getImageType } from "@/common/icons";

const props = defineProps({
  app: {
    type: Object,
    required: true
  }
});

const appIcon = ref(props.app.icon);

const openApp = async () => {
  await $vsk.openApp(props.app.path);
  $vsk.exit();
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
  <button :title="app.name" @click="openApp()">
    <img :src="iconBase64" class="h-10 m-2" />
    <span style="display: none">{{ app.name }}</span>
    <span style="display: none">{{ app.description }}</span>
    <span style="display: none">{{ app.keywords }}</span>
  </button>
</template>
