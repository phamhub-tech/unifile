export interface IDriveJson {
	name: string,
	total_bytes: number,
	used_bytes: number,
	available_bytes: number,
	mount_point: string,
	is_removable: boolean,
	drive_type: 'ssd' | 'hdd' | 'uknown',
}

export enum IDriveType {
	Hdd = 'HDD',
	Ssd = 'SSD',
	Uknown = 'Unknown',
}
export interface IDrive {
	name: string,

	/**
	 * Drive size in bytes
	 */
	total: number,

	/**
	 * Drive space used in bytes
	 */
	used: number,

	/**
	 * Free drive space in bytes
	 */
	available: number,

	mountPoint: string,
	isRemovable: boolean,
	type: IDriveType,
}
