import { getFileTypeFromJson } from "../../utils";
import { TFileSystemEntryType, type IFileSystemEntry, type IFileSystemEntryJson, type TFileType } from "./type";

export * from './type'

export default class FileSystemEntryModel implements IFileSystemEntry {
  name: string;
  extension: string;
  path: string;
  size: number;
  type: TFileSystemEntryType;
  created: Date | null;
  modified: Date | null;
  fileType: TFileType | null;

  constructor(data: IFileSystemEntry) {
    this.name = data.name
    this.path = data.path
    this.size = data.size
    this.created = data.created
    this.modified = data.modified
    this.type = data.type
    this.fileType = data.fileType
    this.extension = data.extension
  }

  static fromJson(json: IFileSystemEntryJson): FileSystemEntryModel {
    let type = TFileSystemEntryType.folder
    if (json.entry_type === 'file') type = TFileSystemEntryType.file

    const jsonName = json.name
    const nameSplit = jsonName.split('.')
    let extension = nameSplit[nameSplit.length - 1];

    let name = jsonName
    if (nameSplit.length > 1 && !jsonName.startsWith('.')) name = nameSplit.slice(0, -1).join('.')

    // Common extensions
    if (name.startsWith('.env')) extension = 'env'

    return new FileSystemEntryModel({
      name,
      extension,
      path: json.path,
      size: json.size,
      created: json.created ? new Date(json.created) : null,
      modified: json.modified ? new Date(json.modified) : null,
      type,
      fileType: json.file_type ? getFileTypeFromJson(json.file_type) : null,
    })
  }

  get isFile(): boolean {
    return this.type === TFileSystemEntryType.file;
  }
}

