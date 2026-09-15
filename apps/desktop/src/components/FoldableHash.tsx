import React, { useState } from "react";
import { Copy, Check } from "lucide-react";
import { foldHash } from "../utils/format";

interface FoldableHashProps {
  algo: string;
  hash: string;
  forceFolded?: boolean | null; // null: auto/adaptive to window width, true: force folded, false: force expanded
  isWideWindow?: boolean; // whether window width is wide enough to expand adaptively
  onCopy: (text: string) => void;
  isCopied: boolean;
}

export function FoldableHash({
  algo,
  hash,
  forceFolded = null,
  isWideWindow = false,
  onCopy,
  isCopied,
}: FoldableHashProps) {
  const isLong = hash.length > 16;
  const [isHovered, setIsHovered] = useState(false);
  const [pinnedExpanded, setPinnedExpanded] = useState<boolean | null>(null);

  // Determine base folded state:
  // If forceFolded is explicitly boolean (true/false), honor it.
  // Otherwise, if auto/adaptive (null), expand when window is wide, fold when narrow.
  let baseFolded = false;
  if (isLong) {
    if (forceFolded === true) {
      baseFolded = true;
    } else if (forceFolded === false) {
      baseFolded = false;
    } else {
      // Auto: adapt to window width
      baseFolded = !isWideWindow;
    }
  }

  // If user explicitly clicked this row's hash, pinnedExpanded overrides baseFolded
  const effectiveFoldedWithoutHover = pinnedExpanded !== null ? !pinnedExpanded : baseFolded;

  // Hovering temporarily expands the hash if it was folded
  const isCurrentlyFolded = isLong && effectiveFoldedWithoutHover && !isHovered;

  const displayHash = (isLong && isCurrentlyFolded) ? foldHash(hash, 8, 8) : hash;

  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (!isLong) return;
    // Toggle pinned state: if currently folded (or hovered), toggle pinned expansion
    setPinnedExpanded((prev) => {
      if (prev === null) {
        return effectiveFoldedWithoutHover; // Pin to the opposite of current base
      }
      return !prev;
    });
  };

  const getTooltip = () => {
    if (!isLong) return hash;
    const actionHint = pinnedExpanded !== null
      ? (pinnedExpanded ? "Pinned expanded. Click to collapse." : "Pinned collapsed. Click to expand.")
      : isHovered
      ? "Hover expanded. Click to pin."
      : "Click or hover to expand.";
    return `${hash}\n(${actionHint})`;
  };

  return (
    <div
      className="digest-code"
      style={{
        display: "inline-flex",
        alignItems: "center",
        gap: 6,
        margin: "2px 0",
        position: "relative",
      }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <span className="digest-tag">{algo.toUpperCase()}:</span>
      <span
        className={`digest-hash-foldable ${isLong ? (isCurrentlyFolded ? "is-folded" : "is-expanded") : ""} ${isHovered ? "is-hovered" : ""}`}
        onClick={handleClick}
        title={getTooltip()}
        style={{
          cursor: isLong ? "pointer" : "text",
          fontFamily: "monospace",
          backgroundColor: isCurrentlyFolded ? "var(--bg-tertiary)" : "transparent",
          padding: "2px 6px",
          borderRadius: 4,
          transition: "background-color 0.15s, border-color 0.15s",
          border: isHovered && isCurrentlyFolded ? "1px dashed var(--accent-color)" : "1px solid transparent",
          userSelect: "text",
        }}
      >
        {displayHash}
      </span>
      <button
        type="button"
        className="btn"
        style={{
          padding: "2px 5px",
          minWidth: 24,
          height: 22,
          border: "none",
          background: "transparent",
          cursor: "pointer",
        }}
        onClick={(e) => {
          e.stopPropagation();
          onCopy(hash);
        }}
        title={`Copy ${algo.toUpperCase()} digest`}
      >
        {isCopied ? <Check size={12} color="var(--status-match)" /> : <Copy size={12} color="var(--text-secondary)" />}
      </button>
    </div>
  );
}
