import { getApiMessage, TApiStatus } from '~/core/api'

import { AppSettings, type TTheme } from './models/settings'
import { AppInfo } from './models/app-info'
import { settingsService } from './service'

export interface ILanguage {
	locale: string
	name: string
	countryCode: string
}
const languages: ILanguage[] = [
	{
		locale: 'en',
		name: 'english',
		countryCode: 'GB',
	},
]

interface IState {
	appInfo: AppInfo | null;
	languages: ILanguage[];
	activeLanguage: ILanguage | null;

	settingsApiStatus: TApiStatus;
	settingsApiMsg: string;
	settings: AppSettings | null;

	saveSettingsApiStatus: TApiStatus;
	saveSettingsApiMsg: string;
}

const state = (): IState => {
	return {
		appInfo: null,
		languages,
		activeLanguage: null,

		settingsApiStatus: TApiStatus.default,
		settingsApiMsg: '',
		settings: null,

		saveSettingsApiStatus: TApiStatus.default,
		saveSettingsApiMsg: '',
	}
}

export const useSettingsStore = defineStore('settings', {
	state,
	actions: {
		// setLanguage(language: ILanguage) {
		// 	this.activeLanguage = language
		// 	this.saveLocalSettings()

		// 	document.documentElement.setAttribute('lang', language.locale)
		// },

		async init() {
			const info = new AppInfo();
			await info.build();
			this.appInfo = info;

			await this.getSettings()
		},

		setTheme(theme: TTheme) {
			const settings = this.settings!;
			settings.theme = theme;
			this.saveSettings(settings)
		},

		async getSettings() {
			if (this.settings !== null) return;

			try {
				this.settingsApiStatus = TApiStatus.loading;
				this.settingsApiMsg = '';

				const { data } = await settingsService.getSettings()
				this.settings = AppSettings.fromJson(data);
				this._syncTheme();

				this.settingsApiStatus = TApiStatus.success;
			} catch (e) {
				this.settingsApiStatus = TApiStatus.error;
				this.settingsApiMsg = getApiMessage(e)
			}
		},
		async saveSettings(settings: AppSettings) {
			try {
				this.saveSettingsApiStatus = TApiStatus.loading
				this.saveSettingsApiMsg = '';

				await settingsService.saveSettings(settings.toJson())
				this.settings = settings;
				this._syncTheme();

				this.saveSettingsApiStatus = TApiStatus.success
			} catch (e) {
				this.saveSettingsApiStatus = TApiStatus.error
				this.saveSettingsApiMsg = getApiMessage(e);
			}
		},

		resetSettings() {
			settingsService.resetSettings()
		},

		/**
		 * Syncs the theme. Should not be called outside this store
		 */
		_syncTheme() {
			const theme = this.settings?.theme;
			if (theme === null) return;

			if (theme === 'dark') {
				document.documentElement.classList.add('dark');
			} else {
				document.documentElement.classList.remove('dark');
			}
		},

		reset() {
			this.$reset()
		},
	},
})
