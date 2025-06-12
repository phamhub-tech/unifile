<template>
  <div v-if="drive !== null">
    <section class="shadow-primary/10 pt-2 bg-background px-4 py-4">
      <h1>
        {{ drive.name }}
        <span class="text-muted-foreground mt-1 mb-2">
          ({{ drive.mountPoint }})
        </span>
      </h1>
      <LinearProgress
        :value="(drive.used / drive.total) * 100"
        :info="
          $t('availableOutOf', {
            available: humanizeBytes(drive.available),
            total: humanizeBytes(drive.total),
          })
        "
      />
    </section>
    <Explorer :drive="drive" />
  </div>
</template>

<script setup lang="ts">
import { watch } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";

import { humanizeBytes } from "~/core/utils";
import { useFSStore } from "~/src/fs/store";
import { LinearProgress } from "~/components/progress";

import Explorer from "~/src/fs/components/Explorer.vue";

definePageMeta({ name: "drive-details" });

const store = useFSStore();
const { drives, drive } = storeToRefs(store);
watch(drives, getDrive);

const route = useRoute();

getDrive();
function getDrive() {
  const path = route.params.drivePath as string;
  store.getDrive(path);
}
</script>
