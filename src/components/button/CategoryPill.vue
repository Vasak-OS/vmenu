<script lang="ts" setup>
import { defineProps, computed } from 'vue';

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

const setCategory = (category: string) => {
  emit('update:categorySelected', category);
};

const isActive = computed(() => {
  return props.category === props.categorySelected ? 'active' : '';
});
</script>

<template>
  <button :class="isActive" @click="setCategory(category)">
    <img :src="'file://' + image" :title="description" :alt="category" />
  </button>
</template>
