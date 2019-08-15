-- Your SQL goes here
CREATE TABLE comics (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  issueNumber FLOAT,
  description TEXT
);

COPY comics FROM '/tmp/data/comics_sanitized.csv' DELIMITER ';' CSV HEADER;

CREATE TABLE characters (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL
);

COPY characters FROM '/tmp/data/characters_sanitized.csv' DELIMITER ';' CSV HEADER;

CREATE TABLE characters_stats (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  alignment VARCHAR NOT NULL,
  intelligence INTEGER NOT NULL,
  strengh INTEGER NOT NULL,
  speed INTEGER NOT NULL,
  durability INTEGER NOT NULL,
  power INTEGER NOT NULL,
  combat INTEGER NOT NULL,
  total INTEGER NOT NULL
);

COPY characters_stats FROM '/tmp/data/characters_stats_sanitized.csv' DELIMITER ';' CSV HEADER;

CREATE TABLE characters_to_comics (
  id INTEGER PRIMARY KEY,
  comics_id INTEGER NOT NULL,
  characters_id INTEGER NOT NULL
);

COPY characters_to_comics FROM '/tmp/data/charactersToComics_sanitized.csv' DELIMITER ';' CSV HEADER;
