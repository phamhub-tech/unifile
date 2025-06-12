<template>
  <div class="space-y-2">
    <h1 class="px- font-semibold">{{ $t("drives", 2) }}</h1>
    <div class="grid grid-cols-3 gap-4 xl:grid-cols-5">
      <RouterLink
        v-for="drive of drives"
        :key="`drive-${drive.name}`"
        v-slot="{ navigate }"
        :to="
          getRoute({
            name: 'drive-details',
            params: {
              drivePath: drive.mountPoint,
              path: pathSep,
            },
          })
        "
        custom
      >
        <div
          :class="[
            'hover:bg-primary/10 border',
            'flex gap-x-2 rounded-lg p-2 transition-colors',
          ]"
          role="button"
          @click="navigate"
        >
          <DiskIcon class="shrink-0" :size="40" :stroke-width="1" />
          <div class="flex-1 space-y-2 truncate">
            <p class="truncate">
              {{ drive.name }}
              <span class="font-medium">({{ drive.mountPoint }})</span>
            </p>
            <LinearProgress
              :value="(drive.used / drive.total) * 100"
              :info="
                $t('availableOutOf', {
                  available: humanizeBytes(drive.available),
                  total: humanizeBytes(drive.total),
                })
              "
              class="bg-white"
            />
          </div>
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import { HardDriveIcon as DiskIcon } from "lucide-vue-next";
import { LinearProgress } from "~/components/progress";

import { pathSep } from "~/core/constants";
import { getRoute, humanizeBytes } from "~/core/utils";
import { useFSStore } from "~/src/fs/store";

definePageMeta({ name: "home" });

const store = useFSStore();
const { drives } = storeToRefs(store);
store.getDrives();
</script>
