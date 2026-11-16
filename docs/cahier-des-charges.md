# Cahier des charges — e_school v1

> **Version de référence (en ligne, commentable) :** https://claude.ai/artifact/BP1z5D8M2jPW2i7JnScRVS
>
> Ce fichier est une copie figée au 16/11/2026. En cas d'écart, la version en ligne fait foi :
> c'est là que les sections laissent leurs remarques et mettent à jour le statut des lots.

## 1. Contexte et objectifs

e_school est un logiciel de gestion d'établissements d'enseignement, auto-hébergeable, dont la v1 cible les **universités ivoiriennes (système LMD)**. Le collège et le lycée suivront sur le même modèle de données.

Objectifs de la v1 :

- Gérer la vie académique d'une université : années, maquettes LMD, étudiants, enseignants, emplois du temps, présences, notes, moyennes et délibérations.
- Produire les documents officiels : bulletins et relevés de notes en PDF.
- Permettre à chaque établissement d'installer sa propre instance, avec des données totalement isolées.
- Permettre à un groupe de relier ses établissements dans une même instance : consolidation, transfert d'étudiants, enseignants partagés, référentiel de matières commun.

Ce document sert de référence commune. Chaque section l'utilise pour valider l'avancement et y laisse ses remarques en commentaire (voir section 10).

## 2. Périmètre

La v1 couvre l'université (LMD) de bout en bout, de l'inscription au bulletin. Le reste est reporté en v2.

| Domaine | Inclus en v1 | Reporté en v2 ou plus tard |
| --- | --- | --- |
| Types d'établissement | Université (LMD) | Collège, lycée, grandes écoles, BTS |
| Déploiement | Une instance = un groupe avec N établissements | Fédération entre serveurs séparés |
| Utilisateurs | Admin, direction, secrétariat, enseignants, étudiants | Parents |
| Années scolaires | Plusieurs années, périodes, clôture en lecture seule | — |
| Maquettes | Filières, niveaux, UE, ECUE, crédits, coefficients, copie d'une année à l'autre | — |
| Classes et étudiants | Promotions, groupes TD/TP, inscriptions annuelles, import Excel/CSV | — |
| Enseignants | Profil, affectations (matière + classe + année), plusieurs matières, charge horaire | Disponibilités, gestion fine des vacataires |
| Emploi du temps | Saisie manuelle, détection des conflits, salles facultatives | Génération automatique |
| Présences | Appel par séance issue de l'emploi du temps (présent / absent) | Retards, justificatifs, seuils d'alerte, appel hors ligne |
| Notes et moyennes | Types d'évaluation pondérés, rattrapage, rang, appréciations, délibération, publication | — |
| Documents | Bulletins et relevés de notes PDF | Attestations, certificats |
| Interface | Web responsive en français, PWA installable | Autres langues |
| Industrialisation | Docker Compose pour l'installation | CI/CD (mis de côté pour l'instant) |

## 3. Acteurs et droits

Six rôles en v1. Un rôle est attribué pour un établissement précis, sauf le super admin qui couvre tout le groupe. Un utilisateur peut cumuler plusieurs rôles et plusieurs établissements.

| Rôle | Périmètre | Peut faire |
| --- | --- | --- |
| Super admin | Groupe | Créer les établissements, gérer les admins, voir les tableaux consolidés, transférer un étudiant, partager un enseignant |
| Admin | Établissement | Paramétrage, années, maquettes, règles de calcul, comptes utilisateurs |
| Direction | Établissement | Tout consulter, délibérer, publier les notes et résultats, clôturer une période ou une année |
| Secrétariat | Établissement | Étudiants, inscriptions, classes, emplois du temps, imports, édition des bulletins |
| Enseignant | Ses affectations | Faire l'appel, créer ses évaluations, saisir ses notes, ajouter une appréciation, voir sa charge horaire |
| Étudiant | Lui-même | Voir son emploi du temps, ses absences, ses notes et résultats publiés, télécharger son bulletin |

Règle transversale : un enseignant ne voit et ne modifie que les classes et matières de ses affectations. Une note n'est plus modifiable par l'enseignant après verrouillage de l'évaluation.

## 4. Exigences fonctionnelles

Chaque exigence porte un identifiant (EF-xx), référencé dans les tickets, les tests et les remarques de validation. Une exigence est validée quand son critère d'acceptation est démontré.

### 4.1 Organisation et années scolaires

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-01 | Gérer le groupe et ses établissements | Un super admin crée 2 établissements ; un utilisateur de l'un ne voit aucune donnée de l'autre |
| EF-02 | Gérer plusieurs années scolaires | Une année passe par brouillon, active, clôturée ; une seule année active par établissement |
| EF-03 | Découper l'année en périodes | Une année LMD contient 2 semestres datés, sans chevauchement |
| EF-04 | Clôturer une année | Après clôture, toute modification des données de l'année est refusée ; la consultation reste possible |

### 4.2 Maquettes (filières, UE, ECUE)

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-10 | Gérer le catalogue de matières | Une matière peut être propre à l'établissement ou partagée au niveau du groupe |
| EF-11 | Gérer filières et niveaux | Création de « Licence Informatique » avec L1, L2, L3 |
| EF-12 | Gérer UE et ECUE | Une UE porte code, semestre, crédits ; ses ECUE portent coefficient, crédits, volume CM/TD/TP |
| EF-13 | Versionner la maquette par année | Modifier la maquette de l'année N ne change aucun résultat de l'année N-1 |
| EF-14 | Copier une maquette | À la création de l'année N+1, la maquette de N est copiée en un clic puis modifiable |

### 4.3 Étudiants, classes et inscriptions

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-20 | Gérer les étudiants | Matricule unique dans le groupe ; un étudiant existe une seule fois quelles que soient les années |
| EF-21 | Gérer promotions et groupes | Une promotion L1 contient des groupes TD et TP ; un étudiant appartient à un groupe de chaque type |
| EF-22 | Inscrire un étudiant pour l'année | Une inscription relie étudiant, année, niveau et promotion ; une seule inscription active par année |
| EF-23 | Importer depuis Excel/CSV | Import de 500 étudiants avec rapport ligne par ligne des erreurs, sans import partiel silencieux |
| EF-24 | Transférer un étudiant | Un étudiant change d'établissement du groupe en gardant son historique |
| EF-25 | Créer le compte étudiant | Chaque étudiant inscrit peut recevoir un compte de connexion |

### 4.4 Enseignants

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-30 | Gérer le profil enseignant | Profil avec statut permanent ou vacataire ; import Excel/CSV |
| EF-31 | Affecter un enseignant | Une affectation relie enseignant, ECUE, groupe, année et heures prévues ; un enseignant peut avoir plusieurs matières |
| EF-32 | Partager un enseignant | Un même enseignant est affecté dans 2 établissements du groupe |
| EF-33 | Suivre la charge horaire | Pour chaque enseignant : heures prévues, heures effectuées (séances réalisées), écart |

### 4.5 Emploi du temps

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-40 | Gérer les salles (facultatif) | Un établissement fonctionne sans aucune salle déclarée |
| EF-41 | Saisir des créneaux récurrents | Un créneau porte jour, heures, affectation, salle éventuelle et période de validité |
| EF-42 | Générer les séances | Les créneaux produisent les séances datées ; une séance peut être annulée ou déplacée |
| EF-43 | Détecter les conflits | Impossible d'enregistrer deux séances qui se chevauchent pour un même enseignant, groupe ou salle |
| EF-44 | Consulter l'emploi du temps | Vues semaine par groupe, enseignant et salle ; vue personnelle pour l'étudiant |

### 4.6 Présences

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-50 | Faire l'appel par séance | L'enseignant ouvre sa séance et marque chaque étudiant présent ou absent |
| EF-51 | Consulter les absences | Récapitulatif par étudiant, groupe et ECUE ; l'étudiant voit ses propres absences |

### 4.7 Notes, moyennes et délibérations

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-60 | Paramétrer les types d'évaluation | Devoir, interrogation, examen, CC, TP avec un poids par établissement |
| EF-61 | Créer une évaluation | Une évaluation porte ECUE, groupe, semestre, type, date, barème et session (normale ou rattrapage) |
| EF-62 | Saisir les notes | Saisie en grille, notes sur le barème ramenées sur 20, mention « absent » possible |
| EF-63 | Verrouiller une évaluation | Après verrouillage, seule la direction peut modifier une note, avec motif obligatoire |
| EF-64 | Calculer les moyennes | Moyennes d'ECUE, d'UE, de semestre et d'année conformes aux règles de la section 5 |
| EF-65 | Calculer le rang | Rang par groupe et par semestre, ex-aequo gérés |
| EF-66 | Gérer les appréciations | Appréciation automatique par tranche + commentaire libre facultatif |
| EF-67 | Gérer le rattrapage | Session 2 pour les étudiants ajournés, selon la règle choisie par l'établissement |
| EF-68 | Délibérer | La délibération fige les résultats ; une note modifiée après ne change pas un résultat délibéré sans nouvelle délibération |
| EF-69 | Publier | L'étudiant ne voit ses notes qu'après publication, et ses moyennes qu'après délibération |

### 4.8 Documents

| ID | Exigence | Critère d'acceptation |
| --- | --- | --- |
| EF-70 | Éditer le bulletin semestriel | PDF par étudiant et par lot (toute une promotion), avec en-tête de l'établissement |
| EF-71 | Éditer le relevé de notes | Relevé annuel avec UE, crédits acquis et décision |
| EF-72 | Personnaliser le modèle | Logo, nom, signataires configurables par établissement |

## 5. Règles de calcul

Les calculs suivent une chaîne fixe : évaluations, ECUE, UE, semestre, année. Les seuils et options sont choisis par chaque université ; les valeurs ci-dessous sont les défauts proposés. Toutes les notes sont ramenées sur 20 et calculées en décimal exact, arrondies à 2 décimales à l'affichage.

### 5.1 Chaîne de calcul

1. **Moyenne d'ECUE** : moyenne des évaluations de chaque type (CC, examen, TP…), puis moyenne pondérée par le poids des types présents. Exemple : 40 % CC + 60 % examen.
2. **Moyenne d'UE** : moyenne des ECUE pondérée par leur coefficient.
3. **Moyenne de semestre** : moyenne des UE pondérée par leurs crédits.
4. **Moyenne annuelle** : moyenne simple des deux semestres.

```latex
M_{ECUE} = \frac{\sum_t p_t \, \bar{n}_t}{\sum_t p_t} \qquad M_{UE} = \frac{\sum_e c_e \, M_e}{\sum_e c_e} \qquad M_{S} = \frac{\sum_u k_u \, M_u}{\sum_u k_u} \qquad M_{A} = \frac{M_{S1} + M_{S2}}{2}
```

Où p = poids du type d'évaluation, n̄ = moyenne des notes de ce type, c = coefficient de l'ECUE, k = crédits de l'UE.

### 5.2 Paramètres choisis par chaque université

| Paramètre | Options | Défaut proposé |
| --- | --- | --- |
| Poids des types d'évaluation | Libre par type | CC 40 %, examen 60 % |
| Pondération des UE dans le semestre | Crédits ou coefficient | Crédits |
| Seuil de validation d'une UE | Note sur 20 | 10 |
| Compensation entre UE d'un semestre | Oui / non | Oui, si moyenne de semestre ≥ 10 |
| Note éliminatoire (bloque la compensation) | Aucune ou seuil sur 20 | 5 |
| Compensation entre S1 et S2 | Oui / non | Non |
| Capitalisation des UE validées | Oui / non | Oui |
| Évaluation manquée | Compte 0 / exclue du calcul | Compte 0 |
| Note de rattrapage | Remplace / meilleure des deux / plafonnée à 10 | Meilleure des deux |
| Accès au rattrapage | UE non validées / ECUE sous le seuil | UE non validées |

### 5.3 Rang, appréciations et décisions

- **Rang** : calculé par groupe et par semestre sur la moyenne de semestre. Les ex-aequo partagent le même rang et le suivant est sauté (1, 2, 2, 4).
- **Appréciation automatique** par tranche, modifiable : moins de 10 Insuffisant ; 10 à 12 Passable ; 12 à 14 Assez bien ; 14 à 16 Bien ; 16 et plus Très bien. Un commentaire libre de l'enseignant ou du jury peut s'y ajouter.
- **Décisions** de semestre : Validé, Validé par compensation, Ajourné (rattrapage), Non validé. La décision annuelle découle des crédits acquis (60 pour une année complète).

Toute modification d'un paramètre s'applique aux calculs à venir ; les résultats déjà délibérés restent figés.

## 6. Modèle de données

Les étudiants et enseignants appartiennent au groupe ; tout le reste appartient à un établissement et porte son identifiant. Tous les identifiants sont des UUID v7, ce qui évite les collisions en cas de fusion ou de synchronisation future entre instances.

| Module | Entité | Rôle | Rattachement |
| --- | --- | --- | --- |
| organization | organization | Le groupe (un par instance) | — |
| organization | establishment | Un établissement, son type et ses paramètres | Groupe |
| academic | academic_year | Année scolaire et son statut | Établissement |
| academic | period | Semestre (ou trimestre en v2), dates, statut | Année |
| curriculum | subject | Matière du catalogue | Établissement ou groupe |
| curriculum | program, level | Filière et niveau (L1, M1…) | Établissement |
| curriculum | teaching_unit | UE : code, semestre, crédits | Niveau + année |
| curriculum | course_element | ECUE : coefficient, crédits, volume CM/TD/TP | UE |
| people | student, teacher | Identité persistante | Groupe |
| identity | user_account, role_assignment | Compte de connexion et rôles par périmètre | Groupe |
| enrollment | class_group | Promotion ou groupe TD/TP (avec parent) | Année + niveau |
| enrollment | enrollment | Inscription annuelle d'un étudiant | Année |
| enrollment | teaching_assignment | Enseignant × ECUE × groupe, heures prévues | Année |
| timetable | room | Salle (facultative) | Établissement |
| timetable | timetable_slot | Créneau récurrent | Affectation |
| timetable | session | Séance datée, statut | Créneau |
| attendance | attendance_record | Présence d'un étudiant à une séance | Séance |
| grading | evaluation_type | Type et poids | Établissement |
| grading | grading_policy | Paramètres de la section 5.2 | Établissement ou filière |
| grading | evaluation, grade | Évaluation et notes | ECUE + groupe + période |
| grading | *_result | Résultats figés : ECUE, UE, semestre, année | Délibération |
| audit | audit_log | Qui a modifié quoi, quand, ancienne et nouvelle valeur | Établissement |

## 7. Architecture technique et déploiement

e_school est un monolithe modulaire : un seul binaire Rust expose l'API et sert l'interface React compilée, devant une base PostgreSQL. Un groupe installe une instance (Docker Compose) et y déclare autant d'établissements que nécessaire.

```text
Navigateur ou PWA installée (React · TypeScript · TanStack · shadcn/ui)
        │  HTTPS · JSON · JWT
        ▼
┌─ Binaire unique (crate app, Axum), qui sert aussi le frontend compilé ─────┐
│  Couche HTTP : routes, authentification JWT, établissement actif, OpenAPI  │
│  identity · organization · academic · curriculum · people                  │
│  enrollment · timetable · attendance · grading · reporting                 │
│  kernel : UUID v7, erreurs, pagination, contexte d'établissement           │
└────────────────────────────────────────────────────────────────────────────┘
        │  SQLx · migrations
        ▼
PostgreSQL (Row-Level Security par établissement)
```

Chaque module : domain · application · infrastructure · http, et n'accède qu'à ses propres tables. Les modules ne s'appellent qu'au travers d'interfaces (traits) ; un module peut ainsi être réécrit ou extrait sans toucher aux autres.

| Couche | Choix |
| --- | --- |
| Backend | Rust, Axum, SQLx, rust_decimal, utoipa (OpenAPI), Typst (PDF) |
| Base de données | PostgreSQL uniquement, migrations SQLx |
| Frontend | React, TypeScript, Vite, TanStack Router / Query / Table, React Hook Form + Zod, shadcn/ui, vite-plugin-pwa |
| Contrat d'API | Client TypeScript généré depuis l'OpenAPI |
| Installation | Docker Compose : application + PostgreSQL |

```text
e_school/
├── backend/
│   ├── Cargo.toml          # workspace
│   ├── migrations/
│   └── crates/             # kernel, identity, organization, academic, curriculum,
│                           # people, enrollment, timetable, attendance, grading,
│                           # reporting, app
├── frontend/
│   └── src/
│       ├── app/            # routeur, providers, mise en page
│       ├── features/       # un dossier par domaine métier
│       └── shared/         # composants UI, client API généré
└── deploy/                 # Docker Compose, guide d'installation
```

## 8. Exigences non fonctionnelles

| ID | Thème | Exigence |
| --- | --- | --- |
| ENF-01 | Isolation | Chaque requête est filtrée par établissement dans le code ET par Row-Level Security PostgreSQL |
| ENF-02 | Authentification | JWT d'accès de 15 min + jeton de rafraîchissement avec rotation, stocké haché en base et révocable |
| ENF-03 | Mots de passe | Hachage Argon2id ; verrouillage temporaire après 5 échecs |
| ENF-04 | Autorisations | Contrôle côté serveur sur chaque action ; l'interface ne fait que masquer |
| ENF-05 | Traçabilité | Toute création, modification ou suppression de note, inscription ou résultat est inscrite au journal d'audit |
| ENF-06 | Exactitude | Calculs en décimal exact, jamais en flottant ; moteur de calcul couvert par des tests unitaires sur chaque règle |
| ENF-07 | PWA | Application installable ; emploi du temps et notes consultables hors ligne en lecture |
| ENF-08 | Performance | Liste paginée de 10 000 étudiants en moins de 500 ms ; calcul d'une promotion de 1 000 étudiants en moins de 5 s |
| ENF-09 | Responsive | Utilisable sur téléphone (appel, consultation) et sur ordinateur (saisie, administration) |
| ENF-10 | Installation | Une commande Docker Compose démarre l'application et PostgreSQL ; migrations appliquées au démarrage |
| ENF-11 | Sauvegarde | Script de sauvegarde et restauration PostgreSQL documenté |
| ENF-12 | Langue | Interface en français ; textes centralisés pour permettre d'autres langues plus tard |
| ENF-13 | Qualité du code | Modules découplés, aucune lecture directe des tables d'un autre module ; formatage et lint obligatoires |

## 9. Plan de livraison et avancement

La v1 est livrée en 10 lots, dans l'ordre des dépendances : chaque lot s'appuie sur les précédents. Le statut à jour est tenu dans la version en ligne.

| Lot | Contenu | Exigences | Statut au 16/11/2026 |
| --- | --- | --- | --- |
| 0 — Socle | Workspace Rust, squelette React, Docker Compose, migrations, OpenAPI, gestion des erreurs | ENF-10, ENF-13 | À faire |
| 1 — Authentification et droits | Comptes, JWT, rôles, choix de l'établissement actif, journal d'audit | ENF-02 à ENF-05 | À faire |
| 2 — Organisation et années | Groupe, établissements, années, semestres, clôture | EF-01 à EF-04, ENF-01 | À faire |
| 3 — Maquettes | Matières, filières, niveaux, UE, ECUE, copie d'année | EF-10 à EF-14 | À faire |
| 4 — Personnes et inscriptions | Étudiants, enseignants, promotions, groupes, inscriptions, affectations, imports | EF-20 à EF-25, EF-30 à EF-32 | À faire |
| 5 — Emploi du temps | Salles, créneaux, séances, conflits, vues | EF-40 à EF-44 | À faire |
| 6 — Présences | Appel par séance, récapitulatifs | EF-50, EF-51 | À faire |
| 7 — Notes | Types d'évaluation, évaluations, saisie, verrouillage | EF-60 à EF-63 | À faire |
| 8 — Moyennes et délibérations | Moteur de calcul, rang, appréciations, rattrapage, délibération, publication, PDF | EF-64 à EF-72, ENF-06 | À faire |
| 9 — Espace étudiant et finitions | Espace étudiant, charge horaire, PWA, sauvegardes, performance | EF-33, ENF-07 à ENF-12 | À faire |

## 10. Validation et remarques

Un lot est validé quand une démonstration montre chacun de ses critères d'acceptation et qu'aucune remarque bloquante n'est ouverte.

1. L'équipe de développement passe le lot « En recette » dans le tableau de la section 9.
2. Chaque section vérifie les exigences qui la concernent (identifiants EF / ENF).
3. Les remarques se font en **commentaire, ancré sur l'exigence concernée**, en commençant par un niveau : `[Bloquant]`, `[À corriger]` ou `[Suggestion]`.
4. Une remarque bloquante passe le lot « Bloqué » jusqu'à sa résolution.
5. Quand toutes les remarques bloquantes sont résolues, la personne qui valide passe le lot « Validé » et inscrit son nom dans la colonne « Validé par ».

Toute demande de changement de périmètre est inscrite dans le journal ci-dessous avant d'être développée.

### Journal des décisions

| Date | Décision |
| --- | --- |
| 16/11/2026 | V1 centrée sur l'université (LMD) ; collège et lycée ensuite |
| 16/11/2026 | Une instance = un groupe avec N établissements isolés ; fédération entre serveurs reportée |
| 16/11/2026 | Règles LMD (compensation, éliminatoire, rattrapage) paramétrables par université |
| 16/11/2026 | Rattrapage inclus en v1 ; grandes écoles et BTS en v2 ; parents en v2 |
| 16/11/2026 | Notes visibles par l'étudiant après publication ; moyennes après délibération |
| 16/11/2026 | Retards, justificatifs et seuils d'alerte d'absence reportés en v2 |
| 16/11/2026 | Stack : Rust (Axum, SQLx), PostgreSQL, React + TypeScript, JWT, PWA ; CI/CD mis de côté |
