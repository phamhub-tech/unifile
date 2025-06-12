import type { IAppSettings, IAppSettingsJson, IScanSettings, IScanSettingsJson } from "./types";

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
	public scan: IScanSettings;
	constructor(data: IAppSettings) {
		this.scan = data.scan;
	}

	static fromJson(json: IAppSettingsJson): AppSettings {
		return new AppSettings({
			scan: ScanSettingsModel.fromJson(json.scan)
		})
	}

	toJson(): IAppSettingsJson {
		const scan = this.scan;
		return {
			scan: {
				ignore_patterns: scan.ignorePatterns,
				use_gitignore: scan.useGitignore,
			}
		}
	}
}

