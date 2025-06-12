import { api } from "~/core/api";

import type { IAppSettingsJson } from "./models/settings"

export const settingsService = {
	getSettings() {
		return api.invoke<IAppSettingsJson>("get_settings");
	},
	saveSettings(settings: IAppSettingsJson) {
		return api.invoke<string>("save_settings", { newSettings: settings })
	},
	resetSettings() {
		return api.invoke<string>("reset_settings")
	}
}
