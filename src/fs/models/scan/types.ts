import type FileSystemEntryModel from "../entry";

export interface IScanEntry {
  name: string;
  totalSize: number;
  fileType: FileSystemEntryModel['fileType'];
  duplicates: FileSystemEntryModel[]
}