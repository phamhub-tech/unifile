import { api } from "@/core/api"

import type { IDriveJson } from "./models/drive"
import type { IFileSystemEntryJson } from "./models/entry";
import type { ScanEvent } from "./types";
import { Channel } from "@tauri-apps/api/core";

export const fsService = {
	getDrives() {
		return api.invoke<IDriveJson[]>('get_drives')
	},
	getEntries(path: string) {
		return api.invoke<IFileSystemEntryJson[]>('get_entries', { path });
	},

	scanPath(path: string, onUpdate: (event: ScanEvent) => void) {
		const onEvent = new Channel<ScanEvent>();
		onEvent.onmessage = (message) => {
			onUpdate(message);
		}

		return api.invoke('scan_path', { path, onEvent })
	}
}
