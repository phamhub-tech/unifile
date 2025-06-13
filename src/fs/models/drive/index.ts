import { IDriveType, type IDrive, type IDriveJson } from './types';

export * from './types';

export class DriveModel implements IDrive {
  name: string;
  total: number;
  used: number;
  available: number;
  mountPoint: string;
  isRemovable: boolean;
  type: IDriveType;

  constructor(data: IDrive) {
    this.name = data.name
    this.total = data.total
    this.used = data.used
    this.available = data.available
    this.mountPoint = data.mountPoint
    this.isRemovable = data.isRemovable
    this.type = data.type
  }

  static fromJson(json: IDriveJson): DriveModel {
    let type = IDriveType.Uknown
    switch (json.drive_type) {
      case 'ssd':
        type = IDriveType.Ssd
        break;
      case 'hdd':
        type = IDriveType.Hdd
        break;
    }

    return new DriveModel({
      name: json.name,
      total: json.total_bytes,
      used: json.used_bytes,
      available: json.available_bytes,
      mountPoint: json.mount_point,
      isRemovable: json.is_removable,
      type,
    })
  }
}