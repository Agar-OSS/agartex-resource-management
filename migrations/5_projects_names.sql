ALTER TABLE projects
RENAME COLUMN  owner
TO owner_id;

ALTER TABLE projects
RENAME COLUMN name 
TO project_name;