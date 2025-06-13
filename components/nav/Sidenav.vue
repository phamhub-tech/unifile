<template>
  <aside class="flex w-14 flex-col gap-y-3 bg-gray-50 p-2 dark:bg-gray-800">
    <div
      v-for="(group, index) of routes"
      :key="`sidenav-link-group-${index}`"
      :class="['space-y-3', { 'md:!mt-auto': group.isBottom }]"
    >
      <NuxtLinkLocale
        v-for="r of group.routes"
        :key="`link-${r.name}`"
        :to="r.route"
        active-class="bg-primary/20 font-medium"
        :class="[
          'grid-centered block aspect-square border border-transparent',
          'hover:bg-primary/10 overflow-hidden rounded transition-all',
        ]"
      >
        <component :is="r.icon" class="w-6 shrink-0" />
      </NuxtLinkLocale>
    </div>

    <div class="mt-auto">
      <ThemeButton
        v-slot="{ icon }"
        :class="[
          'grid-centered mt-auto block aspect-square border border-transparent',
          'hover:bg-primary/10 overflow-hidden rounded transition-all',
        ]"
      >
        <component :is="icon" class="w-6 shrink-0" />
      </ThemeButton>
      <p class="text-[.6rem] text-muted-foreground text-center">{{ appInfo?.version }}</p>
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

import { HomeIcon } from "../icons/two-tone";
import { getRouteFromName } from "~/core/utils";
import { useSettingsStore } from "~/src/settings/store";

const store = useSettingsStore();
const { appInfo } = storeToRefs(store);

interface IRoute {
  icon: Component;
  name: string;
  route: RouteLocationRaw;
  useExact?: boolean;
}
interface IGroupedRoute {
  title?: string;
  isBottom?: boolean;
  routes: IRoute[];
}

// const i18n = useI18n();
const routes: IGroupedRoute[] = [
  {
    routes: [
      {
        icon: HomeIcon,
        name: "Explorer",
        route: getRouteFromName("drives"),
      },
    ],
  },
];
</script>
