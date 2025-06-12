export enum TApiStatus {
	default,
	loading,
	success,
	error
}

export interface IApiResponse<T = null> {
	error: boolean
	message: string
	data: T
}

export type TApiErrorData = Record<string, string[]> | null;
export interface IApiError {
	error: true
	message: string
	data?: TApiErrorData
}

export type TSortType = 'asc' | 'desc' | undefined;