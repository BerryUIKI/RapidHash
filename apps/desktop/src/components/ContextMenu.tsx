import { useEffect, useRef } from "react";
import { Copy, Edit3, Trash2 } from "lucide-react";

export interface ContextMenuPosition {
  x: number;
  y: number;
}

export interface ContextMenuItemData {
  fileName: string;
  path: string;
  digests: Record<string, string>;
}

interface ContextMenuProps {
  position: ContextMenuPosition;
  item: ContextMenuItemData;
  onClose: () => void;
  onCopyText: (text: string) => void;
  onWriteCrc: (path: string, crcHex?: string) => void;
  onRemove: (path: string) => void;
}

export function ContextMenu({
  position,
  item,
  onClose,
  onCopyText,
  onWriteCrc,
  onRemove,
}: ContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    }
    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        onClose();
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [onClose]);

  return (
    <div
      ref={menuRef}
      style={{
        position: "fixed",
        top: position.y,
        left: position.x,
        zIndex: 9999,
        minWidth: 200,
        backgroundColor: "var(--bg-primary)",
        border: "1px solid var(--border-color)",
        borderRadius: 8,
        boxShadow: "0 6px 16px rgba(0,0,0,0.15)",
        padding: "4px 0",
        display: "flex",
        flexDirection: "column",
        fontSize: "0.85rem",
      }}
    >
      <div
        style={{
          padding: "6px 12px",
          borderBottom: "1px solid var(--border-color)",
          fontWeight: 600,
          color: "var(--text-secondary)",
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          maxWidth: 240,
        }}
        title={item.fileName}
      >
        {item.fileName}
      </div>

      <button
        className="menu-item"
        onClick={() => {
          onCopyText(item.fileName);
          onClose();
        }}
        style={menuItemStyle}
      >
        <Copy size={14} /> Copy File Name
      </button>

      <button
        className="menu-item"
        onClick={() => {
          onCopyText(item.path);
          onClose();
        }}
        style={menuItemStyle}
      >
        <Copy size={14} /> Copy Full Path
      </button>

      {Object.entries(item.digests).map(([algo, hex]) => (
        <button
          key={algo}
          className="menu-item"
          onClick={() => {
            onCopyText(hex);
            onClose();
          }}
          style={menuItemStyle}
        >
          <Copy size={14} /> Copy {algo.toUpperCase()} ({hex.slice(0, 8)}...)
        </button>
      ))}

      <div style={{ height: 1, backgroundColor: "var(--border-color)", margin: "4px 0" }} />

      <button
        className="menu-item"
        onClick={() => {
          onWriteCrc(item.path, item.digests.crc32);
          onClose();
        }}
        style={menuItemStyle}
      >
        <Edit3 size={14} color="var(--accent-color)" /> CRC into Filename
      </button>

      <button
        className="menu-item"
        onClick={() => {
          onRemove(item.path);
          onClose();
        }}
        style={{ ...menuItemStyle, color: "var(--status-mismatch)" }}
      >
        <Trash2 size={14} /> Remove from List
      </button>
    </div>
  );
}

const menuItemStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: 8,
  padding: "8px 12px",
  border: "none",
  background: "transparent",
  color: "var(--text-primary)",
  textAlign: "left",
  cursor: "pointer",
  width: "100%",
};
