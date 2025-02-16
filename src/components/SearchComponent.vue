<script lang="ts" setup>
import { defineProps, defineEmits } from 'vue';

const vFocus = {
  mounted: (el: HTMLElement) => el.focus()
};

defineProps({
  filter: {
    type: String,
    required: true
  }
});

const emit = defineEmits(['update:filter']);
</script>

<template>
  <div>
    <input
      type="text"
      class="form-control rounded-xs"
      placeholder="Search"
      aria-label="Search"
      id="search"
      v-focus
      :value="filter"
      @input="emit('update:filter', ($event.target as any).value)"
    />
  </div>
</template>
