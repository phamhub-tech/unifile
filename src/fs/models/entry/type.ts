
export interface IFileSystemEntryJson {
	name: string;
	path: string;
	size: number;

	/**
	 * Whether the entry is a file or folder
	 */
	entry_type: string;
	created: string | null;
	modified: string | null;

	/**
	 * The file type.
	 * 
	 * Only available for files.
	 */
	file_type: string | null;
}

export enum TFileType {
	app = 'app',
	audio = 'audio',
	archive = 'archive',
	document = 'document',
	ebook = 'ebook',
	image = 'image',
	video = 'video',
	other = 'other',
}

export enum TFileSystemEntryType {
	file = 'file',
	folder = 'folder',
}
export interface IFileSystemEntry {
	name: string,
	path: string,
	size: number,
	created: Date | null
	modified: Date | null
	type: TFileSystemEntryType,
	extension: string,

	/*
	 * The file type incase this entry is file
	 */
	fileType: TFileType | null
}


