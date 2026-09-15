import React, { useState, useRef, useEffect } from "react";
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
  const [pinnedExpanded, setPinnedExpanded] = useState<boolean | null>(null);
  const [showPopover, setShowPopover] = useState(false);
  const [popoverCoords, setPopoverCoords] = useState<{ x: number; y: number }>({ x: 0, y: 0 });

  const triggerRef = useRef<HTMLSpanElement>(null);
  const enterTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const leaveTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Clean up timers on unmount
  useEffect(() => {
    return () => {
      if (enterTimerRef.current) clearTimeout(enterTimerRef.current);
      if (leaveTimerRef.current) clearTimeout(leaveTimerRef.current);
    };
  }, []);

  // Determine base folded state:
  let baseFolded = false;
  if (isLong) {
    if (forceFolded === true) {
      baseFolded = true;
    } else if (forceFolded === false) {
      baseFolded = false;
    } else {
      // Adaptive to window width
      baseFolded = !isWideWindow;
    }
  }

  // If user clicked to toggle inline expansion, pinnedExpanded overrides baseFolded
  const isInlineFolded = isLong && (pinnedExpanded !== null ? !pinnedExpanded : baseFolded);

  // The inline display text NEVER changes on hover!
  // This guarantees zero GUI jumping or table column shifting.
  const displayHash = isInlineFolded ? foldHash(hash, 8, 8) : hash;

  const handleMouseEnter = () => {
    if (leaveTimerRef.current) {
      clearTimeout(leaveTimerRef.current);
      leaveTimerRef.current = null;
    }

    if (!isLong) return;

    // Hover delay: trigger popover after 350ms lingering
    enterTimerRef.current = setTimeout(() => {
      if (triggerRef.current) {
        const rect = triggerRef.current.getBoundingClientRect();
        // Popover is ~490px wide on desktop; clamp x to viewport
        const popoverWidth = 490;
        const rawX = rect.left;
        const clampedX = Math.max(16, Math.min(window.innerWidth - popoverWidth - 16, rawX));
        // If too close to top of viewport, show below element, else show above
        const showBelow = rect.top < 50;
        const y = showBelow ? rect.bottom + 6 : rect.top - 44;

        setPopoverCoords({ x: clampedX, y });
        setShowPopover(true);
      }
    }, 350);
  };

  const handleMouseLeave = () => {
    if (enterTimerRef.current) {
      clearTimeout(enterTimerRef.current);
      enterTimerRef.current = null;
    }

    // Grace period before closing popover so user can move mouse onto popover
    leaveTimerRef.current = setTimeout(() => {
      setShowPopover(false);
    }, 180);
  };

  const handlePopoverMouseEnter = () => {
    if (leaveTimerRef.current) {
      clearTimeout(leaveTimerRef.current);
      leaveTimerRef.current = null;
    }
  };

  const handlePopoverMouseLeave = () => {
    leaveTimerRef.current = setTimeout(() => {
      setShowPopover(false);
    }, 180);
  };

  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (!isLong) return;
    // Clicking toggles inline pinned expansion
    setPinnedExpanded((prev) => {
      if (prev === null) {
        return isInlineFolded; // Pin to expanded
      }
      return !prev;
    });
  };

  return (
    <>
      <div
        className="digest-code"
        style={{
          display: "inline-flex",
          alignItems: "center",
          gap: 6,
          margin: "2px 0",
          position: "relative",
        }}
      >
        <span className="digest-tag">{algo.toUpperCase()}:</span>
        <span
          ref={triggerRef}
          className={`digest-hash-foldable ${isLong ? (isInlineFolded ? "is-folded" : "is-expanded") : ""}`}
          onClick={handleClick}
          onMouseEnter={handleMouseEnter}
          onMouseLeave={handleMouseLeave}
          title={isLong ? "Click to toggle inline expansion. Hover to view full digest." : hash}
          style={{
            cursor: isLong ? "pointer" : "text",
            fontFamily: "monospace",
            backgroundColor: isInlineFolded ? "var(--bg-tertiary)" : "transparent",
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

      {/* Floating popover card shown after lingering hover */}
      {showPopover && (
        <div
          className="digest-popover"
          style={{
            top: popoverCoords.y,
            left: popoverCoords.x,
          }}
          onMouseEnter={handlePopoverMouseEnter}
          onMouseLeave={handlePopoverMouseLeave}
        >
          <span
            style={{
              fontWeight: 700,
              fontSize: "0.8rem",
              color: "var(--accent-color)",
              padding: "1px 6px",
              borderRadius: 3,
              backgroundColor: "var(--bg-tertiary)",
            }}
          >
            {algo.toUpperCase()}
          </span>
          <span
            style={{
              userSelect: "all",
              letterSpacing: "0.5px",
              color: "var(--text-primary)",
            }}
          >
            {hash}
          </span>
          <button
            type="button"
            className="btn"
            style={{
              padding: "3px 8px",
              height: 24,
              fontSize: "0.8rem",
              gap: 4,
            }}
            onClick={(e) => {
              e.stopPropagation();
              onCopy(hash);
            }}
            title="Copy full digest"
          >
            {isCopied ? (
              <>
                <Check size={13} color="var(--status-match)" />
                <span style={{ color: "var(--status-match)", fontSize: "0.75rem" }}>Copied!</span>
              </>
            ) : (
              <>
                <Copy size={13} />
                <span style={{ fontSize: "0.75rem" }}>Copy</span>
              </>
            )}
          </button>
        </div>
      )}
    </>
  );
}
