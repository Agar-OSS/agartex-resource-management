CREATE TABLE sharing(
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(project_id) NOT NULL,
    friend_id INTEGER REFERENCES users(user_id), 
    token CHAR(64) NOT NULL UNIQUE
);
-- problem? many nullified entries for one project may be open 
ALTER TABLE sharing ADD CONSTRAINT project_friend_id UNIQUE (project_id, friend_id)
