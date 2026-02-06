# Packaging Guide

Rise RTS now ships as a web-first Pixi client that can be hosted on any static site provider.

## Prerequisites

- Bun (or npm) for building the web client

## Build the Web Client

```bash
cd web-client
bun install
bun run build
```

The optimized output is written to `web-client/dist/`.

## Deploy

Upload the `web-client/dist/` folder to your static host of choice:

- Cloudflare Pages
- Netlify
- Vercel
- GitHub Pages
- Any S3-compatible static hosting

## Optional: Local Preview

```bash
cd web-client
bun run dev
```

Then open the local URL printed by Vite.
