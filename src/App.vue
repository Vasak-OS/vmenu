<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useConfigStore } from "@vasakgroup/plugin-config-manager";

import SearchComponent from "@/components/SearchComponent.vue";
import MenuSection from "@/components/section/MenuSection.vue";
import FilterSection from "@/components/section/FilterSection.vue";
import UserInfo from "@/components/UserInfo.vue";
import SessionButton from "@/components/button/SessionButton.vue";
import CategoryPill from "@/components/button/CategoryPill.vue";
import WeatherWidget from "@/components/widget/WeatherWidget.vue";

// Importar iconos de sesión
import shutdownImg from "@/assets/img/shutdown.svg";
import rebootImg from "@/assets/img/reboot.svg";
import logoutImg from "@/assets/img/logout.svg";
import suspendImg from "@/assets/img/suspend.svg";

// Estado global
const menuData = ref<Array<any>>([]);
const categorySelected = ref<any>("all");
const filter = ref<string>("");
const configStore = useConfigStore();
let unlistenConfig: Function | null = null;

// Cargar menú
const setMenu = async () => {
  try {
    menuData.value = await invoke("get_menu_items");
  } catch (error) {
    console.error("Error al cargar el menú:", error);
  }
};

const logout = async () => {
  await invoke("logout");
};

const shutdown = async () => {
  await invoke("shutdown");
};

const reboot = async () => {
  await invoke("reboot");
};

const suspend = async () => {
  await invoke("suspend");
};

const apps = computed(() => {
  const allApps = (menuData.value as any)["all"]?.apps;
  return allApps;
});

const appsOfCategory = computed(
  () => (menuData.value as any)[categorySelected.value]?.apps
);

onMounted(async () => {
  setMenu();
  configStore.loadConfig();
  unlistenConfig = await listen("config-changed", async () => {
    configStore.loadConfig();
  });
});

onUnmounted(() => {
  if (unlistenConfig !== null) {
    unlistenConfig();
  }
});
</script>

<template>
  <div
    class="vmenu background"
  >
    <!-- Header Section -->
    <div class="flex items-center justify-between animate-fadeIn">
      <UserInfo />

      <SearchComponent
        v-model:filter="filter"
        class="w-1/3 mx-4 transition-all duration-300 focus-within:w-2/5"
      />

      <div class="flex items-center space-x-2">
        <SessionButton
          v-for="(action, index) in [
            { title: 'Shutdown', img: shutdownImg, handler: shutdown },
            { title: 'Reboot', img: rebootImg, handler: reboot },
            { title: 'Logout', img: logoutImg, handler: logout },
            { title: 'Suspend', img: suspendImg, handler: suspend },
          ]"
          :key="index"
          :title="action.title"
          :img="action.img"
          @click="action.handler"
          class="w-8 h-8 hover:bg-white/5 rounded-full p-1.5 transition-colors transform hover:scale-110 hover:rotate-3 transition-all duration-200"
        />
      </div>
    </div>

    <!-- Main Content -->
    <template v-if="filter !== ''">
      <FilterSection v-model:apps="apps" v-model:filter="filter" />
    </template>
    <template v-else>
      <div class="grid grid-cols-3 gap-6 p-6 animate-slideUp">
        <!-- Apps -->
        <div
          class="bg-white/50 dark:bg-black/50 rounded-xl p-4 h-[calc(100vh-7rem)] overflow-y-auto"
        >
          <MenuSection v-model:apps="appsOfCategory" />
        </div>

        <!-- Weather Widget -->
        <div class="space-y-4">
          <WeatherWidget
            class="bg rounded-xl p-4 transform hover:-translate-y-1 transition-transform duration-200"
          />
        </div>

        <!-- Categories -->
        <div>
          <div class="flex flex-wrap flex-row justify-center">
            <CategoryPill
              v-for="(value, key) in menuData"
              :key="key"
              :category="key"
              :image="value.icon"
              :description="value.description"
              v-model:categorySelected="categorySelected"
              :class="{
                'bg-white/50 dark:bg-black/50': categorySelected === key,
              }"
              class="transform hover:translate-y-1 transition-transform duration-200"
            />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style>
@reference "./style.css";

.vmenu {
  @apply h-screen p-4;
  border-radius: calc(var(--border-radius) + 16px);
}
/* Animaciones personalizadas */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fadeIn {
  animation: fadeIn 0.5s ease-out;
}

.animate-slideUp {
  animation: slideUp 0.5s ease-out;
}

.session-button img {
  @apply brightness-0 invert opacity-75 hover:opacity-100 transition-opacity;
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>
