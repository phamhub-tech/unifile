import type { IApiResponse } from "./types"

export function getApiMessage(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  err: any,
  message = 'An error occured. Please try again later.',
): string {
  const msg = (err as IApiResponse).message ?? message

  if (msg === message) console.error(err)
  return msg
}