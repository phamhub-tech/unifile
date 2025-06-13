import { ref } from 'vue'
import { describe, it } from 'vitest'

import { TApiStatus } from '../types'

import { useApiHandle } from '.'

describe.concurrent('Api Handle', () => {
  it('returns expected api status', ({ expect }) => {
    const status = ref(TApiStatus.default)
    const apiHandle = useApiHandle(status)

    status.value = TApiStatus.loading
    expect(apiHandle.isLoading.value).true

    status.value = TApiStatus.success
    expect(apiHandle.isSuccess.value).true

    status.value = TApiStatus.error
    expect(apiHandle.isError.value).true
  })
})
