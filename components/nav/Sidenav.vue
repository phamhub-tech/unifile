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

    <div class="mt-auto space-y-1">
      <ThemeButton
        v-slot="{ icon }"
        :class="[
          'grid-centered mt-auto block aspect-square border border-transparent',
          'hover:bg-primary/10 overflow-hidden rounded transition-all',
        ]"
      >
        <component :is="icon" class="w-6 shrink-0" />
      </ThemeButton>
      <div class="relative flex items-center justify-center gap-x-1">
        <p class="text-muted-foreground text-center text-[.6rem]">
          {{ appInfo?.version }}
        </p>
        <AppTooltip v-if="updateApiStatus === TApiStatus.error" side="right">
          <AlertIcon class="size-4 stroke-red-600 dark:stroke-red-400" />

          <template #tooltip>
            <p class="text-destructive font-medium">{{ $t("updateError") }}</p>
            <p>{{ updateApiMsg }}</p>
          </template>
        </AppTooltip>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { AlertTriangleIcon as AlertIcon } from "lucide-vue-next";
import type { RouteLocationRaw } from "vue-router";

import { HomeIcon } from "../icons/two-tone";
import { getRouteFromName } from "~/core/utils";
import { useSettingsStore } from "~/src/settings/store";
import { TApiStatus } from "~/core/api";

const store = useSettingsStore();
const { appInfo, updateApiStatus, updateApiMsg } = storeToRefs(store);

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
