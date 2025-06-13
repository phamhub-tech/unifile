import type { IFileSystemEntryJson } from "./models/entry";

export type ScanEvent =
  {
    event: 'started';
    data: unknown;
  }
  | {
    event: 'progress';
    data: { entry: IFileSystemEntryJson };
  }
  | {
    event: 'finished';
    data: unknown;
  }