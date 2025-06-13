<template>
  <div
    v-bind="attrsToBind"
    :class="
      cn(
        'bg-white dark:bg-white/[0.025]',
        $attrs.class as string | undefined,
      )
    "
  >
    <main :class="twMerge('relative overflow-x-auto', bodyClass)">
      <table class="styled w-full text-left">
        <thead :key="`key-${headerRerenderTriggerKey}`">
          <tr>
            <th
              v-for="(header, i) in headers"
              :key="`header-${i}`"
              @click="sortHeader(header)"
            >
              <div
                :class="
                  twMerge(
                    'smallcaps flex items-center gap-x-1 font-medium text-sm tracking-wider',
                    header.sort !== undefined && 'cursor-pointer select-none',
                    (contentAlignment(header.align) === 'center' ||
                      (header.isDate &&
                        contentAlignment(header.align) === undefined)) &&
                      'justify-center text-center',
                    contentAlignment(header.align) === 'right' &&
                      'justify-end text-right',
                    header.classes,
                  )
                "
              >
                <slot name="header" v-bind="{ header }">
                  {{ header.label }}
                </slot>
                <MoveUpIcon
                  v-if="header.sort === 'asc'"
                  class="inline-block h-5 w-5 shrink-0 stroke-2"
                />
                <MoveDownIcon
                  v-else-if="header.sort === 'desc'"
                  class="inline-block h-5 w-5 shrink-0 stroke-2"
                />
              </div>
            </th>
            <th v-if="hasItemActions">Actions</th>
          </tr>
        </thead>
        <tbody ref="itemsList">
          <tr v-if="filteredItems.length === 0">
            <td
              :colspan="headers.length"
              class="text-muted-foreground !pt-8 text-center"
            >
              {{ noItemsText ?? $t("noItems") }}
            </td>
          </tr>
          <template v-else>
            <tr
              v-for="(item, i) of filteredItems"
              :key="`item-${i}`"
              :class="[
                'border-y border-slate-200 dark:border-slate-200/10',
                { 'last:border-y-transparent': !hasFooter },
                isItemSelected && isItemSelected(item)
                  ? 'bg-primary/10 text-primary-dark'
                  : isItemClickable(item) && 'hover:bg-primary/5',
              ]"
              @click="isItemClickable(item) && onItemClick!(item, $event)"
            >
              <td
                v-for="(header, j) of headers"
                :key="`item-value-${j}`"
                :class="[
                  {
                    'text-center':
                      contentAlignment(header.align) == 'center' ||
                      (header.isDate &&
                        contentAlignment(header.align) === undefined),
                    'text-right': contentAlignment(header.align) == 'right',
                  },
                  typeof header.itemClass === 'string' ||
                  header.itemClass === undefined
                    ? header.itemClass
                    : header.itemClass(item),
                  itemClass,
                ]"
              >
                <slot
                  name="list-item-value"
                  v-bind="{
                    key: header.key,
                    value: header.ignoreValue
                      ? null
                      : getItemValue(item, header.value, header.isDate),
                    item,
                    isSelected: isItemSelected && isItemSelected(item),
                  }"
                >
                  {{ getItemValue(item, header.value, header.isDate) }}
                </slot>
              </td>
              <td v-if="hasItemActions" class="text-center">
                <slot name="item-actions" v-bind="{ item }" />
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </main>
  </div>
</template>

<script setup lang="ts" generic="T">
import { MoveUpIcon, MoveDownIcon } from "lucide-vue-next";
import { twMerge } from "tailwind-merge";

import { humanizeDate, removeFromAttrs } from "~/core/utils";
import { cn } from "~/lib/utils";

import type { IHeader, ISortField, TDataTableAlignment } from "./types";
import type { TSortType } from "~/core/api";

defineOptions({ inheritAttrs: false });
const props = withDefaults(
  defineProps<{
    headers: IHeader<T>[];
    items: T[];
    noItemsText?: string;
    title?: string;
    searchPlaceholder?: string;
    itemClass?: string;
    isPaginationLoading?: boolean;
    searchDelay?: number;
    sortDelay?: number;
    hasGridLayout?: boolean;
    hasHeader?: boolean;
    hasFooter?: boolean;
    nullValue?: string;
    align?: TDataTableAlignment;
    isRefreshing?: boolean;
    bodyClass?: string;
    inputClass?: string;
    isItemSelected?: (item: T) => boolean;
    canClickItem?: (item: T) => boolean;
    onItemClick?: (item: T, event: MouseEvent) => void;
    onRefresh?: () => void;
  }>(),
  {
    searchPlaceholder: "Search item",
    hasHeader: true,
    hasFooter: true,
    sortDelay: 800,
    searchDelay: 800,
    nullValue: "-",
  },
);

//! Do not use the new emits syntax until https://github.com/vuejs/language-tools/issues/3431 is fixed
const emit = defineEmits<{
  (e: "sort", field: ISortField): void;
}>();

const attrs = useAttrs();
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const attrsToBind = computed<Record<string, any>>(() => {
  return removeFromAttrs(attrs, "class");
});

function contentAlignment(
  headerAlignment?: TDataTableAlignment,
): TDataTableAlignment | undefined {
  return headerAlignment ?? props.align;
}

function isItemClickable(item: T): boolean {
  if (props.canClickItem) return props.canClickItem(item);
  return !!props.onItemClick;
}

const slots = useSlots();
const hasItemActions = ref(false);
let observer: MutationObserver | null = null;
const itemsList = ref<HTMLElement | null>(null);
onMounted(() => {
  observer?.disconnect();
  hasItemActions.value = slots["item-actions"] !== undefined;

  observer = new MutationObserver(() => {
    hasItemActions.value = slots["item-actions"] !== undefined;
  });
  observer.observe(itemsList.value!, {
    childList: true,
    subtree: true,
  });
});
onUnmounted(() => observer?.disconnect);

const filteredItems = computed(() => {
  return props.items;
});

const headerRerenderTriggerKey = ref(true);
let sortTimeout: number | null = null;
function sortHeader(header: IHeader<T>) {
  if (sortTimeout) clearTimeout(sortTimeout);
  const mode = header.sort;
  if (mode === undefined) return;

  let sortMode: TSortType | undefined = "desc";
  if (mode === "desc") sortMode = "asc";
  else if (mode === "asc") sortMode = undefined;

  header.sort = sortMode;
  headerRerenderTriggerKey.value = !headerRerenderTriggerKey.value;
  sortTimeout = window.setTimeout(
    () =>
      emit("sort", {
        name: (header.key ?? header.value) as string,
        value: sortMode,
      }),
    props.sortDelay ?? 0,
  );
}

function getItemValue(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  item: any,
  key: IHeader<T>["value"],
  formatAsDate?: boolean,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): any {
  if (typeof key === "function") {
    const value = key(item as T);
    return value ?? props.nullValue;
  }

  const split = key.toString().split(".");
  if (split.length === 1) {
    const returnValue: string | number | Date | null = item[key];
    if (returnValue === null || returnValue.toString().trim() === "")
      return props.nullValue;

    if (formatAsDate)
      return humanizeDate(returnValue as Date, false)
        .split(" ")
        .map((value) => value.replace(/^(\w)/, (c) => c.toUpperCase()))
        .join(" ");

    return returnValue ?? props.nullValue;
  }

  return getItemValue(
    item[split[0] as string],
    split.slice(1).join("."),
    formatAsDate,
  );
}
</script>

<style lang="scss" scoped>
th {
  @apply whitespace-nowrap;
}

.dt-actions button:not(.not-action) {
  @apply cursor-pointer rounded p-2 hover:bg-slate-200 dark:hover:bg-slate-50/10;

  svg {
    @apply h-6 w-6;
  }
}
</style>
