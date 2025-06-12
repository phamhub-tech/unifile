export interface IScanSettingsJson {
	ignore_patterns: string[];
	use_gitignore: boolean;
}

export interface IAppSettingsJson {
	scan: IScanSettingsJson
}

export interface IScanSettings {
	ignorePatterns: string[];
	useGitignore: boolean;
}
export interface IAppSettings {
	scan: IScanSettings;
}
