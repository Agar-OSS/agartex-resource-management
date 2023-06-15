ALTER TABLE resources
ADD CONSTRAINT project_id_name_unique
UNIQUE (project_id, name);
