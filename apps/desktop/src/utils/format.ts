/**
 * Truncates a string in the middle with an ellipsis (...) if it exceeds maxLength.
 * Preserves the start and end of the string.
 */
export function truncateMiddle(str: string, maxLength: number = 36): string {
  if (!str || str.length <= maxLength) {
    return str;
  }
  const ellipsis = "...";
  const charsToShow = maxLength - ellipsis.length;
  if (charsToShow <= 0) {
    return str.slice(0, maxLength);
  }
  const frontChars = Math.ceil(charsToShow / 2);
  const backChars = Math.floor(charsToShow / 2);
  return `${str.slice(0, frontChars)}${ellipsis}${str.slice(str.length - backChars)}`;
}

/**
 * Folds a long hash string (e.g. 64-character SHA-256) by preserving the prefix and suffix.
 */
export function foldHash(hash: string, prefixLen: number = 6, suffixLen: number = 6): string {
  if (!hash || hash.length <= prefixLen + suffixLen + 3) {
    return hash;
  }
  return `${hash.slice(0, prefixLen)}...${hash.slice(hash.length - suffixLen)}`;
}

/**
 * Format bytes into human-readable size.
 */
export function formatBytes(bytes?: number): string {
  if (bytes === undefined || bytes === null) return "-";
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}
