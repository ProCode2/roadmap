-- Add up migration script here
CREATE UNIQUE INDEX mapid_tagid_uniq
ON map_tag (map_id, tag_id); 
