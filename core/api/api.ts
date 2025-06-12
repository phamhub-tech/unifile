import { listen } from '@tauri-apps/api/event'
import type { EventCallback } from '@tauri-apps/api/event'

import type { IApiResponse } from './types'

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function logObject(title: string, obj?: Record<string, any>): void {
	if (!obj) return console.log(`${title}: ${obj}`)

	console.groupCollapsed(title)
	for (const [key, param] of Object.entries(obj)) {
		console.log(`${key}: `, param)
	}

	console.groupEnd()
}

const shouldLogTraffic = true
class Api {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	private _logRequest(command: string, args?: any) {
		if (shouldLogTraffic) {
			console.groupCollapsed(`Request  (${command})`)

			logObject('Args', args)

			console.groupEnd()
		}
	}
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	private _logResponse(command: string, response: IApiResponse<any>) {
		console.groupCollapsed(`Response (${command})`)
		console.log('Data: ', response)
		console.groupEnd()
	}

	/**
	 * Sends a message to the backend
	 */
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	async invoke<T>(command: string, args?: Record<string, any>): Promise<IApiResponse<T>> {
		this._logRequest(command, args)

		const { invoke } = await import('@tauri-apps/api/core')
		const response: IApiResponse<T> = await invoke(command, args)

		this._logResponse(command, response)

		if (response.error) throw new ApiError(response)
		return response
	}

	/**
	 * Registers a listener
	 *
	 * Returns a function that unlistens to the event when called
	 */
	async listen<T>(event: string, handler: EventCallback<T>): Promise<VoidFunction> {
		console.log(`Subscribing to event: ${event}`)

		const unlistener = await listen<T>(event, handler)
		return () => {
			console.log(`Unsubscribing from event: ${event}`)
			unlistener()
		}
	}
}

export class ApiError<T> extends Error {
	response: IApiResponse<T>

	constructor(response: IApiResponse<T>, message?: string,) {
		super(message ?? response.message);

		this.response = response;
	}
}

export const api = new Api()
