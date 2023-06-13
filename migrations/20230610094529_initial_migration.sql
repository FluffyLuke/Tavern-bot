create table banned_words (
    guild_id VARCHAR(30) NOT NULL,
    banned_word VARCHAR(30) NOT NULL
);

create table guilds (
    guild_id VARCHAR(30) NOT NULL,
    role_id VARCHAR(30) NOT NULL,
    PRIMARY KEY(guild_id)
);
