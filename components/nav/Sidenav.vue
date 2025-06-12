<template>
  <aside class="w-14 p-2">
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
          'grid-centered block border border-transparent aspect-square',
          'hover:bg-primary/10 overflow-hidden rounded transition-all',
        ]"
      >
        <component :is="r.icon" class="w-6 shrink-0" />
      </NuxtLinkLocale>
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

import { HomeIcon } from "../icons/two-tone";
import { getRouteFromName } from "~/core/utils";

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
        name: "Home",
        route: getRouteFromName("home"),
      },
    ],
  },
];
</script>
