# ScreenDiffAI 🖼️🔍

**AI-powered visual diff for digital signage — upload before and after screenshots and get a Pass/Review needed/Fail verdict with change-by-change regression analysis.**

Upload a before and after screen capture, add optional context, and get a full visual comparison: overall diff rating, diff score, per-change cards (area, type, severity, regression flag), regression list, improvement list, verdict, and an AI summary.

---

## 🌟 Features

### 🖼️ Core Features
- ✅ **Before Screenshot** — drag-and-drop or click to upload the baseline image
- ✅ **After Screenshot** — drag-and-drop or click to upload the updated image
- ✅ **Context Input** — optional description of what changed (e.g. "template update v2.4")
- ✅ **Overall Diff Rating** — Identical / Minor changes / Significant changes / Major regression
- ✅ **Diff Score** — 0–100 visual difference score
- ✅ **Change Cards** — per change: area, type (Layout/Content/Color/Typography/Missing element/New element), severity (critical/high/medium/low), description, regression flag
- ✅ **Regressions** — list of confirmed regressions found
- ✅ **Improvements** — list of positive changes detected
- ✅ **Verdict** — Pass ✅ / Review needed ⚠️ / Fail ❌
- ✅ **Summary** — one-paragraph visual comparison narrative

### 🤖 AI Features
- ✅ **Claude Sonnet 4.6 Vision** — analyzes both screenshots pixel-by-pixel with multimodal AI
- ✅ **Change classification** — distinguishes Layout from Content from Color from Typography changes
- ✅ **Regression detection** — flags changes that degrade rather than improve the display
- ✅ **Improvement recognition** — positive changes are explicitly called out
- ✅ **Context-aware** — optional context shifts interpretation (e.g. intentional redesign vs hotfix)

### ⚙️ Technical Features
- ✅ **Next.js 15 App Router** — server + client components
- ✅ **TypeScript strict mode** — fully typed change and verdict interfaces
- ✅ **Tailwind CSS** — dark indigo theme with side-by-side drop zones and verdict color coding
- ✅ **Vision API** — both images passed as base64 to Claude multimodal endpoint

---

## 🏗️ Architecture

```
ScreenDiffAI/
├── 📁 app/
│   ├── 📄 page.tsx          # Main UI — before/after drop zones + diff report
│   ├── 📄 layout.tsx        # Root layout with dark background
│   ├── 📄 globals.css       # Global styles
│   └── 📁 api/
│       └── 📁 compare/
│           └── 📄 route.ts  # POST /api/compare — Claude visual diff analyzer
├── 📁 public/               # Static assets
├── 📄 .env.example          # Environment variable template
├── 📄 package.json
└── 📄 README.md
```

---

## 🖥️ UI Overview

| Section | Description |
|---|---|
| **Before Drop Zone** | Drag-and-drop or click — baseline screenshot |
| **After Drop Zone** | Drag-and-drop or click — updated screenshot |
| **Context Input** | Optional description of the change |
| **Compare Screens** | Triggers Claude visual diff analysis |
| **Verdict** | Pass / Review needed / Fail with color badge |
| **Overall Diff** | Identical / Minor / Significant / Major regression |
| **Diff Score** | 0–100 visual change score |
| **Change Cards** | Area, type chip, severity badge, description, regression flag |
| **Regressions** | Red list of confirmed regressions |
| **Improvements** | Green list of positive changes |
| **Summary** | Visual comparison narrative |

---

## 🚀 Getting Started

### Prerequisites
- Node.js 18+
- pnpm
- Anthropic API key ([console.anthropic.com](https://console.anthropic.com))

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/SUDARSHANCHAUDHARI/ScreenDiffAI.git
   cd ScreenDiffAI
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Set up environment**
   ```bash
   cp .env.example .env.local
   # Edit .env.local and add your ANTHROPIC_API_KEY
   ```

4. **Run dev server**
   ```bash
   pnpm dev
   ```
   Open [http://localhost:3000](http://localhost:3000)

---

## 📜 Scripts

```bash
pnpm dev      # Start development server (Turbopack)
pnpm build    # Production build
pnpm start    # Start production server
pnpm lint     # ESLint check
```

---

## 🔑 Environment Variables

| Variable | Description | Required |
|---|---|---|
| `ANTHROPIC_API_KEY` | Your Anthropic API key | ✅ Yes |

Get your key at [console.anthropic.com](https://console.anthropic.com). Add it to `.env.local` — this file is gitignored and never committed.

---

## 📊 Current Status

| Property | Value |
|---|---|
| **Version** | 1.0.0 |
| **Status** | ✅ MVP Complete |
| **Model** | claude-sonnet-4-6 (vision) |
| **Verdicts** | 3 (Pass, Review needed, Fail) |
| **Diff Ratings** | 4 (Identical, Minor changes, Significant changes, Major regression) |
| **Change Types** | 6 (Layout, Content, Color, Typography, Missing element, New element) |
| **Severities** | 4 (critical, high, medium, low) |

---

## 🛠️ Tech Stack

| Component | Technology |
|---|---|
| **Framework** | Next.js 15 (App Router) |
| **Language** | TypeScript (strict mode) |
| **Styling** | Tailwind CSS |
| **AI** | Claude API — claude-sonnet-4-6 (multimodal vision) |
| **Package Manager** | pnpm |

---

## 🔒 Security

- `ANTHROPIC_API_KEY` lives in `.env.local` — gitignored, never committed
- `.env.example` contains placeholder values only
- API key sent directly to Anthropic — no intermediate server
- Screenshots not stored or logged server-side

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/your-feature`)
3. Commit your changes (`git commit -m 'feat: add your feature'`)
4. Push to the branch (`git push origin feat/your-feature`)
5. Open a Pull Request

---

## 📞 Support

- 🐛 Issues: [GitHub Issues](https://github.com/SUDARSHANCHAUDHARI/ScreenDiffAI/issues)

---

<div align="center">

**Made with ❤️ by [SUDARSHANCHAUDHARI](https://github.com/SUDARSHANCHAUDHARI)**

[⭐ Star this repo](https://github.com/SUDARSHANCHAUDHARI/ScreenDiffAI) · [🐛 Report Issue](https://github.com/SUDARSHANCHAUDHARI/ScreenDiffAI/issues)

</div>
