CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);