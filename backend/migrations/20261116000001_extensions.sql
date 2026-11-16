-- Extensions utilisées par les modules :
-- btree_gist : contraintes d'exclusion (conflits d'emploi du temps, lot 5)
-- citext     : emails et identifiants insensibles à la casse
CREATE EXTENSION IF NOT EXISTS btree_gist;
CREATE EXTENSION IF NOT EXISTS citext;
