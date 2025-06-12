import { defineStore } from "pinia"

import { TApiStatus, getApiMessage } from "@/core/api"

import { DriveModel } from "./models/drive"
import { homeService } from "./service"

interface IState {
	getDrivesApiStatus: TApiStatus
	getDrivesApiMsg: string
	allDrives: DriveModel[] | null

	drive: DriveModel | null
}
const state = (): IState => ({
	getDrivesApiStatus: TApiStatus.default,
	getDrivesApiMsg: '',
	allDrives: null,

	drive: null
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

				const { data } = await homeService.getDrives()
				this.allDrives = data.map((drive) => DriveModel.fromJson(drive))

				this.getDrivesApiStatus = TApiStatus.success
			} catch (e) {
				this.getDrivesApiStatus = TApiStatus.error
				this.getDrivesApiMsg = getApiMessage(e)
			}
		},

		async getDrive(mountPath: string) {
			const drives = this.allDrives
			if (drives === null) return

			this.drive = drives.find((d) => d.mountPoint === mountPath) ?? null
		},
	}
})
