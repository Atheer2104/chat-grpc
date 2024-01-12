CREATE TABLE auth_tokens(
    auth_token TEXT NOT NULL,
    user_id SERIAL NOT NULL REFERENCES account (user_id),
    PRIMARY KEY (auth_token)
)
