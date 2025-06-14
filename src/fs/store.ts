import { defineStore } from "pinia"

import { TApiStatus, getApiMessage } from "@/core/api"

import { DriveModel } from "./models/drive"
import { fsService } from "./service"
import FileSystemEntryModel from "./models/entry";
import type { IScanEntry } from "./models/scan/types";
import type { StyleValue } from "vue";

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

	scanPathApiStatus: TApiStatus;
	scanPathApiMsg: string;
	scanEntries: Record<string, IScanEntry> | null;
	currentFileName: string | null;
	currentFilePath: string | null;
	
	/**
	 * Styles that should be passed to main.
	 * 
	 * This is very volatile and very experimental.  Only meant to be used with useVirtualList
	 */
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	mainStyle: any | null;
}
const state = (): IState => ({
	getDrivesApiStatus: TApiStatus.default,
	getDrivesApiMsg: '',
	allDrives: null,

	drive: null,

	getEntriesApiStatus: TApiStatus.default,
	getEntriesApiMsg: '',
	entries: {},

	scanPathApiStatus: TApiStatus.default,
	scanPathApiMsg: '',
	scanEntries: null,
	currentFileName: null,
	currentFilePath: null,

	mainStyle: null,
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
		async scanPath(path: string) {
			try {
				this.scanPathApiStatus = TApiStatus.loading;
				this.scanPathApiMsg = '';
				this.clearScan();

				console.profile('Scan')
				// let lastUpdateMs = 0;
				// const throttleMs = 20000;
				this.scanEntries = {};
				const scanEntries: typeof this.scanEntries = {};
				// const scanEntries = { ...toRaw(this.scanEntries) };
				// const scanEntries = new Map(Object.entries({ ...toRaw(this.scanEntries) }));
				fsService.scanPath(path, (event) => {
					if (event.event === 'progress') {
						// const scanEntries = this.scanEntries!;
						// const scanEntries = Object.assign({}, toRaw(this.scanEntries ?? {}));
						// const scanEntries = { ...toRaw(this.scanEntries) };
						const entry = FileSystemEntryModel.fromJson(event.data.entry);
						const scanEntry: IScanEntry | undefined = scanEntries[entry.name];
						// const scanEntry = scanEntries.get(entry.name);

						if (scanEntry) {
							scanEntry.totalSize += entry.size;
							scanEntry.duplicates.push(entry);
							// scanEntries[scanEntry.name] = scanEntry;
							// scanEntries.set(scanEntry.name, scanEntry);
						} else {
							scanEntries[entry.name] = {
								name: entry.name,
								totalSize: entry.size,
								fileType: entry.fileType,
								// Set this entry as a duplicate since when showing duplicates this entry should be included
								duplicates: [entry],
							};
							// scanEntries.set(entry.name, {
							// 	name: entry.name,
							// 	totalSize: entry.size,
							// 	fileType: entry.fileType,
							// 	// Set this entry as a duplicate since when showing duplicates this entry should be included
							// 	duplicates: [entry],
							// })
						}

						this.currentFilePath = entry.path;
						this.currentFileName = entry.name;

						// const now = Date.now();
						// if (now - lastUpdateMs > throttleMs) {
						// 	// 	// this.scanEntries = Object.fromEntries(scanEntries);
						// 	this.scanEntries = scanEntries;
						// 	lastUpdateMs = now;
						// }
					}

					if (event.event === 'finished') {
						this.currentFilePath = null;
						this.currentFileName = null;
						this.scanEntries = scanEntries;
						// this.scanEntries = Object.fromEntries(scanEntries);

						this.scanPathApiStatus = TApiStatus.success;
						console.profileEnd('Scan')
					}
				});

				// this.scanPathApiStatus = TApiStatus.success;
			} catch (e) {
				this.scanPathApiStatus = TApiStatus.error;
				this.scanPathApiMsg = getApiMessage(e);
				console.error(e);
			}
		},

		clearScan(reset = true) {
			// this.scanSize = 0;
			this.mainStyle = null;

			if (reset) {
				this.scanEntries = null;
				return
			}

			this.scanEntries = {}
		},

		setMainStyle(style: StyleValue | null) {
			this.mainStyle = style;
		},
	}
})
