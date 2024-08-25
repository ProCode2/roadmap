-- Add down migration script here
ALTER TABLE map_tag DROP CONSTRAINT mapid_tagid_uniq;
