import type { IAppSettings, IAppSettingsJson, IScanSettings, IScanSettingsJson, TTheme } from "./types";

export * from './types';

export class ScanSettingsModel implements IScanSettings {
	public ignorePatterns: string[];
	public useGitignore: boolean;
	constructor(data: IScanSettings) {
		this.ignorePatterns = data.ignorePatterns
		this.useGitignore = data.useGitignore
	}

	static buildData(json: IScanSettingsJson): IScanSettings {
		return {
			ignorePatterns: json.ignore_patterns,
			useGitignore: json.use_gitignore,
		}
	}

	static fromJson(json: IScanSettingsJson): ScanSettingsModel {
		return new ScanSettingsModel(this.buildData(json));
	}

	toJson(): IScanSettingsJson {
		return {
			ignore_patterns: this.ignorePatterns,
			use_gitignore: this.useGitignore,
		}
	}
}

export class AppSettings implements IAppSettings {
	public theme: TTheme;
	public language: string;
	public scan: IScanSettings;
	constructor(data: IAppSettings) {
		this.theme = data.theme;
		this.language = data.language;
		this.scan = data.scan;
	}

	static fromJson(json: IAppSettingsJson): AppSettings {
		let theme: TTheme;
		switch (json.theme) {
			case 'system':
				theme = 'system';
				break;
			case 'dark':
				theme = 'dark';
				break;
			default:
				theme = 'light'
		}

		return new AppSettings({
			theme,
			language: json.language,
			scan: ScanSettingsModel.fromJson(json.scan)
		})
	}

	toJson(): IAppSettingsJson {
		const scan = this.scan;
		return {
			theme: this.theme,
			language: this.language,
			scan: {
				ignore_patterns: scan.ignorePatterns,
				use_gitignore: scan.useGitignore,
			}
		}
	}
}

