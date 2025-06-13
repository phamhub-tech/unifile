import type { IApiResponse } from "./types"

export function getApiMessage(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  err: any,
  message = 'An error occured. Please try again later.',
): string {
  let msg = message;
  
  if ((err as IApiResponse).message) msg = message
  else if (typeof err === 'string') msg = err

  if (msg === message) console.error(err)
  return msg
}