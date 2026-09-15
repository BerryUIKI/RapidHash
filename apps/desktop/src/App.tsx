import React, { useState, useEffect, useRef } from "react";
import { 
  Calculator, 
  CheckCircle2, 
  FileText, 
  Settings as SettingsIcon, 
  Trash2, 
  Play, 
  FolderPlus, 
  FilePlus, 
  UploadCloud, 
  FileSearch,
  Edit3,
  Save,
  Minimize2,
  Maximize2
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open, save } from "@tauri-apps/plugin-dialog";
import { Locale, FALLBACK_STRINGS } from "./i18n";
import { FoldableHash } from "./components/FoldableHash";
import { ContextMenu, ContextMenuPosition, ContextMenuItemData } from "./components/ContextMenu";
import { truncateMiddle, formatBytes } from "./utils/format";

interface AlgorithmInfo {
  id: string;
  name: string;
  category: string;
  is_recommended: boolean;
  digest_length_bytes: number;
}

interface FileResultDto {
  path: string;
  file_name: string;
  size_bytes?: number;
  digests: Record<string, string>;
  status?: string;
  error?: string;
  filename_crc?: string;
}

interface RenameResultDto {
  old_path: string;
  new_path: string;
  success: boolean;
  error?: string;
}

interface ManifestSummaryDto {
  total: number;
  matches: number;
  mismatches: number;
  missing: number;
  unreadable: number;
  malformed: number;
  unsupported: number;
  items: FileResultDto[];
}

export function App() {
  const [activeTab, setActiveTab] = useState<"calculate" | "verify" | "generate" | "settings">("calculate");
  const [locale, setLocale] = useState<Locale>("en");
  const [theme, setTheme] = useState<"auto" | "light" | "dark">("auto");
  const [strings, setStrings] = useState<Record<string, string>>(FALLBACK_STRINGS.en);
  const [algorithms, setAlgorithms] = useState<AlgorithmInfo[]>([]);
  const [selectedAlgorithms, setSelectedAlgorithms] = useState<string[]>(["crc32", "sha256"]);
  const [inputPaths, setInputPaths] = useState<string[]>([]);
  const [results, setResults] = useState<FileResultDto[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);
  const [copiedKey, setCopiedKey] = useState<string | null>(null);
  const [autoCalculate, setAutoCalculate] = useState<boolean>(true);
  const [allFolded, setAllFolded] = useState<boolean | null>(null);
  const [isWideWindow, setIsWideWindow] = useState<boolean>(() => typeof window !== "undefined" ? window.innerWidth >= 1150 : false);

  // Monitor window resize for adaptive hash expansion
  useEffect(() => {
    const handleResize = () => {
      setIsWideWindow(window.innerWidth >= 1150);
    };
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  // Context menu state
  const [contextMenu, setContextMenu] = useState<{
    position: ContextMenuPosition;
    item: ContextMenuItemData;
  } | null>(null);

  // Verification state
  const [verifyPath, setVerifyPath] = useState("");
  const [verifyExpectedDigest, setVerifyExpectedDigest] = useState("");
  const [verifyAlgorithm, setVerifyAlgorithm] = useState("sha256");
  const [verifyManifestPath, setVerifyManifestPath] = useState("");
  const [manifestSummary, setManifestSummary] = useState<ManifestSummaryDto | null>(null);

  // Generation state
  const [generateFormat, setGenerateFormat] = useState<"sfv" | "sha256">("sfv");

  const fileInputRef = useRef<HTMLInputElement>(null);
  const folderInputRef = useRef<HTMLInputElement>(null);

  // Load strings when locale changes
  useEffect(() => {
    async function loadStrings() {
      try {
        const res = await invoke<Record<string, string>>("get_locale_strings", { locale });
        setStrings({ ...FALLBACK_STRINGS[locale], ...res });
      } catch {
        setStrings(FALLBACK_STRINGS[locale]);
      }
    }
    loadStrings();
  }, [locale]);

  // Load supported algorithms
  useEffect(() => {
    async function loadAlgorithms() {
      try {
        const algos = await invoke<AlgorithmInfo[]>("get_supported_algorithms");
        setAlgorithms(algos);
      } catch {
        setAlgorithms([
          { id: "crc32", name: "CRC32", category: "NonCryptographic", is_recommended: false, digest_length_bytes: 4 },
          { id: "sha256", name: "SHA-256", category: "ModernCryptographic", is_recommended: true, digest_length_bytes: 32 },
          { id: "blake3", name: "BLAKE3", category: "ModernCryptographic", is_recommended: true, digest_length_bytes: 32 },
        ]);
      }
    }
    loadAlgorithms();
  }, []);

  // Update root theme attribute
  useEffect(() => {
    if (theme === "dark") {
      document.documentElement.setAttribute("data-theme", "dark");
    } else if (theme === "light") {
      document.documentElement.removeAttribute("data-theme");
    } else {
      if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        document.documentElement.setAttribute("data-theme", "dark");
      } else {
        document.documentElement.removeAttribute("data-theme");
      }
    }
  }, [theme]);

  // Execute calculation for specific paths
  const calculatePaths = async (pathsToCalculate: string[], algosToUse?: string[]) => {
    if (pathsToCalculate.length === 0) return;
    setIsProcessing(true);
    try {
      const algos = algosToUse || selectedAlgorithms;
      const res = await invoke<FileResultDto[]>("calculate_hashes", {
        paths: pathsToCalculate,
        algorithmIds: algos,
      });
      setResults((prev) => {
        // Merge or replace by path
        const resMap = new Map<string, FileResultDto>();
        for (const item of prev) {
          resMap.set(item.path, item);
        }
        for (const item of res) {
          resMap.set(item.path, item);
        }
        return Array.from(resMap.values());
      });
    } catch (e) {
      console.error("Calculation failed:", e);
    } finally {
      setIsProcessing(false);
    }
  };

  // Tauri Drag-and-drop listener
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    async function registerDropListener() {
      try {
        const webview = getCurrentWebview();
        unlisten = await webview.onDragDropEvent((event) => {
          if (event.payload.type === "enter" || event.payload.type === "over") {
            setIsDragOver(true);
          } else if (event.payload.type === "drop") {
            setIsDragOver(false);
            const droppedPaths = event.payload.paths;
            if (droppedPaths && droppedPaths.length > 0) {
              handleIncomingPaths(droppedPaths);
            }
          } else {
            setIsDragOver(false);
          }
        });
      } catch (err) {
        console.warn("Tauri drag-drop listener not supported in this context:", err);
      }
    }
    registerDropListener();
    return () => {
      if (unlisten) unlisten();
    };
  }, [autoCalculate, selectedAlgorithms]);

  const handleIncomingPaths = (newPaths: string[]) => {
    setInputPaths((prev) => {
      const set = new Set(prev);
      for (const p of newPaths) {
        set.add(p);
      }
      return Array.from(set);
    });

    if (autoCalculate) {
      calculatePaths(newPaths);
    }
  };

  const t = (key: string): string => strings[key] || key;

  const handlePickFiles = async () => {
    try {
      const selected = await open({
        multiple: true,
        directory: false,
        title: t("calculate-add-files"),
      });
      if (selected) {
        const paths = Array.isArray(selected) ? selected : [selected];
        handleIncomingPaths(paths);
      }
    } catch {
      fileInputRef.current?.click();
    }
  };

  const handlePickFolder = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: true,
        title: t("calculate-add-folder"),
      });
      if (selected && typeof selected === "string") {
        handleIncomingPaths([selected]);
      }
    } catch {
      folderInputRef.current?.click();
    }
  };

  const handlePickManifest = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: t("verify-manifest-input"),
        filters: [
          { name: "Checksum Manifests", extensions: ["sha256", "sfv", "md5", "txt"] }
        ]
      });
      if (selected && typeof selected === "string") {
        setVerifyManifestPath(selected);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleBrowserFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files) return;
    const paths: string[] = [];
    for (let i = 0; i < files.length; i++) {
      paths.push(files[i].name);
    }
    handleIncomingPaths(paths);
  };

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedKey(text);
    setTimeout(() => setCopiedKey(null), 2000);
  };

  const runCalculate = async () => {
    if (inputPaths.length === 0) return;
    calculatePaths(inputPaths);
  };

  // RapidCRC Feature: Rename file to embed CRC into filename
  const handleWriteCrc = async (filePath: string) => {
    setIsProcessing(true);
    try {
      const res = await invoke<RenameResultDto>("write_crc_to_filename", {
        filePath,
      });
      if (res.success) {
        // Update results and inputPaths
        setResults((prev) =>
          prev.map((item) => {
            if (item.path === res.old_path) {
              const newFileName = res.new_path.split(/[\/\\]/).pop() || item.file_name;
              return {
                ...item,
                path: res.new_path,
                file_name: newFileName,
                status: "Match",
              };
            }
            return item;
          })
        );
        setInputPaths((prev) =>
          prev.map((p) => (p === res.old_path ? res.new_path : p))
        );
      } else {
        alert(`Failed to rename file: ${res.error}`);
      }
    } catch (e) {
      console.error("write_crc_to_filename error:", e);
      alert(`Error renaming file: ${e}`);
    } finally {
      setIsProcessing(false);
    }
  };

  // RapidCRC Feature: Batch write CRC for all completed items
  const handleBatchWriteCrc = async () => {
    if (results.length === 0) return;
    setIsProcessing(true);
    try {
      for (const item of results) {
        if (item.digests.crc32) {
          const res = await invoke<RenameResultDto>("write_crc_to_filename", {
            filePath: item.path,
          });
          if (res.success) {
            setResults((prev) =>
              prev.map((it) => {
                if (it.path === res.old_path) {
                  const newFileName = res.new_path.split(/[\/\\]/).pop() || it.file_name;
                  return {
                    ...it,
                    path: res.new_path,
                    file_name: newFileName,
                    status: "Match",
                  };
                }
                return it;
              })
            );
            setInputPaths((prev) =>
              prev.map((p) => (p === res.old_path ? res.new_path : p))
            );
          }
        }
      }
    } catch (e) {
      console.error("Batch write CRC error:", e);
    } finally {
      setIsProcessing(false);
    }
  };

  // Save Manifest (.sfv or .sha256)
  const handleSaveManifest = async (format: "sfv" | "sha256") => {
    if (results.length === 0) return;
    try {
      const ext = format === "sfv" ? "sfv" : "sha256";
      const savePath = await save({
        defaultPath: `checksums.${ext}`,
        filters: [{ name: `${format.toUpperCase()} Manifest`, extensions: [ext] }],
      });
      if (savePath && typeof savePath === "string") {
        await invoke("save_manifest_file", {
          outputPath: savePath,
          format,
          items: results,
        });
        alert(`Manifest saved successfully to:\n${savePath}`);
      }
    } catch (e) {
      console.error("Save manifest error:", e);
      alert(`Failed to save manifest: ${e}`);
    }
  };

  const handleRowContextMenu = (e: React.MouseEvent, row: FileResultDto) => {
    e.preventDefault();
    setContextMenu({
      position: { x: e.clientX, y: e.clientY },
      item: {
        fileName: row.file_name,
        path: row.path,
        digests: row.digests,
      },
    });
  };

  const handleRemoveItem = (path: string) => {
    setInputPaths((prev) => prev.filter((p) => p !== path));
    setResults((prev) => prev.filter((r) => r.path !== path));
  };

  const runCompareChecksum = async () => {
    if (!verifyPath.trim() || !verifyExpectedDigest.trim()) return;
    setIsProcessing(true);
    try {
      const res = await invoke<FileResultDto>("verify_file_checksum", {
        path: verifyPath.trim(),
        algorithm: verifyAlgorithm,
        expectedDigest: verifyExpectedDigest.trim(),
      });
      setResults([res]);
    } catch (e) {
      console.error("Verification failed:", e);
    } finally {
      setIsProcessing(false);
    }
  };

  const runVerifyManifest = async () => {
    if (!verifyManifestPath.trim()) return;
    setIsProcessing(true);
    try {
      const summary = await invoke<ManifestSummaryDto>("verify_manifest_file", {
        manifestPath: verifyManifestPath.trim(),
      });
      setManifestSummary(summary);
      setResults(summary.items);
    } catch (e) {
      console.error("Manifest verification failed:", e);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="app-container">
      {/* Hidden browser file inputs for fallback */}
      <input 
        type="file" 
        multiple 
        ref={fileInputRef} 
        style={{ display: "none" }} 
        onChange={handleBrowserFileInput}
      />
      <input 
        type="file" 
        multiple 
        // @ts-expect-error directory attribute
        webkitdirectory="" 
        ref={folderInputRef} 
        style={{ display: "none" }} 
        onChange={handleBrowserFileInput}
      />

      {/* Row Right-Click Context Menu */}
      {contextMenu && (
        <ContextMenu
          position={contextMenu.position}
          item={contextMenu.item}
          onClose={() => setContextMenu(null)}
          onCopyText={handleCopy}
          onWriteCrc={handleWriteCrc}
          onRemove={handleRemoveItem}
        />
      )}

      <aside className="sidebar">
        <div className="sidebar-header">
          <Calculator size={24} color="var(--accent-color)" />
          <span>{t("app-name")}</span>
        </div>
        <nav className="sidebar-nav">
          <button 
            className={`nav-item ${activeTab === "calculate" ? "active" : ""}`}
            onClick={() => setActiveTab("calculate")}
          >
            <Calculator size={18} />
            <span>{t("nav-calculate")}</span>
          </button>
          <button 
            className={`nav-item ${activeTab === "verify" ? "active" : ""}`}
            onClick={() => setActiveTab("verify")}
          >
            <CheckCircle2 size={18} />
            <span>{t("nav-verify")}</span>
          </button>
          <button 
            className={`nav-item ${activeTab === "generate" ? "active" : ""}`}
            onClick={() => setActiveTab("generate")}
          >
            <FileText size={18} />
            <span>{t("nav-generate")}</span>
          </button>
          <button 
            className={`nav-item ${activeTab === "settings" ? "active" : ""}`}
            onClick={() => setActiveTab("settings")}
          >
            <SettingsIcon size={18} />
            <span>{t("nav-settings")}</span>
          </button>
        </nav>
      </aside>

      <main className="main-content">
        {activeTab === "calculate" && (
          <div className="view-container">
            <div 
              className={`drop-zone ${isDragOver ? "drag-active" : ""}`}
              onClick={handlePickFiles}
              onDoubleClick={handlePickFolder}
              title="Click to add files, double-click to add folder"
            >
              <UploadCloud size={40} color="var(--accent-color)" />
              <span className="drop-zone-text">{t("calculate-drop-zone")}</span>
              <span className="drop-zone-hint">
                {t("calculate-drop-zone.hint")} (Click for files, double-click for folder)
              </span>
              {inputPaths.length > 0 && (
                <div style={{ marginTop: 8 }}>
                  <span className="badge">
                    {inputPaths.length} items loaded
                  </span>
                </div>
              )}
            </div>

            <div className="action-bar">
              <button className="btn btn-primary" onClick={runCalculate} disabled={isProcessing || inputPaths.length === 0}>
                <Play size={16} />
                <span>{isProcessing ? "Calculating..." : t("calculate-start")}</span>
              </button>
              <button className="btn" onClick={handlePickFiles}>
                <FilePlus size={16} />
                <span>{t("calculate-add-files")}</span>
              </button>
              <button className="btn" onClick={handlePickFolder}>
                <FolderPlus size={16} />
                <span>{t("calculate-add-folder")}</span>
              </button>

              {/* RapidCRC Feature: Batch CRC into Filename */}
              <button
                className="btn"
                onClick={handleBatchWriteCrc}
                disabled={isProcessing || results.length === 0}
                title="Write computed CRC32 into filenames (e.g. video [4E2A10FB].mkv)"
              >
                <Edit3 size={16} color="var(--accent-color)" />
                <span>{t("action-crc-into-filename")}</span>
              </button>

              {/* Export Manifest */}
              <button
                className="btn"
                onClick={() => handleSaveManifest("sfv")}
                disabled={isProcessing || results.length === 0}
                title="Export SFV checksum manifest"
              >
                <Save size={16} />
                <span>.SFV</span>
              </button>

              <button
                className="btn"
                onClick={() => handleSaveManifest("sha256")}
                disabled={isProcessing || results.length === 0}
                title="Export SHA-256 checksum manifest"
              >
                <Save size={16} />
                <span>.SHA256</span>
              </button>

              {/* Toggle hash folding mode (Auto/Adaptive -> Expand -> Fold -> Auto) */}
              <button
                className="btn"
                onClick={() => {
                  if (allFolded === null) setAllFolded(false);
                  else if (allFolded === false) setAllFolded(true);
                  else setAllFolded(null);
                }}
                title={
                  allFolded === null
                    ? `${t("action-hash-mode-auto")} (${isWideWindow ? "Expanded" : "Folded"}). Click to Expand All.`
                    : allFolded === false
                    ? `${t("action-expand-all")}. Click to Fold All.`
                    : `${t("action-fold-all")}. Click for Adaptive.`
                }
              >
                {allFolded === false ? <Minimize2 size={16} /> : <Maximize2 size={16} />}
                <span>
                  {allFolded === null
                    ? `${t("action-hash-mode-auto")} (${isWideWindow ? "Expanded" : "Folded"})`
                    : allFolded === false
                    ? t("action-expand-all")
                    : t("action-fold-all")}
                </span>
              </button>

              <button className="btn" onClick={() => { setInputPaths([]); setResults([]); }}>
                <Trash2 size={16} />
                <span>{t("calculate-clear-all")}</span>
              </button>

              {/* Auto calculate toggle */}
              <label style={{ display: "inline-flex", alignItems: "center", gap: 6, fontSize: "0.85rem", cursor: "pointer", marginLeft: 4 }}>
                <input
                  type="checkbox"
                  checked={autoCalculate}
                  onChange={(e) => setAutoCalculate(e.target.checked)}
                />
                <span>{t("auto-calculate-label")}</span>
              </label>

              {/* Algorithm selections */}
              <div style={{ marginLeft: "auto", display: "flex", gap: 10, alignItems: "center" }}>
                <span style={{ fontSize: "0.85rem", color: "var(--text-secondary)" }}>{t("table-column-algorithm")}:</span>
                {algorithms.map((algo) => (
                  <label key={algo.id} style={{ display: "flex", alignItems: "center", gap: 4, fontSize: "0.85rem", cursor: "pointer" }}>
                    <input 
                      type="checkbox"
                      checked={selectedAlgorithms.includes(algo.id)}
                      onChange={(e) => {
                        if (e.target.checked) {
                          setSelectedAlgorithms([...selectedAlgorithms, algo.id]);
                        } else {
                          setSelectedAlgorithms(selectedAlgorithms.filter((id) => id !== algo.id));
                        }
                      }}
                    />
                    {algo.name}
                  </label>
                ))}
              </div>
            </div>

            <div className="table-wrapper">
              <table>
                <thead>
                  <tr>
                    <th className="col-filename">{t("table-column-name")}</th>
                    <th>{t("table-column-status")}</th>
                    <th>{t("table-column-size")}</th>
                    <th>{t("table-column-algorithm")}</th>
                    <th className="col-digest">{t("table-column-digest")}</th>
                  </tr>
                </thead>
                <tbody>
                  {results.length === 0 && inputPaths.length > 0 ? (
                    inputPaths.map((p, idx) => {
                      const name = p.split(/[\/\\]/).pop() || p;
                      return (
                        <tr key={idx}>
                          <td className="col-filename" title={p}>
                            <span className="cell-truncate-middle">{truncateMiddle(name, 36)}</span>
                          </td>
                          <td>
                            <span className="badge">Queued</span>
                          </td>
                          <td>-</td>
                          <td>{selectedAlgorithms.join(", ").toUpperCase()}</td>
                          <td style={{ color: "var(--text-secondary)", fontStyle: "italic" }}>Calculating...</td>
                        </tr>
                      );
                    })
                  ) : results.length === 0 ? (
                    <tr>
                      <td colSpan={5} style={{ textAlign: "center", padding: 30, color: "var(--text-secondary)" }}>
                        No items added yet. Drag & drop files or click to add.
                      </td>
                    </tr>
                  ) : (
                    results.map((row, idx) => (
                      <tr 
                        key={idx}
                        onContextMenu={(e) => handleRowContextMenu(e, row)}
                        style={{ cursor: "context-menu" }}
                      >
                        <td className="col-filename" title={row.path}>
                          <span className="cell-truncate-middle">{truncateMiddle(row.file_name, 36)}</span>
                        </td>
                        <td>
                          {row.status === "Match" || row.status === "verification-status-match" ? (
                            <span className="status-badge status-match">
                              {row.filename_crc ? "Match (CRC)" : t("verification-status-match")}
                            </span>
                          ) : row.status === "Mismatch" || row.status === "verification-status-mismatch" ? (
                            <span className="status-badge status-mismatch">
                              {row.filename_crc ? "Mismatch (CRC)" : t("verification-status-mismatch")}
                            </span>
                          ) : row.error ? (
                            <span className="status-badge status-mismatch">Error</span>
                          ) : (
                            <span className="status-badge status-match" style={{ opacity: 0.75 }}>OK</span>
                          )}
                        </td>
                        <td>{formatBytes(row.size_bytes)}</td>
                        <td>{Object.keys(row.digests).join(", ").toUpperCase()}</td>
                        <td>
                          {row.error ? (
                            <span style={{ color: "var(--status-mismatch)" }}>{row.error}</span>
                          ) : (
                            <div style={{ display: "flex", flexDirection: "column", gap: 2 }}>
                              {Object.entries(row.digests).map(([algo, hex]) => (
                                <FoldableHash
                                  key={algo}
                                  algo={algo}
                                  hash={hex}
                                  forceFolded={allFolded}
                                  isWideWindow={isWideWindow}
                                  onCopy={handleCopy}
                                  isCopied={copiedKey === hex}
                                />
                              ))}
                            </div>
                          )}
                        </td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === "verify" && (
          <div className="view-container">
            <h3>{t("nav-verify")}</h3>
            
            {/* Section 1: Compare single file */}
            <div style={{ display: "flex", flexDirection: "column", gap: 12, padding: 16, border: "1px solid var(--border-color)", borderRadius: 8 }}>
              <div style={{ fontWeight: 600, fontSize: "1rem" }}>Single File Verification</div>
              <label style={{ fontSize: "0.85rem", fontWeight: 600 }}>{t("verify-target-input")}</label>
              <div style={{ display: "flex", gap: 8 }}>
                <input 
                  type="text" 
                  value={verifyPath}
                  onChange={(e) => setVerifyPath(e.target.value)}
                  placeholder={t("verify-target-input.placeholder")} 
                  style={{ flex: 1, padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                />
                <button className="btn" onClick={async () => {
                  try {
                    const sel = await open({ multiple: false, directory: false });
                    if (sel && typeof sel === "string") setVerifyPath(sel);
                  } catch (err) { console.error(err); }
                }}>
                  <FileSearch size={16} /> Browse
                </button>
              </div>

              <div style={{ display: "flex", gap: 12 }}>
                <div style={{ flex: 1, display: "flex", flexDirection: "column", gap: 6 }}>
                  <label style={{ fontSize: "0.85rem", fontWeight: 600 }}>{t("verify-digest-input")}</label>
                  <input 
                    type="text" 
                    value={verifyExpectedDigest}
                    onChange={(e) => setVerifyExpectedDigest(e.target.value)}
                    placeholder={t("verify-digest-input.placeholder")} 
                    style={{ padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                  />
                </div>
                <div style={{ width: 140, display: "flex", flexDirection: "column", gap: 6 }}>
                  <label style={{ fontSize: "0.85rem", fontWeight: 600 }}>{t("table-column-algorithm")}</label>
                  <select 
                    value={verifyAlgorithm}
                    onChange={(e) => setVerifyAlgorithm(e.target.value)}
                    style={{ padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                  >
                    <option value="crc32">CRC32</option>
                    <option value="sha256">SHA-256</option>
                    <option value="blake3">BLAKE3</option>
                  </select>
                </div>
              </div>

              <button className="btn btn-primary" style={{ alignSelf: "flex-start" }} onClick={runCompareChecksum} disabled={isProcessing}>
                {t("verify-action-compare")}
              </button>
            </div>

            {/* Section 2: Verify Checksum Manifest */}
            <div style={{ display: "flex", flexDirection: "column", gap: 12, padding: 16, border: "1px solid var(--border-color)", borderRadius: 8 }}>
              <div style={{ fontWeight: 600, fontSize: "1rem" }}>Manifest Verification (.sfv / .sha256)</div>
              <label style={{ fontSize: "0.85rem", fontWeight: 600 }}>{t("verify-manifest-input")}</label>
              <div style={{ display: "flex", gap: 8 }}>
                <input 
                  type="text" 
                  value={verifyManifestPath}
                  onChange={(e) => setVerifyManifestPath(e.target.value)}
                  placeholder={t("verify-manifest-input.placeholder")} 
                  style={{ flex: 1, padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                />
                <button className="btn" onClick={handlePickManifest}>
                  <FileSearch size={16} /> Browse
                </button>
              </div>
              <button className="btn btn-primary" style={{ alignSelf: "flex-start" }} onClick={runVerifyManifest} disabled={isProcessing}>
                {t("verify-action-verify-manifest")}
              </button>

              {manifestSummary && (
                <div style={{ display: "flex", gap: 8, marginTop: 4 }}>
                  <span className="badge">Total: {manifestSummary.total}</span>
                  <span className="status-badge status-match">Match: {manifestSummary.matches}</span>
                  <span className="status-badge status-mismatch">Mismatch: {manifestSummary.mismatches}</span>
                  {manifestSummary.missing > 0 && <span className="status-badge status-missing">Missing: {manifestSummary.missing}</span>}
                  {manifestSummary.unreadable > 0 && <span className="status-badge status-unreadable">Unreadable: {manifestSummary.unreadable}</span>}
                </div>
              )}
            </div>

            {results.length > 0 && (
              <div className="table-wrapper">
                <table>
                  <thead>
                    <tr>
                      <th className="col-filename">{t("table-column-name")}</th>
                      <th>{t("table-column-status")}</th>
                      <th>{t("table-column-algorithm")}</th>
                      <th className="col-digest">{t("table-column-digest")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {results.map((row, idx) => (
                      <tr 
                        key={idx}
                        onContextMenu={(e) => handleRowContextMenu(e, row)}
                      >
                        <td className="col-filename" title={row.path}>
                          <span className="cell-truncate-middle">{truncateMiddle(row.file_name, 36)}</span>
                        </td>
                        <td>
                          {row.status === "verification-status-match" || row.status === "Match" ? (
                            <span className="status-badge status-match">{t("verification-status-match")}</span>
                          ) : row.status === "verification-status-mismatch" || row.status === "Mismatch" ? (
                            <span className="status-badge status-mismatch">{t("verification-status-mismatch")}</span>
                          ) : (
                            <span className="status-badge status-missing">{row.status || "-"}</span>
                          )}
                        </td>
                        <td>{Object.keys(row.digests).join(", ").toUpperCase()}</td>
                        <td>
                          <div style={{ display: "flex", flexDirection: "column", gap: 2 }}>
                            {Object.entries(row.digests).map(([algo, hex]) => (
                              <FoldableHash
                                key={algo}
                                algo={algo}
                                hash={hex}
                                forceFolded={allFolded}
                                isWideWindow={isWideWindow}
                                onCopy={handleCopy}
                                isCopied={copiedKey === hex}
                              />
                            ))}
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        )}

        {activeTab === "generate" && (
          <div className="view-container">
            <h3>{t("nav-generate")}</h3>
            <div 
              className="drop-zone"
              onClick={handlePickFiles}
              onDoubleClick={handlePickFolder}
            >
              <UploadCloud size={40} color="var(--accent-color)" />
              <span className="drop-zone-text">{t("calculate-drop-zone")}</span>
              <span className="drop-zone-hint">Add files or directories to include in manifest</span>
              {results.length > 0 && (
                <span className="badge" style={{ marginTop: 8 }}>
                  {results.length} items ready for manifest
                </span>
              )}
            </div>

            <div style={{ display: "flex", gap: 16, alignItems: "center", marginTop: 8 }}>
              <label style={{ display: "flex", alignItems: "center", gap: 6, fontWeight: 500 }}>
                Format:
                <select
                  value={generateFormat}
                  onChange={(e) => setGenerateFormat(e.target.value as "sfv" | "sha256")}
                  style={{ padding: 6, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                >
                  <option value="sfv">SFV (.sfv - CRC32)</option>
                  <option value="sha256">GNU SHA-256 (.sha256)</option>
                </select>
              </label>

              <button 
                className="btn btn-primary" 
                onClick={() => handleSaveManifest(generateFormat)}
                disabled={results.length === 0}
              >
                <Save size={16} />
                <span>Save {generateFormat.toUpperCase()} Manifest</span>
              </button>
            </div>
          </div>
        )}

        {activeTab === "settings" && (
          <div className="view-container">
            <h3>{t("settings-title")}</h3>
            <div style={{ display: "flex", flexDirection: "column", gap: 16, maxWidth: 400 }}>
              <div>
                <label style={{ display: "block", marginBottom: 6, fontWeight: 600 }}>{t("settings-language")}</label>
                <select 
                  value={locale} 
                  onChange={(e) => setLocale(e.target.value as Locale)}
                  style={{ width: "100%", padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                >
                  <option value="en">English (en)</option>
                  <option value="zh-CN">简体中文 (zh-CN)</option>
                </select>
              </div>

              <div>
                <label style={{ display: "block", marginBottom: 6, fontWeight: 600 }}>{t("settings-appearance")}</label>
                <select 
                  value={theme} 
                  onChange={(e) => setTheme(e.target.value as "auto" | "light" | "dark")}
                  style={{ width: "100%", padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
                >
                  <option value="auto">{t("settings-theme-auto")}</option>
                  <option value="light">{t("settings-theme-light")}</option>
                  <option value="dark">{t("settings-theme-dark")}</option>
                </select>
              </div>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
