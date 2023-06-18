CREATE TABLE sharing(
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(project_id) NOT NULL,
    friend_id INTEGER REFERENCES users(user_id) NOT NULL
);

CREATE TABLE tokens(
    id SERIAL PRIMARY KEY,
    token CHAR(64) NOT NULL,
    project_id INTEGER  NOT NULL
);

ALTER TABLE sharing ADD CONSTRAINT project_friend_id UNIQUE (project_id, friend_id)
