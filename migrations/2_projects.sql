CREATE TABLE projects (
    project_id SERIAL PRIMARY KEY,
    main_document_id INTEGER UNIQUE,
    owner_id INTEGER REFERENCES users(user_id),
    project_name VARCHAR(128) NOT NULL
);

CREATE TABLE documents (
    document_id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(project_id),
    name VARCHAR(128) NOT NULL
);

CREATE TABLE resources(
    resource_id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(project_id),
    name VARCHAR(128) NOT NULL
);

ALTER TABLE projects
ADD CONSTRAINT main_document_key
FOREIGN KEY (main_document_id) 
REFERENCES documents (document_id)
