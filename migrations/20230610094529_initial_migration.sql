create table banned_words (
    guild_id VARCHAR(30) NOT NULL,
    banned_word VARCHAR(30) NOT NULL
);

create table modetated_role (
    guild_id VARCHAR(30) NOT NULL,
    role_id VARCHAR(30) NOT NULL
);

create table basic_role (
    guild_id VARCHAR(30) NOT NULL,
    role_id VARCHAR(30) NOT NULL
);
