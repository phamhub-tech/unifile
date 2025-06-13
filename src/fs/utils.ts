import {
  FileIcon,
  FolderArchiveIcon as ArchiveIcon,
  ImageIcon,
  MusicIcon as AudioIcon,
  VideoIcon,
  FolderIcon,
} from 'lucide-vue-next'

import { TFileType } from "./models/entry";

/**
 * Gets the `TFileType` from the given `type`
 * 
 * @param type The raw file type string
 * @returns The typed TFileType
 */
export function getFileTypeFromJson(type: string): TFileType {
  let fileType = TFileType.other;
  switch (type) {
    case 'app':
      fileType = TFileType.app
      break;
    case 'audio':
      fileType = TFileType.audio
      break;
    case 'archive':
      fileType = TFileType.archive
      break;
    case 'document':
      fileType = TFileType.document
      break;
    case 'image':
      fileType = TFileType.image
      break;
    case 'video':
      fileType = TFileType.video
      break;
  }

  return fileType;
}

export function getHexForFileType(type: TFileType | null): { light: string, dark: string } {
  if (type === null) {
    return {
      light: 'oklch(90.5% 0.182 98.111)',
      dark: 'oklch(79.5% 0.184 86.047)'
    }
  }

  let lightHex = ''
  let darkHex = ''

  switch (type) {
    case TFileType.archive:
      lightHex = '#d8b4fe';
      darkHex = '#a855f7';
      break;
    case TFileType.audio:
      lightHex = darkHex = '#ec4899'
      break;
    case TFileType.image:
      lightHex = darkHex = '#22c55e'
      break;
    case TFileType.video:
      lightHex = '#fdba74';
      darkHex = '#f97316';
      break;
    case TFileType.ebook:
      lightHex = '#fca5a5';
      darkHex = '#ef4444';
      break;
    case TFileType.other:
      lightHex = '#e2e8f0';
      darkHex = '#94a3b8';
      break;
    default:
      lightHex = '#e2e8f0'
      darkHex = '#94a3b8'
  }

  return {
    light: lightHex,
    dark: darkHex,
  }
}

/**
 * Gets the icon for this file or folder.
 * 
 * If the type is null, then it is assumed this is a folder.
 * 
 * @param type The file type
 * @returns The corresponding icon
 */
export function getIconForFileType(type: TFileType | null): Component {
  if (type === null) return FolderIcon;


  switch (type) {
    case TFileType.archive:
      return ArchiveIcon
    case TFileType.audio:
      return AudioIcon
    case TFileType.image:
      return ImageIcon
    case TFileType.video:
      return VideoIcon
    default:
      return FileIcon
  }
}

export function getStylesForFileType(type: TFileType | null): string {
  const { light, dark } = getHexForFileType(type)
  let styles = `fill: ${light} !important; stroke: ${dark} !important;`

  switch (type) {
    case TFileType.audio:
    case TFileType.image:
      styles = `stroke: ${dark}`
      break;
  }

  return styles
}
