import { app } from '@tauri-apps/api'
import type { Theme } from '@tauri-apps/api/window'

export class AppInfo {
	private _name: string = ''
	private _version: string = ''
	private _theme: Theme | null = 'light'

	constructor() {
		this.build()
	}

	async build() {
		this._version = await app.getVersion()
		this._name = await app.getName()
	}

	get name(): string {
		return this._name;
	}
	get version(): string {
		return this._version;
	}

	get theme() {
		return this._theme;
	}
	set theme(theme: Theme | null)  {
		this._theme = theme;
	}
}
