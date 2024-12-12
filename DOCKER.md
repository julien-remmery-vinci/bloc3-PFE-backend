## Setup docker et lancement
Assurez vous d'avoir la structure de dossiers ci présente
```
.
├── compose.yml
├── init_db/
├── migrations/
├── backend/
│   ├── Dockerfile
│   └── (source code files)
└── frontend/
    ├── Dockerfile
    └── (source code files)
```

## Lancer le docker compose
```console
docker compose up -d
```

## Accéder aux applications
- L'application frontend est disponnible sur le port **4200**.
- L'application backend est disponnible sur le port **3000**.
- La base de données est disponnible sur le port **5432**.