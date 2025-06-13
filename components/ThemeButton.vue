<template>
  <component :is="as" variant="ghost" size="icon" @click="toggle">
    <slot :icon="icon">
      <component :is="icon" />
    </slot>
  </component>
</template>

<script setup lang="ts">
import { MonitorIcon, MoonIcon, SunIcon } from "lucide-vue-next";

import { useSettingsStore } from "~/src/settings/store";
import type { TTheme } from "~/src/settings/models/settings";

import { Button } from "./ui/button";

withDefaults(defineProps<{ as?: string | Component }>(), { as: Button });

const store = useSettingsStore();
const { settings } = storeToRefs(store);

const icon = computed(() => {
  // const pref = colorMode.preference;
  const pref = settings.value!.theme;

  let icon = MonitorIcon;
  if (pref === "light") icon = SunIcon;
  else if (pref === "dark") icon = MoonIcon;

  return icon;
});

function toggle() {
  const pref = settings.value!.theme;

  let mode: TTheme = "light";
  if (pref === "light") mode = "dark";
  else if (pref === "dark") mode = "system";

  store.setTheme(mode);
  // console.log('Pref', pref, mode, store.setTheme)
}
</script>
