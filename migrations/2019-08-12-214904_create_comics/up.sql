-- Your SQL goes here
CREATE TABLE comics (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  issueNumber FLOAT,
  description TEXT
);

COPY comics FROM '/tmp/data/comics_sanitized.csv' DELIMITER ';' CSV HEADER;
