import { api } from "@/core/api"

import type { IDriveJson } from "./models/drive"

export const homeService = {
	getDrives() {
		return api.invoke<IDriveJson[]>('get_drives')
	}
}
