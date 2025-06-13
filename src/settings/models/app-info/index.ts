import { app } from '@tauri-apps/api'

export class AppInfo {
	private _name: string = ''
	private _version: string = ''

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
}
