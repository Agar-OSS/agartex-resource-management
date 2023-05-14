CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(128) NOT NULL UNIQUE,
    password_hash VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    session_id CHAR(64) PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id),
    expires BIGINT NOT NULL
);


CREATE TABLE projects (
    project_id SERIAL PRIMARY KEY,
    main_document_id INTEGER UNIQUE,
    owner INTEGER REFERENCES users(user_id),
    name VARCHAR(128) NOT NULL
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
