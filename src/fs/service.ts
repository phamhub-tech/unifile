import { api } from "@/core/api"

import type { IDriveJson } from "./models/drive"
import type { IFileSystemEntryJson } from "./models/entry";

export const fsService = {
	getDrives() {
		return api.invoke<IDriveJson[]>('get_drives')
	},

	getEntries(path: string) {
		return api.invoke<IFileSystemEntryJson[]>('get_entries', { path });
	},
}
