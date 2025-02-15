<script lang="ts" setup>
import { defineProps, computed, defineEmits, ref, onMounted } from 'vue';
import { getIcon, getImageType } from "@/common/icons";

const emit = defineEmits(['update:categorySelected']);

const props = defineProps({
  category: {
    type: String,
    required: true
  },
  image: {
    type: String,
    required: true
  },
  description: {
    type: String,
    required: true
  },
  categorySelected: {
    type: String,
    required: true
  }
});

const appIcon = ref(props.image);

const setCategory = (category: string) => {
  emit('update:categorySelected', category);
};

const isActive = computed(() => {
  return props.category === props.categorySelected ? 'active' : '';
});

const getAppIcon = async () => {
  appIcon.value = await getIcon(props.image);
};

const iconBase64 = computed(() => {
  return `data:${getImageType(appIcon.value)};base64,${appIcon.value}`;
});

onMounted(() => {
  getAppIcon();
});
</script>

<template>
  <button :class="isActive" class="p-2 rounded-lg hover:scale-120 transition-all duration-300" @click="setCategory(category)">
    <img :src="iconBase64" :title="description" :alt="category" class="h-12" />
  </button>
</template>
