ALTER TABLE leaderboards ADD COLUMN character TEXT NOT NULL DEFAULT '';
ALTER TABLE leaderboards ADD COLUMN goal TEXT NOT NULL DEFAULT '';
ALTER TABLE leaderboards ADD COLUMN game_version TEXT NOT NULL DEFAULT '';
ALTER TABLE leaderboards ADD COLUMN modifier TEXT NOT NULL DEFAULT '';

UPDATE leaderboards SET character = title;

ALTER TABLE leaderboards DROP COLUMN title;