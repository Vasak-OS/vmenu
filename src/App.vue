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
    <div class="flex items-center justify-between animate-fadeIn mb-4">
      <UserInfo />

      <SearchComponent
        v-model:filter="filter"
        class="search-component"
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
          class="session-button"
        />
      </div>
    </div>

    <!-- Main Content -->
    <template v-if="filter !== ''">
      <FilterSection v-model:apps="apps" v-model:filter="filter" />
    </template>
    <template v-else>
      <div class="grid grid-cols-3 gap-4 animate-slideUp h-[calc(100vh-88px)]">
        <!-- Apps -->
        <div
          class="background rounded-vsk p-4 h-full overflow-y-auto"
        >
          <MenuSection v-model:apps="appsOfCategory" />
        </div>

        <!-- Weather Widget -->
        <div class="rounded-vsk background p-4 space-y-4 h-full overflow-y-auto">
          <WeatherWidget />
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
                'background': categorySelected === key,
              }"
              class="transform hover:translate-y-1"
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

.search-component {
  @apply w-2/5 border-b-2 border-vsk-primary;
}

.session-button {
  @apply w-10 h-10 hover:bg-vsk-primary/30 rounded-vsk p-1 transform hover:scale-110 hover:rotate-3;
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
