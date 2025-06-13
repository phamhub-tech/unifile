import type { TSortType } from "~/core/api"

export type TDataTableAlignment = 'left' | 'center' | 'right'
export interface IHeader<T> {
	label: string;
	value: (keyof T | (string & {})) | ((item: T) => string | number | null);
	sort?: TSortType | null;
	classes?: string;
	itemClass?: string | ((item: T) => string);
	isDate?: boolean;
	align?: TDataTableAlignment;

	/**
	 * Used to uniquely identify this header
	 */
	key?: string;

	/**
	 * Used to show whether the value for this header should
	 * be computed
	 */
	ignoreValue?: boolean;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export interface ISortField<T = any> {
	name: keyof T;
	value: TSortType | undefined;
}
