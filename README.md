# Rustforge-Starter

Rustforge-Starter is the consumer application skeleton that depends on Rustforge framework crates.
Use this repository to build real products. Keep framework changes in Rustforge, keep domain logic here.

## Repository Layout

| Folder | Purpose |
| --- | --- |
| `app/` | Main application crate (API/websocket/worker/console binaries, internal modules, contracts, validation, seeds). |
| `generated/` | Generated crate from `db-gen` using `app/schemas`, `app/permissions.toml`, `app/configs.toml`. |
| `migrations/` | Application SQL migrations. |
| `i18n/` | Project-owned translation catalogs (`en.json`, `zh.json`, ...). |
| `public/` | Optional static output directory for built frontend assets (`PUBLIC_PATH`). |
| `bin/` | Short wrappers to run API/websocket/worker/console with expected env defaults. |
| `.env.example` | Runtime environment template. |
| `Cargo.toml` | Workspace root and Rustforge dependency wiring. |

## First Boot

1. Copy env and adjust values:

```bash
cp .env.example .env
```

2. Ensure PostgreSQL and Redis are running.
3. Generate code:

```bash
cargo build -p generated
```

4. Build migration files and run them:

```bash
./bin/console migrate pump
./bin/console migrate run
```

5. Start services:

```bash
./bin/api-server
./bin/websocket-server
./bin/worker
```

## Daily Commands

```bash
make check
make run-api
make run-ws
make run-worker
make migrate-pump
make migrate-run
make assets-publish ASSETS_ARGS="--from frontend/dist --clean"
make framework-docs-build
```

## Static Assets (Optional)

1. Keep `PUBLIC_PATH=public` (or set your own path in `.env`).
2. Build your frontend project (for example Vite `dist` output).
3. Publish files into `PUBLIC_PATH`:

```bash
./bin/console assets publish --from frontend/dist --clean
```

When `PUBLIC_PATH/index.html` exists, API server serves that folder at `/` with SPA fallback.

## Redis Key Isolation

Keep `REDIS_CACHE_PREFIX` empty by default. Framework auto-derives `{APP_NAME}_{APP_ENV}` to namespace keys.
Set `REDIS_CACHE_PREFIX` only when you need a custom prefix strategy.

## Where To Implement Code

- HTTP routes/handlers/router composition:
  - `app/src/internal/api/*`
- Middleware:
  - `app/src/internal/middleware/*`
- Domain workflows:
  - `app/src/internal/workflows/*`
- Jobs and schedules:
  - `app/src/internal/jobs/*`
- Realtime policies/channels wiring:
  - `app/src/internal/realtime/*`
- Request/response DTO contracts:
  - `app/src/contracts/*`
- Custom validation helpers:
  - `app/src/validation/*`

## Single Source of Truth Rules

- Models/enums/relations: `app/schemas/*.toml`
- Permissions catalog: `app/permissions.toml`
- App static config: `app/configs.toml`
- SQL schema changes: `migrations/*.sql`
- User-facing translations: `i18n/{lang}.json`
- Do not manually edit generated files under `generated/src/*`.

## Rustforge Dependency Mode

Current default is git dependency mode from:
`https://github.com/weiloon1234/Rustforge.git` (branch `main`).

For release stability, pin to a tag/revision once published:

```toml
# bootstrap = { git = "https://github.com/weiloon1234/Rustforge.git", tag = "v0.1.0" }
```

## i18n Ownership

This starter owns translation files.
`I18N_DIR=i18n` is set in `.env.example`, and API locale is resolved from `Accept-Language`/`x-locale` by framework middleware.

## Optional Framework Docs Route

`ENABLE_FRAMEWORK_DOCS` is `false` by default in `.env.example`.

1. Build framework docs frontend assets:

```bash
make framework-docs-build
```

`framework-docs-build` expects a local Rustforge checkout at `../Rustforge` (or override `RUSTFORGE_PATH`).

2. Enable docs and choose path in `.env`:

```bash
ENABLE_FRAMEWORK_DOCS=true
FRAMEWORK_DOCS_PATH=/framework-documentation
SERVER_PORT=4582
```

3. Start API and open:

```text
http://127.0.0.1:4582/framework-documentation
```
