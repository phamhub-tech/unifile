import { check } from '@tauri-apps/plugin-updater';

import { api } from "~/core/api";

import type { IAppSettingsJson } from "./models/settings"
import { humanizeBytes } from '~/core/utils';

export const settingsService = {
	getSettings() {
		return api.invoke<IAppSettingsJson>("get_settings");
	},
	saveSettings(settings: IAppSettingsJson) {
		return api.invoke<string>("save_settings", { newSettings: settings })
	},
	resetSettings() {
		return api.invoke<string>("reset_settings")
	},

	// Handle updates
	async checkUpdate() {
		const update = await check();
		if (!update) {
			console.log('App is up to date');
			return;
		}

		await update.downloadAndInstall((event) => {
			switch (event.event) {
				case 'Started':
					console.log('Started downloading: ', humanizeBytes(event.data.contentLength ?? 0));
					break;
				case 'Finished':
					console.log('Donwload finished');
					break;
			}
		})

		console.log('Update installed');
	}
}
