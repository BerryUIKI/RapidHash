import { useState, useEffect } from "react";
import { 
  Calculator, 
  CheckCircle2, 
  FileText, 
  Settings as SettingsIcon, 
  Trash2, 
  Play, 
  Copy, 
  Check
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { Locale, FALLBACK_STRINGS } from "./i18n";

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
}

export function App() {
  const [activeTab, setActiveTab] = useState<"calculate" | "verify" | "generate" | "settings">("calculate");
  const [locale, setLocale] = useState<Locale>("en");
  const [theme, setTheme] = useState<"auto" | "light" | "dark">("auto");
  const [strings, setStrings] = useState<Record<string, string>>(FALLBACK_STRINGS.en);
  const [algorithms, setAlgorithms] = useState<AlgorithmInfo[]>([]);
  const [selectedAlgorithms, setSelectedAlgorithms] = useState<string[]>(["sha256"]);
  const [inputPaths, setInputPaths] = useState<string[]>([]);
  const [results, setResults] = useState<FileResultDto[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [copiedKey, setCopiedKey] = useState<string | null>(null);

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
        // Fallback default
        setAlgorithms([
          { id: "sha256", name: "SHA-256", category: "ModernCryptographic", is_recommended: true, digest_length_bytes: 32 },
          { id: "blake3", name: "BLAKE3", category: "ModernCryptographic", is_recommended: true, digest_length_bytes: 32 },
          { id: "crc32", name: "CRC32", category: "NonCryptographic", is_recommended: false, digest_length_bytes: 4 },
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

  const t = (key: string): string => strings[key] || key;

  const handleCopy = (text: string, key: string) => {
    navigator.clipboard.writeText(text);
    setCopiedKey(key);
    setTimeout(() => setCopiedKey(null), 2000);
  };

  const runCalculate = async () => {
    if (inputPaths.length === 0) return;
    setIsProcessing(true);
    try {
      const res = await invoke<FileResultDto[]>("calculate_hashes", {
        paths: inputPaths,
        algorithmIds: selectedAlgorithms,
      });
      setResults(res);
    } catch (e) {
      console.error("Calculation failed:", e);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="app-container">
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
            <div className="drop-zone">
              <span className="drop-zone-text">{t("calculate-drop-zone")}</span>
              <span className="drop-zone-hint">{t("calculate-drop-zone.hint")}</span>
            </div>

            <div className="action-bar">
              <button className="btn btn-primary" onClick={runCalculate} disabled={isProcessing}>
                <Play size={16} />
                <span>{t("calculate-start")}</span>
              </button>
              <button className="btn" onClick={() => { setInputPaths([]); setResults([]); }}>
                <Trash2 size={16} />
                <span>{t("calculate-clear-all")}</span>
              </button>
              <div style={{ marginLeft: "auto", display: "flex", gap: 8, alignItems: "center" }}>
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
                    <th>{t("table-column-name")}</th>
                    <th>{t("table-column-size")}</th>
                    <th>{t("table-column-algorithm")}</th>
                    <th>{t("table-column-digest")}</th>
                    <th style={{ width: 80 }}>{t("table-column-actions")}</th>
                  </tr>
                </thead>
                <tbody>
                  {results.length === 0 ? (
                    <tr>
                      <td colSpan={5} style={{ textAlign: "center", padding: 30, color: "var(--text-secondary)" }}>
                        No items calculated yet.
                      </td>
                    </tr>
                  ) : (
                    results.map((row, idx) => (
                      <tr key={idx}>
                        <td title={row.path}>{row.file_name}</td>
                        <td>{row.size_bytes !== undefined ? `${(row.size_bytes / 1024).toFixed(1)} KB` : "-"}</td>
                        <td>{Object.keys(row.digests).join(", ").toUpperCase()}</td>
                        <td>
                          {Object.entries(row.digests).map(([algo, hex]) => (
                            <div key={algo} className="digest-code">
                              <span style={{ color: "var(--text-secondary)", marginRight: 6 }}>{algo.toUpperCase()}:</span>
                              {hex}
                            </div>
                          ))}
                        </td>
                        <td>
                          {Object.entries(row.digests).map(([algo, hex]) => {
                            const key = `${idx}-${algo}`;
                            return (
                              <button 
                                key={key}
                                className="btn"
                                style={{ padding: "4px 8px" }}
                                onClick={() => handleCopy(hex, key)}
                                title={`Copy ${algo.toUpperCase()} digest`}
                              >
                                {copiedKey === key ? <Check size={14} color="var(--status-match)" /> : <Copy size={14} />}
                              </button>
                            );
                          })}
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
            <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
              <label style={{ fontSize: "0.9rem", fontWeight: 600 }}>{t("verify-target-input")}</label>
              <input 
                type="text" 
                placeholder={t("verify-target-input.placeholder")} 
                style={{ padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
              />
              <label style={{ fontSize: "0.9rem", fontWeight: 600 }}>{t("verify-digest-input")}</label>
              <input 
                type="text" 
                placeholder={t("verify-digest-input.placeholder")} 
                style={{ padding: 8, borderRadius: 6, border: "1px solid var(--border-color)", background: "var(--bg-secondary)", color: "var(--text-primary)" }}
              />
              <button className="btn btn-primary" style={{ alignSelf: "flex-start" }}>
                {t("verify-action-compare")}
              </button>
            </div>
          </div>
        )}

        {activeTab === "generate" && (
          <div className="view-container">
            <h3>{t("nav-generate")}</h3>
            <div className="drop-zone">
              <span className="drop-zone-text">{t("calculate-drop-zone")}</span>
            </div>
            <div className="action-bar">
              <button className="btn btn-primary">
                {t("nav-generate")} Manifest
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
