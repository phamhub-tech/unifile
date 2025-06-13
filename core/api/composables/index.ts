import { computed, type ComputedRef, type Ref } from 'vue'

import { TApiStatus } from '@/core/api'

export interface IApiHandle {
  isLoading: ComputedRef<boolean>
  isSuccess: ComputedRef<boolean>
  isError: ComputedRef<boolean>
}

export function useApiHandle(status: Ref<TApiStatus>): IApiHandle {
  return {
    isLoading: computed<boolean>(() => status.value === TApiStatus.loading),
    isSuccess: computed<boolean>(() => status.value === TApiStatus.success),
    isError: computed<boolean>(() => status.value === TApiStatus.error),
  }
}
