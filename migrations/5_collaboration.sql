CREATE TABLE collaboration(
    collab_id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(project_id) NOT NULL
    friend_id INTEGER REFERENCES users(user_id) DEFAULT NULL
);