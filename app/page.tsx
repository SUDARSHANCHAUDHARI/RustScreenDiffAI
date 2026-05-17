"use client";
import { useState, useRef } from "react";
const VERDICT_STYLE: Record<string, { text: string; bg: string; border: string }> = {
  "Pass": { text: "text-emerald-400", bg: "bg-emerald-500/10", border: "border-emerald-500/30" },
  "Review needed": { text: "text-amber-400", bg: "bg-amber-500/10", border: "border-amber-500/30" },
  "Fail": { text: "text-red-400", bg: "bg-red-500/10", border: "border-red-500/30" },
};
const SEV_COLORS: Record<string, string> = { critical: "bg-red-500/20 text-red-400 border-red-500/30", high: "bg-orange-500/20 text-orange-400 border-orange-500/30", medium: "bg-amber-500/20 text-amber-400 border-amber-500/30", low: "bg-zinc-700/50 text-zinc-400 border-zinc-600" };
const TYPE_COLORS: Record<string, string> = { Layout: "bg-blue-900/40 text-blue-400", Content: "bg-purple-900/40 text-purple-400", Color: "bg-pink-900/40 text-pink-400", Typography: "bg-yellow-900/40 text-yellow-400", "Missing element": "bg-red-900/40 text-red-400", "New element": "bg-emerald-900/40 text-emerald-400" };

interface Change { area: string; type: string; severity: string; description: string; regression: boolean; }
interface Result { overallDiff: string; diffScore: number; changes: Change[]; regressions: string[]; improvements: string[]; verdict: string; summary: string; }

function DropZone({ label, preview, onFile }: { label: string; preview: string | null; onFile: (b64: string, mime: string) => void }) {
  const ref = useRef<HTMLInputElement>(null);
  function handleFile(file: File) {
    const reader = new FileReader();
    reader.onload = e => { const b64 = (e.target?.result as string).split(",")[1]; onFile(b64, file.type as "image/jpeg" | "image/png" | "image/webp"); };
    reader.readAsDataURL(file);
  }
  return (
    <div onClick={() => ref.current?.click()} onDragOver={e => e.preventDefault()} onDrop={e => { e.preventDefault(); const f = e.dataTransfer.files[0]; if (f) handleFile(f); }} className="relative border-2 border-dashed border-zinc-700 hover:border-indigo-500 rounded-xl overflow-hidden cursor-pointer transition-colors h-48">
      <input ref={ref} type="file" accept="image/*" className="hidden" onChange={e => { const f = e.target.files?.[0]; if (f) handleFile(f); }} />
      {preview ? <img src={preview} alt={label} className="w-full h-full object-contain bg-black" /> : (
        <div className="flex flex-col items-center justify-center h-full gap-2">
          <div className="text-3xl">🖼</div>
          <p className="text-sm text-zinc-400">{label}</p>
          <p className="text-xs text-zinc-600">Drop image or click</p>
        </div>
      )}
    </div>
  );
}

export default function ScreenDiffAI() {
  const [beforeB64, setBeforeB64] = useState<string | null>(null);
  const [afterB64, setAfterB64] = useState<string | null>(null);
  const [beforePreview, setBeforePreview] = useState<string | null>(null);
  const [afterPreview, setAfterPreview] = useState<string | null>(null);
  const [mimeType, setMimeType] = useState("image/png");
  const [context, setContext] = useState("");
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<Result | null>(null);
  const [error, setError] = useState("");

  function handleBefore(b64: string, mime: string) { setBeforeB64(b64); setMimeType(mime); setBeforePreview(`data:${mime};base64,${b64}`); }
  function handleAfter(b64: string, mime: string) { setAfterB64(b64); setMimeType(mime); setAfterPreview(`data:${mime};base64,${b64}`); }

  async function compare() {
    if (!beforeB64 || !afterB64) return;
    setLoading(true); setResult(null); setError("");
    try {
      const res = await fetch("/api/compare", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ beforeImage: beforeB64, afterImage: afterB64, mimeType, context }) });
      const data = await res.json();
      if (data.error) setError(data.error); else setResult(data);
    } catch { setError("Network error"); }
    setLoading(false);
  }

  const vStyle = result ? VERDICT_STYLE[result.verdict] || VERDICT_STYLE["Review needed"] : null;

  return (
    <div className="min-h-screen bg-[#0a0a0f] text-white font-sans">
      <div className="max-w-5xl mx-auto px-6 py-10">
        <div className="mb-8">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-9 h-9 rounded-lg bg-indigo-500/20 border border-indigo-500/30 flex items-center justify-center text-lg">🔍</div>
            <h1 className="text-2xl font-bold">ScreenDiffAI</h1>
          </div>
          <p className="text-zinc-400 text-sm">Visual regression detection for digital signage. Upload before/after screenshots to catch layout changes, missing elements, and regressions.</p>
        </div>

        <div className="bg-zinc-900/60 border border-zinc-800 rounded-xl p-6 mb-6">
          <div className="grid grid-cols-2 gap-4 mb-4">
            <div><p className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">Before</p><DropZone label="Before screenshot" preview={beforePreview} onFile={handleBefore} /></div>
            <div><p className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">After</p><DropZone label="After screenshot" preview={afterPreview} onFile={handleAfter} /></div>
          </div>
          <div className="mb-4">
            <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2 block">Context (optional)</label>
            <input value={context} onChange={e => setContext(e.target.value)} placeholder="e.g. Firmware update v4.2, layout redesign, content refresh" className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-3 text-white placeholder-zinc-500 text-sm focus:outline-none focus:border-indigo-500" />
          </div>
          <button onClick={compare} disabled={loading || !beforeB64 || !afterB64} className="px-8 py-3 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-40 disabled:cursor-not-allowed text-white font-semibold rounded-lg text-sm transition-colors">
            {loading ? "Comparing…" : "Compare Screens"}
          </button>
        </div>

        {error && <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-4 text-red-400 text-sm mb-6">{error}</div>}
        {loading && <div className="flex items-center justify-center py-16"><div className="flex flex-col items-center gap-3"><div className="w-10 h-10 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin" /><p className="text-zinc-400 text-sm">Analyzing visual differences…</p></div></div>}

        {result && vStyle && (
          <div className="flex flex-col gap-5">
            <div className={`${vStyle.bg} border ${vStyle.border} rounded-xl p-5 flex items-center gap-5`}>
              <div className="relative w-20 h-20 shrink-0">
                <svg className="w-20 h-20 -rotate-90" viewBox="0 0 36 36">
                  <circle cx="18" cy="18" r="16" fill="none" stroke="#27272a" strokeWidth="3" />
                  <circle cx="18" cy="18" r="16" fill="none" strokeWidth="3" strokeDasharray={`${result.diffScore} 100`} strokeLinecap="round" className={vStyle.text.replace("text-", "stroke-")} />
                </svg>
                <div className="absolute inset-0 flex items-center justify-center"><span className={`text-xl font-black ${vStyle.text}`}>{result.diffScore}</span></div>
              </div>
              <div>
                <div className={`text-xl font-black ${vStyle.text} mb-1`}>{result.verdict} · {result.overallDiff}</div>
                <p className="text-zinc-300 text-sm">{result.summary}</p>
              </div>
            </div>

            <div className="bg-zinc-900/60 border border-zinc-800 rounded-xl p-5">
              <h2 className="text-sm font-semibold text-white mb-4">Changes Detected ({result.changes.length})</h2>
              <div className="flex flex-col gap-3">
                {result.changes.map((c, i) => (
                  <div key={i} className="bg-zinc-800/60 rounded-lg p-3">
                    <div className="flex items-start justify-between gap-2 mb-1">
                      <span className="text-sm font-semibold text-zinc-200">{c.area}</span>
                      <div className="flex gap-1.5 shrink-0">
                        <span className={`text-xs rounded px-2 py-0.5 ${TYPE_COLORS[c.type] || "bg-zinc-700 text-zinc-300"}`}>{c.type}</span>
                        <span className={`text-[10px] font-medium border rounded-full px-2 py-0.5 ${SEV_COLORS[c.severity]}`}>{c.severity}</span>
                        {c.regression && <span className="text-[10px] bg-red-500/20 text-red-400 border border-red-500/30 rounded-full px-2 py-0.5">regression</span>}
                      </div>
                    </div>
                    <p className="text-xs text-zinc-400">{c.description}</p>
                  </div>
                ))}
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
              {result.regressions.length > 0 && <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-4"><h3 className="text-xs font-semibold text-red-400 uppercase tracking-wider mb-3">Regressions</h3><ul className="flex flex-col gap-2">{result.regressions.map((r, i) => <li key={i} className="text-sm text-zinc-300 flex gap-2"><span className="text-red-400 shrink-0">!</span>{r}</li>)}</ul></div>}
              {result.improvements.length > 0 && <div className="bg-emerald-500/10 border border-emerald-500/30 rounded-xl p-4"><h3 className="text-xs font-semibold text-emerald-400 uppercase tracking-wider mb-3">Improvements</h3><ul className="flex flex-col gap-2">{result.improvements.map((r, i) => <li key={i} className="text-sm text-zinc-300 flex gap-2"><span className="text-emerald-400 shrink-0">✓</span>{r}</li>)}</ul></div>}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
