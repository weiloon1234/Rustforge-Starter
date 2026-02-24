# Frontend — Multi-Portal React + Vite + Tailwind 4

This directory contains the frontend source for the Rustforge starter. It ships two independent SPA portals:

| Portal | Base | Dev port | Build output |
|--------|------|----------|--------------|
| **user** | `/` | 5173 | `../public/` (root) |
| **admin** | `/admin/` | 5174 | `../public/admin/` |

Each portal has its own Vite config, HTML entry, CSS theme, and source tree.

## Directory Structure

```
frontend/
├── package.json
├── postcss.config.js
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.user.ts
├── vite.config.admin.ts
├── user.html
├── admin.html
└── src/
    ├── shared/            # Cross-portal components, hooks, utilities
    ├── user/              # User portal source
    │   ├── main.tsx
    │   ├── App.tsx
    │   └── app.css        # Tailwind 4 theme (@theme block)
    └── admin/             # Admin portal source
        ├── main.tsx
        ├── App.tsx
        └── app.css        # Tailwind 4 theme (@theme block)
```

## Commands

```bash
make dev              # All: Vite user + Vite admin + Rust API
make dev-user         # Vite user portal only (port 5173)
make dev-admin        # Vite admin portal only (port 5174)
make dev-api          # Rust API only (cargo-watch, port 3000)
make build-frontend   # Clean build all portals → public/
```

## Tailwind CSS 4

Each portal customises its design tokens in its own `app.css` via `@theme { }`. No `tailwind.config.js` is used — Tailwind 4 reads theme configuration from CSS.

```css
@import "tailwindcss";

@theme {
  --color-primary: #2563eb;
}
```

## Adding a New Portal

1. Create `vite.config.{name}.ts` — set `base`, `server.port`, `build.outDir`.
2. Create `{name}.html` entry point.
3. Create `src/{name}/` with `main.tsx`, `App.tsx`, `app.css`.
4. Add `dev:{name}` and `build:{name}` scripts to `package.json`.
5. Update the `build` script ordering (build nested portals first).
6. In Rust, add `nest_service("/{name}", ...)` in `build_router` (see `app/src/internal/api/mod.rs`).

## Production

`make build-frontend` writes optimised assets into `public/`. The Rust API serves them:
- `/admin/*` → `public/admin/index.html` (admin SPA fallback)
- `/*` → `public/index.html` (user SPA fallback)
