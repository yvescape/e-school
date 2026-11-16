# Cahier des charges

- Référence en ligne (fait foi, commentée par les sections) : https://claude.ai/artifact/BP1z5D8M2jPW2i7JnScRVS
- Copie locale : [docs/cahier-des-charges.md](docs/cahier-des-charges.md)
- Toute fonctionnalité développée référence ses exigences (EF-xx / ENF-xx).

# Utilisation des skills

- Avant chaque tâche, consulte la liste des skills disponibles et utilise
  ceux dont la description correspond à la tâche. Ne te contente pas de tes
  connaissances par défaut si un skill couvre le sujet.
- Combine plusieurs skills quand la tâche couvre plusieurs phases
  (ex. : spécification → plan → implémentation → tests → revue).
- Adapte-toi au projet : lis d'abord sa structure, sa stack et ses conventions
  (README, package.json, CLAUDE.md du projet, DESIGN.md, CONSTRAINTS.md…), puis
  choisis les skills pertinents pour ce contexte. Ignore ceux qui ne
  s'appliquent pas (ex. : pas de skill frontend sur un projet backend pur).
- Si deux skills se recoupent, privilégie le plus spécifique et dis-moi
  lequel tu utilises.
- Annonce en une ligne les skills que tu vas utiliser avant de commencer,
  sans me demander de validation sauf si le choix change le résultat.


# test 
ecrit les tests avant le code de la fonctionnalite, la logique de la fonctionnalite doit est valide par le test, un code ne doit pas a lui seul change un test si le test echoue 

# Code Modulaire 
je ne veux pas de fichier de code superieurs a 50 lignes pour le backend 

# API
le choix du type api doit dependre de utilite et du plus performant dans la situation. pas utilise api rest seulement ou graph seulement 

