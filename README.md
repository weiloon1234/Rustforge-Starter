# Rustforge Starter

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
./console migrate pump
./console migrate run
```

5. Start services:

```bash
./bin/api-server
./bin/websocket-server
./bin/worker
```

## Daily Commands

```bash
make dev
make check
make run-api
make run-ws
make run-worker
./console migrate pump
./console migrate run
make server-install
make server-update
make framework-docs-build
```

## Ubuntu Server Install (Interactive)

Run as root on Ubuntu 24/25:

```bash
sudo ./scripts/install-ubuntu.sh
# or
make server-install
```

The installer is idempotent (safe to run multiple times) and will:
- create/reuse an isolated Linux user per project
- configure SSH access (copy root key, manual key, or generated password)
- recursively `chown` project files to the isolated user
- upsert `.env` values (domain/env/db/redis/ports)
- generate/update nginx site config
- optionally configure Supervisor programs
- optionally issue/renew Let's Encrypt certificates with cron renewal

## Server Update Script

Use the generated update helper for deploy-like updates:

```bash
./scripts/update.sh
# optional opt-out
RUN_MIGRATIONS=false ./scripts/update.sh
```

It will:
- `git pull --ff-only`
- compile release binaries (`cargo build --release --workspace`)
- run migrations by default (set `RUN_MIGRATIONS=false` to skip)
- reread/update and restart Supervisor programs from the installed supervisor config

## i18n Ownership

This starter owns translation files.
`I18N_DIR=i18n` is set in `.env.example`, and API locale is resolved from `Accept-Language`/`x-locale` by framework middleware.

## Static Assets (Optional)

1. Keep `PUBLIC_PATH=public` (or set your own path in `.env`).
2. Build your frontend project (for example Vite `dist` output).
3. Publish files into `PUBLIC_PATH`:

```bash
./console assets publish --from frontend/dist --clean
```

When `PUBLIC_PATH/index.html` exists, API server serves that folder at `/` with SPA fallback.

## Redis Key Isolation

Keep `REDIS_CACHE_PREFIX` empty by default. Framework auto-derives `{APP_NAME}_{APP_ENV}` to namespace keys.
Set `REDIS_CACHE_PREFIX` only when you need a custom prefix strategy.

## Dependency Mode

This starter uses git dependencies to Rustforge.
For production stability, pin to a tag in `Cargo.toml`.

`make framework-docs-build` publishes framework docs assets into
`PUBLIC_PATH + FRAMEWORK_DOCS_PATH` (default: `public/framework-documentation`).
