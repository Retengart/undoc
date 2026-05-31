# @iyulab/undoc (WASM)

Browser/Node.js WebAssembly package for [undoc](https://github.com/iyulab/undoc).

Extracts DOCX, XLSX, and PPTX files to Markdown, plain text, or JSON — no server required.

## Install

```bash
npm install @iyulab/undoc
```

## Usage (ESM)

```js
import init, { parse } from '@iyulab/undoc';
await init();

const bytes = new Uint8Array(await file.arrayBuffer());
const doc = parse(bytes);

console.log(doc.format());       // "docx" | "xlsx" | "pptx"
console.log(doc.toMarkdown());   // Markdown string
console.log(doc.toText());       // Plain text string
console.log(doc.toJson());       // JSON string
```

## API

### `parse(data: Uint8Array): OfficeDocument`

Parse a DOCX, XLSX, or PPTX byte array. Throws if the format is unrecognized.

### `OfficeDocument`

| Method | Returns | Description |
|--------|---------|-------------|
| `fromBytes(data)` | `OfficeDocument` | Alias for module-level `parse()` |
| `format()` | `string` | `"docx"` \| `"xlsx"` \| `"pptx"` |
| `toMarkdown()` | `string` | Full document as Markdown |
| `toText()` | `string` | Plain text extraction |
| `toJson()` | `string` | Structured JSON |
| `metadata()` | `string` | JSON of title/author/subject etc. |

## Playground

Live demo: https://iyulab.github.io/undoc/
