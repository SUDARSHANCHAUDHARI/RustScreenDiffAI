import Anthropic from "@anthropic-ai/sdk";
import { NextRequest, NextResponse } from "next/server";
const client = new Anthropic({ apiKey: process.env.ANTHROPIC_API_KEY });
export async function POST(req: NextRequest) {
  const { beforeImage, afterImage, mimeType, context } = await req.json();
  if (!beforeImage || !afterImage) return NextResponse.json({ error: "Both images required" }, { status: 400 });
  const prompt = `You are a digital signage QA expert. Compare these two screen captures (before and after) and identify visual regressions, layout changes, and content differences.
Context: ${context || "Signage content update comparison"}
Return JSON: { "overallDiff": "Identical"|"Minor changes"|"Significant changes"|"Major regression", "diffScore": 0-100, "changes": [{ "area": "string", "type": "Layout"|"Content"|"Color"|"Typography"|"Missing element"|"New element", "severity": "critical"|"high"|"medium"|"low", "description": "string", "regression": boolean }], "regressions": ["string"], "improvements": ["string"], "verdict": "Pass"|"Review needed"|"Fail", "summary": "string" }
Return ONLY valid JSON.`;
  try {
    const msg = await client.messages.create({ model: "claude-sonnet-4-6", max_tokens: 1200, messages: [{ role: "user", content: [{ type: "image", source: { type: "base64", media_type: mimeType, data: beforeImage } }, { type: "image", source: { type: "base64", media_type: mimeType, data: afterImage } }, { type: "text", text: prompt }] }] });
    return NextResponse.json(JSON.parse((msg.content[0] as { type: string; text: string }).text));
  } catch (e) { return NextResponse.json({ error: String(e) }, { status: 500 }); }
}
