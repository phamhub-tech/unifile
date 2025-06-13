<template>
  <div class="">
    <div class="space-y-3">
      <p class="text-muted-foreground">
        {{ $t("duplicatesFound", entriesToShow.length) }}
      </p>

      <Accordion type="multiple">
        <AccordionItem
          v-for="entry of entriesToShow"
          :key="`entry-header-${entry.name}`"
          :value="entry.name"
        >
          <AccordionTrigger
            class="flex items-center justify-between gap-x-2 py-2"
          >
            <div class="flex items-center gap-x-2">
              <component
                :is="getIconForFileType(entry.fileType)"
                :style="getStylesForFileType(entry.fileType)"
                class="size-6 shrink-0"
              />
              <p class="truncate">{{ entry.name }}</p>
            </div>
            <p class="ml-auto text-sm opacity-60">
              {{ humanizeBytes(entry.totalSize) }}
              ({{ entry.duplicates.length }})
            </p>
          </AccordionTrigger>
          <AccordionContent class="pl-4">
            <TableDataTable
              align="left"
              class="border-t-0 select-none"
              :has-header="false"
              :items="entry.duplicates"
              :headers="[
                {
                  label: $t('name'),
                  value: 'name',
                  key: 'name',
                  itemClass:
                    'w-[1%] whitespace-nowrap max-w-xl !pr-10 overflow-ellipsis',
                },
                {
                  label: $t('path'),
                  value: 'path',
                },
                {
                  label: $t('modified'),
                  value: 'modified',
                  isDate: true,
                  itemClass: 'opacity-60',
                },
                {
                  label: $t('type'),
                  value: 'type',
                  key: 'type',
                  itemClass: 'opacity-60',
                },
                {
                  label: $t('size'),
                  value: ({ size, type }) =>
                    type === TFileSystemEntryType.file
                      ? humanizeBytes(size)
                      : '',
                  align: 'right',
                  itemClass: 'opacity-60',
                },
              ]"
            >
              <template #list-item-value="{ key, item: { type, extension } }">
                <template v-if="key === 'type'">
                  <p v-if="type === TFileSystemEntryType.folder">
                    {{ $t("folders") }}
                  </p>
                  <p v-else-if="type === TFileSystemEntryType.file">
                    {{ $t("extFileType", { ext: extension.toUpperCase() }) }}
                  </p>
                </template>
              </template>
            </TableDataTable>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  </div>
</template>

<script setup lang="ts">
import { humanizeBytes } from "~/core/utils";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "~/components/ui/accordion";

import type { IScanEntry } from "../models/scan/types";
import { getIconForFileType, getStylesForFileType } from "../utils";
import { TFileSystemEntryType } from "../models/entry";

const props = defineProps<{ scanEntries: Record<string, IScanEntry> }>();

const entriesToShow = computed(() =>
  Object.values(props.scanEntries)
    .filter((entry) => entry.duplicates.length > 1)
    .sort((a, b) => b.totalSize - a.totalSize),
);
</script>
