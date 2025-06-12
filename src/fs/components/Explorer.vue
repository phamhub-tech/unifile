<template>
  <section class="sticky bg-background -top-[1px] z-10 border-y pr-4">
    <div class="flex items-center gap-x-2 overflow-x-auto">
      <template
        v-for="({ name, path, isRoot }, i) of pathSplit"
        :key="`path-${i}`"
      >
        <NuxtLink
          v-slot="{ navigate }"
          :to="
            isRoot
              ? getRouteFromName('drives')
              : getRoute({
                  name: 'drive-details',
                  params: { drivePath: drive.mountPoint, path },
                })
          "
          custom
        >
          <div
            :class="[
              'px-4 py-3 text-sm whitespace-nowrap opacity-80 hover:opacity-100',
              'hover:bg-primary/10 transition-colors outline-none',
              'focus:bg-primary/10',
            ]"
            role="button"
            tabindex="0"
            @click="
              () => {
                clearScan();
                navigate();
              }
            "
          >
            <RootIcon v-if="isRoot" :size="20" />
            <p v-else>{{ name }}</p>
          </div>
          <ChevronRightIcon
            v-if="i < pathSplit.length - 1"
            class="shrink-0"
            :size="16"
          />
        </NuxtLink>
      </template>
    </div>
  </section>

  <TableDataTable
    align="left"
    class="border-t-0 select-none"
    :has-header="false"
    :items="entries"
    :headers="[
      {
        label: $t('name'),
        value: 'name',
        key: 'name',
        itemClass: 'w-[1%] whitespace-nowrap max-w-xl !pr-10 overflow-ellipsis',
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
          type === TFileSystemEntryType.file ? humanizeBytes(size) : '',
        align: 'right',
        itemClass: 'opacity-60',
      },
    ]"
    :no-items-text="$t('folderEmpty')"
    :show-selection-checkbox="false"
    :is-item-selected="({ path }) => selectedEntries.has(path)"
    @item-click="selectEntry"
  >
    <template
      #list-item-value="{ key, item: { type, extension, name, fileType } }"
    >
      <template v-if="key === 'type'">
        <p v-if="type === TFileSystemEntryType.folder">
          {{ $t("folders") }}
        </p>
        <p v-else-if="type === TFileSystemEntryType.file">
          {{ $t("extFileType", { ext: extension.toUpperCase() }) }}
        </p>
      </template>

      <div v-if="key === 'name'" class="flex items-center gap-x-2 truncate">
        <component
          :is="
            type === TFileSystemEntryType.folder
              ? FolderIcon
              : getIconForFileType(fileType!)
          "
          :size="20"
          :class="[
            'shrink-0',
            type === TFileSystemEntryType.folder
              ? 'fill-yellow-300 stroke-yellow-500'
              : null,
          ]"
          :style="
            type === TFileSystemEntryType.file
              ? getStylesForFileType(fileType!)
              : null
          "
        />
        <p class="truncate">{{ name }}</p>
      </div>
    </template>
  </TableDataTable>
</template>

<script setup lang="ts">
import {
  ChevronRightIcon,
  FolderIcon,
  MonitorIcon as RootIcon,
  ScanLineIcon as ScanIcon,
  Trash2Icon as ClearIcon,
} from "lucide-vue-next";

import { pathSep } from "~/core/constants";
import { getRoute, getRouteFromName, humanizeBytes } from "~/core/utils";

import type { DriveModel } from "../models/drive";
import { useFSStore } from "../store";
import type FileSystemEntryModel from "../models/entry";
import { TFileSystemEntryType } from "../models/entry";
import { getIconForFileType, getStylesForFileType } from "../utils";

const props = defineProps<{ drive: DriveModel }>();

const store = useFSStore();
const { entries: allEntries } = storeToRefs(store);

const route = useRoute();
watch(route, getEntries);

const currentPath = computed<string>(() => {
  let relPath = route.params.path as string;
  if (relPath.startsWith(pathSep)) relPath = relPath.substring(1);

  return `${props.drive.mountPoint}${pathSep}${relPath}`;
});

interface IPathSplit {
  name: string;
  path: string;
  isRoot: boolean;
}
const pathSplit = computed<IPathSplit[]>(() => {
  const path = currentPath.value;
  const drive = props.drive;
  const drivePath = drive.mountPoint;
  const split = path.split(drivePath).filter((s) => !!s.trim());

  const paths: IPathSplit[] = [
    { name: "root", path: pathSep, isRoot: true },
    { name: drive.name, path: pathSep, isRoot: false },
  ];
  if (split.length === 1) {
    const names = split[0].split(pathSep).filter((s) => !!s.trim());
    let basePath = "";
    for (const name of names) {
      const path: IPathSplit = {
        name,
        path: `${basePath}${pathSep}${name}`,
        isRoot: false,
      };
      paths.push(path);

      basePath = path.path;
    }
  }

  return paths;
});

const entries = computed<FileSystemEntryModel[]>(() => {
  const es = allEntries.value[currentPath.value] ?? [];

  // Always ensure folders come first
  return es.sort(({ type: typeA }, { type: typeB }) => {
    if (typeA === typeB) return 0;
    if (typeA === TFileSystemEntryType.folder) return -1;

    return 1;
  });
});

getEntries();
function getEntries() {
  const path = currentPath.value;
  store.getEntries(path);
}

const selectedEntries = ref(new Set<string>());
let timeout: number | null = null;
const clicks = ref(0);
async function selectEntry(
  entry: FileSystemEntryModel,
  event: MouseEvent | KeyboardEvent,
) {
  clicks.value++;
  event.preventDefault();
  clearSelectTimeout();

  if (event instanceof KeyboardEvent || clicks.value === 2) {
    await browseEntry(entry);
    selectedEntries.value.delete(entry.path);
    clicks.value = 0;
    return;
  }
  
  // For now assume we can't do multi entries
  selectedEntries.value.clear();

  // Assume this is a selection
  selectedEntries.value.add(entry.path);

  // if (!isSelectingMultiple.value) {
  //   selectedEntries.value.clear();
  // }

  // Reset the clicks after some time so that double clicks should happen within a short interval
  timeout = window.setTimeout(() => {
    clicks.value = 0;
  }, 400);
}
function clearSelectTimeout() {
  if (timeout) clearTimeout(timeout);
}

async function browseEntry(entry: FileSystemEntryModel) {
  if (entry.isFile) return;

  const drivePath = props.drive.mountPoint;
  const path = entry.path.replace(drivePath, "");
  // router.push({ name: 'drive-details', params: { drivePath: drivePath, path } })
  return navigateTo(
    getRoute({
      name: "drive-details",
      params: {
        drivePath: encodeURIComponent(drivePath),
        path: encodeURI(path),
      },
    }),
  );
}

function clearScan() {}
</script>
