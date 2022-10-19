-- This file should undo anything in `up.sql`

ALTER TABLE submissions
ALTER COLUMN issued_at DROP NOT NULL;