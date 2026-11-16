# e_school

Gestion d'établissements d'enseignement (v1 : universités, système LMD).
Cahier des charges : [docs/cahier-des-charges.md](docs/cahier-des-charges.md)
(référence en ligne : https://claude.ai/artifact/BP1z5D8M2jPW2i7JnScRVS).

| Dossier | Contenu |
| --- | --- |
| `backend/` | Workspace Rust : `crates/kernel` (socle partagé), `crates/app` (API Axum), `migrations/` |
| `frontend/` | React + TypeScript + Vite, TanStack Router/Query, shadcn/ui |
| `deploy/` | Docker Compose de développement et de production |

## Développement

```bash
# 1. Base de données (port 5433)
docker compose -f deploy/compose.dev.yml up -d

# 2. API sur :8080 (lit backend/.env, applique les migrations)
cd backend && cp -n .env.example .env && cargo run -p app

# 3. Interface sur :5173 (proxy /api vers :8080)
cd frontend && npm install && npm run dev
```

- Documentation interactive de l'API : http://localhost:8080/api/docs
- Après un changement d'API : `cd frontend && npm run api:generate` (régénère le client typé)

## Tests

```bash
cd backend && cargo test          # nécessite la base de dev (étape 1)
cd backend && cargo clippy --all-targets && cargo fmt --check
cd frontend && npm test && npm run typecheck && npm run lint
```

## Production

```bash
cp deploy/.env.example deploy/.env   # puis définir POSTGRES_PASSWORD
docker compose -f deploy/docker-compose.yml --env-file deploy/.env up -d --build
```

L'application écoute sur le port `APP_PORT` (8080 par défaut) et applique les migrations au démarrage.

## Conventions

- Tests écrits avant le code ; un test qui échoue se corrige dans le code, pas dans le test.
- Backend : aucun fichier de plus de 50 lignes ; un crate par module métier, sans accès aux tables des autres modules.
- Erreurs d'API au format `application/problem+json` ; chaque réponse porte un `x-request-id`.
