export interface IScanSettingsJson {
	ignore_patterns: string[];
	use_gitignore: boolean;
}

export type TTheme = "light" | "dark" | "system"

export interface IAppSettingsJson {
	theme: string;
	language: string;
	scan: IScanSettingsJson;
}

export interface IScanSettings {
	ignorePatterns: string[];
	useGitignore: boolean;
}
export interface IAppSettings {
	theme: TTheme;
	language: string;
	scan: IScanSettings;
}
