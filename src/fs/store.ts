import { defineStore } from "pinia"

import { TApiStatus, getApiMessage } from "@/core/api"

import { DriveModel } from "./models/drive"
import { fsService } from "./service"
import FileSystemEntryModel from "./models/entry";

interface IState {
	getDrivesApiStatus: TApiStatus;
	getDrivesApiMsg: string;
	allDrives: DriveModel[] | null;

	drive: DriveModel | null;

	/*
	 * Contains entries that have been requested
	 *
	 * It is an object where the keys are paths. This is usefull for caching
	 */
	entries: Record<string, FileSystemEntryModel[]>;

	getEntriesApiStatus: TApiStatus;
	getEntriesApiMsg: string;
}
const state = (): IState => ({
	getDrivesApiStatus: TApiStatus.default,
	getDrivesApiMsg: '',
	allDrives: null,

	drive: null,

	getEntriesApiStatus: TApiStatus.default,
	getEntriesApiMsg: '',
	entries: {},
})

export const useFSStore = defineStore('home', {
	state,
	getters: {
		drives(): DriveModel[] | null {
			return this.allDrives?.filter((drive) => drive.name.trim() !== 'none') ?? null
		}
	},
	actions: {
		async getDrives() {
			try {
				this.getDrivesApiStatus = TApiStatus.loading
				this.getDrivesApiMsg = '';

				const { data } = await fsService.getDrives()
				this.allDrives = data.map((drive) => DriveModel.fromJson(drive))

				this.getDrivesApiStatus = TApiStatus.success
			} catch (e) {
				this.getDrivesApiStatus = TApiStatus.error
				this.getDrivesApiMsg = getApiMessage(e)
			}
		},
		async getDrive(mountPath: string) {
			if (this.allDrives === null) {
				await this.getDrives();
				if (this.allDrives == null) return;
			}

			const drives = this.allDrives
			this.drive = drives!.find((d) => d.mountPoint === mountPath) ?? null
		},

		async getEntries(path: string, useCache = true) {
			if (useCache && this.entries[path] !== undefined) return;

			try {
				this.getEntriesApiStatus = TApiStatus.loading
				this.getEntriesApiMsg = ''

				const { data } = await fsService.getEntries(path)
				this.entries[path] = data.map((datum) => FileSystemEntryModel.fromJson(datum))

				this.getEntriesApiStatus = TApiStatus.success
			} catch (e) {
				this.getEntriesApiStatus = TApiStatus.error
				this.getEntriesApiMsg = getApiMessage(e)
			}
		},
	}
})
