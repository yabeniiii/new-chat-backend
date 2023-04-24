CREATE TABLE users (
    id serial PRIMARY KEY UNIQUE,
    email text UNIQUE NOT NULL,
    display_name text NOT NULL,
    display_color char (6),
    avatar_url text
);
CREATE TABLE messages (
    id serial PRIMARY KEY,
    sender_id int REFERENCES users (id) NOT NULL,
    content text NOT NULL
)
