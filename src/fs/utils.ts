import {
  FileIcon,
  FolderArchiveIcon as ArchiveIcon,
  ImageIcon,
  MusicIcon as AudioIcon,
  VideoIcon,
} from 'lucide-vue-next'

import { TFileType } from "./models/entry";

export function getFileTypeFromJson(type: string): TFileType {
  let fileType = TFileType.other
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

export function getHexForFileType(type: TFileType): { light: string, dark: string } {
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
      lightHex = 'hsl(var(--primary) / .2)'
      darkHex = 'hsl(var(--primary)'
  }

  return {
    light: lightHex,
    dark: darkHex,
  }
}

export function getIconForFileType(type: TFileType): Component {
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

export function getStylesForFileType(type: TFileType): string {
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
