create table banned_words (
    guild_id VARCHAR(30) NOT NULL,
    banned_word VARCHAR(30) NOT NULL
);

create table guild (
    guild_id VARCHAR(30) NOT NULL,
    moderated_role_id VARCHAR(30),
    basic_role_id VARCHAR(30),
    PRIMARY KEY (guild_id)
);
