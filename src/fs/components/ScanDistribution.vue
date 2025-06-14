<template>
  <div
    v-if="scanEntries"
    class="bg-background mt-8 space-y-4 rounded-md border p-3 dark:bg-gray-800"
  >
    <div class="">
      <h2 class="text-sm font-medium">{{ $t("usedSpaceDistribution") }}</h2>
      <p class="">
        {{
          $t("xOfY", {
            x: humanizeBytes(scanSize),
            y: humanizeBytes(drive.used),
          })
        }}
      </p>
    </div>
    <ProgressLinearProgress
      :value="scanPercent"
      :info="$t('usedSpacePercent', { percent: Math.floor(scanPercent) })"
    />

    <ProgressCompoundLinearProgress
      :values="distribution.map((d) => (d.size / scanSize) * 100)"
      :duration="800"
      :colors="distribution.map((d) => getHexForFileType(d.type).dark)"
    >
      <template #legend="{ index, percentage }">
        <div class="flex-1 space-y-0.5">
          <div class="flex">
            <p>{{ $t(`fileTypes.${distribution[index].type}`, 2) }}</p>
            <p class="ml-auto">{{ percentage }}%</p>
          </div>
          <p class="text-xs">
            {{ humanizeBytes(distribution[index].size) }}
            <span>•</span>
            {{ distribution[index].count }}
          </p>
        </div>
      </template>
    </ProgressCompoundLinearProgress>

    <div class="text-muted-foreground truncate text-xs">
      <p>{{ currentFileName }}</p>
      <p>{{ currentFilePath }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { humanizeBytes } from "~/core/utils";

import type { DriveModel } from "../models/drive";
import { useFSStore } from "../store";
import type { TFileType } from "../models/entry";
import { getHexForFileType } from "../utils";

const props = defineProps<{ drive: DriveModel }>();

const store = useFSStore();
const { scanEntries, currentFilePath, currentFileName } = storeToRefs(store);

const scanSize = computed<number>(() => {
  return Object.values(scanEntries.value!).reduce(
    (size, scanEntry) => size + scanEntry.totalSize,
    0,
  );
});
const scanPercent = computed<number>(
  () => (scanSize.value / props.drive.used) * 100,
);

interface IDistribution {
  type: TFileType;
  size: number;
  count: number;
}
const distribution = computed<IDistribution[]>(() => {
  const dis: Record<string, { size: number; count: number }> = {};
  for (const entry of Object.values(scanEntries.value!)) {
    const fileType = entry.fileType;
    if (fileType === null) continue; // This is a folder. Ignore

    const d: { size: number; count: number } | undefined = dis[fileType];
    if (d !== undefined) {
      d.size += entry.totalSize;
      d.count += 1;
      continue;
    }

    dis[fileType] = {
      size: entry.totalSize,
      count: 1,
    };
  }

  return Object.entries(dis)
    .map(([fileType, stat]) => ({
      type: fileType as TFileType,
      ...stat,
    }))
    .sort((a, b) => b.size - a.size);
});
</script>
