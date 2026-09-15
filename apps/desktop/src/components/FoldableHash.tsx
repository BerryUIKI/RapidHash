import { useState } from "react";
import { Copy, Check } from "lucide-react";
import { foldHash } from "../utils/format";

interface FoldableHashProps {
  algo: string;
  hash: string;
  forceFolded?: boolean | null;
  onCopy: (text: string) => void;
  isCopied: boolean;
}

export function FoldableHash({ algo, hash, forceFolded, onCopy, isCopied }: FoldableHashProps) {
  const isLong = hash.length > 16;
  const [localFolded, setLocalFolded] = useState<boolean>(true);

  // If forceFolded is provided, use it; otherwise use local state
  const folded = forceFolded !== null && forceFolded !== undefined ? forceFolded : (isLong && localFolded);

  const displayHash = (isLong && folded) ? foldHash(hash, 8, 8) : hash;

  return (
    <div className="digest-code" style={{ display: "inline-flex", alignItems: "center", gap: 6, margin: "2px 0" }}>
      <span className="digest-tag">{algo.toUpperCase()}:</span>
      <span
        className={isLong ? "digest-hash-foldable" : ""}
        onClick={() => {
          if (isLong) {
            setLocalFolded(!folded);
          }
        }}
        title={isLong ? `${hash}\n(Click to ${folded ? "expand" : "collapse"})` : hash}
        style={{
          cursor: isLong ? "pointer" : "text",
          fontFamily: "monospace",
          backgroundColor: folded ? "var(--bg-tertiary)" : "transparent",
          padding: "1px 4px",
          borderRadius: 3,
        }}
      >
        {displayHash}
      </span>
      <button
        type="button"
        className="btn"
        style={{ padding: "2px 5px", minWidth: 24, height: 22, border: "none", background: "transparent" }}
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
