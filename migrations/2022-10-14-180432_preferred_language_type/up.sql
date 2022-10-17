-- Your SQL goes here

ALTER TABLE users
ALTER COLUMN preferred_language TYPE uuid USING 'aea02f71-ab0d-470e-9d0d-3577ec870e29';