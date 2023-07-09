use sqlx::{Sqlite, Executor, Pool};
use serenity::utils::MessageBuilder;
#[derive(Debug)]
pub struct GuildDescription<> {
    guild_id: String,
    moderated_role_id: Option<String>,
    basic_role_id: Option<String>,
    banned_words: Vec<String>
}


impl GuildDescription {
    pub async fn build(database: &Pool<Sqlite>, guild_id: &str) -> Result<GuildDescription, sqlx::Error> 
    {
        let query = sqlx::query!("Select * from guild where guild_id = ?", guild_id)
        .fetch_one(database)
        .await;
        match query {
            Ok(guild_record) => {
                let mut banned_words= Vec::<String>::new(); 
                let query = sqlx::query!("SELECT * FROM banned_words where guild_id = ?", guild_id)
                    .fetch_all(database)
                    .await;
                match query {
                    Ok(query) => {
                        for record in query.iter() {
                            banned_words.push(record.banned_word.clone());
                        }
                    }
                    Err(sqlx::Error::RowNotFound) => { }
                    Err(err) => { return Err(err)}
                }
                return Ok(GuildDescription {
                guild_id: guild_record.guild_id,
                moderated_role_id: guild_record.moderated_role_id,
                basic_role_id: guild_record.basic_role_id,
                banned_words: banned_words,
                }) 
            },
            Err(sqlx::Error::RowNotFound) => {
                sqlx::query!("Insert into guild (guild_id) values (?)", guild_id)
                .execute(database)
                .await?;
                return Ok(GuildDescription {
                    guild_id: guild_id.to_string(),
                    moderated_role_id: Some("ERR".to_string()),
                    basic_role_id: Some("ERR".to_string()),
                    banned_words: Vec::<String>::new(),
                })
            },
            Err(err) => return Err(err), 
        }
    }

    pub fn get_guild_id(&self) -> &str {
        &self.guild_id
    }
    pub fn get_moderated_role_id(&self) -> Option<&str> {
        if let Some(id) = &self.moderated_role_id {
            return Some(id)
        }
        None
    }
    pub fn get_basic_role_id(&self) -> Option<&str> {
        if let Some(id) = &self.basic_role_id {
            return Some(&id)
        }
        None
    }
    pub fn guild_description_msg(&self) -> String{
        let mut response = MessageBuilder::new();
        
        //Guild id;
        response.push("Guild id: ").push_bold_line(self.get_guild_id());

        // Moderated role
        response.push("Id of moderated role: ");
        if let Some(id) = self.get_moderated_role_id() {
            response.push_bold_line(id);
        } else {
            response.push_italic_line("None");
        }

        // Basic role
        response.push("Id of basic role: ");
        if let Some(id) = self.get_basic_role_id() {
            response.push_bold_line(id);
        } else {
            response.push_italic_line("None");
        }

        // Banned words
        response.push_line("List of banned words: ");
        if self.get_banned_words().is_empty() {
            response.push_italic_line("None");
        } else {
            for word in self.get_banned_words().iter() {
                response.push("> ").push_bold_line(word);
            }
        }
        response.build()
    }
    pub async fn create_moderated_role<'a, D>(&mut self, database: D, moderated_role: String) -> Result<(), sqlx::Error> 
    where
        D: Executor<'a, Database = Sqlite>
    {
        sqlx::query!("update guild set moderated_role_id = (?) where guild_id = ?", moderated_role, self.guild_id)
            .execute(database)
            .await?;
        self.moderated_role_id = Some(moderated_role);
        Ok(())
    }
    pub async fn delete_moderated_role<'a, D>(&mut self, database: D) -> Result<(), sqlx::Error> 
    where
        D: Executor<'a, Database = Sqlite>
    {
        sqlx::query!("update guild set moderated_role_id = NULL where guild_id = ?", self.guild_id)
            .execute(database)
            .await?;
        self.moderated_role_id = None;
        Ok(())
    }
    pub async fn add_word_to_moderate<'a, D>(&mut self, database: D, word: String) -> Result<(), sqlx::Error> 
    where
        D: Executor<'a, Database = Sqlite>
    {
        sqlx::query!("INSERT INTO banned_words (guild_id, banned_word) VALUES (?, ?)", self.guild_id, word)
            .execute(database)
            .await?;
        self.banned_words.push(word);
        Ok(())
    }
    pub async fn remove_word_from_moderation<'a, D>(&mut self, database: D, word: &str) -> Result<(), sqlx::Error> 
    where
        D: Executor<'a, Database = Sqlite>
    {
        sqlx::query!("DELETE FROM banned_words WHERE guild_id = ? and banned_word = ?", self.guild_id, word)
            .execute(database)
            .await?;
        if let Some(index) = self.banned_words.iter().position(|value| *value == word){
            self.banned_words.remove(index);
        }
        Ok(())
    }
    pub fn get_banned_words(&self) -> &Vec<String>
    where
    {
        &self.banned_words
    }

    pub async fn create_basic_role<'a, D>(&mut self, database: D, basic_role: String) -> Result<(), sqlx::Error> 
    where
        D: Executor<'a, Database = Sqlite>
    {
        sqlx::query!("update guild set basic_role_id = (?) where guild_id = ?", basic_role, self.guild_id)
            .execute(database)
            .await?;
        self.basic_role_id = Some(basic_role);
        Ok(())
    }

    pub async fn delete_basic_role<'a, D>(&mut self, database: D) -> Result<(), sqlx::Error> 
    where
        D: Executor<'a, Database = Sqlite>
    {
        sqlx::query!("update guild set basic_role_id = NULL where guild_id = ?", self.guild_id)
            .execute(database)
            .await?;
        self.basic_role_id = None;
        Ok(())
    }
}

pub fn strip_mention(mention :&str) -> &str {
    if let Some(stripped_prefix) = mention.strip_prefix("<@&") {
        if let Some(stripped_suffix) = stripped_prefix.strip_suffix(">") {
            return stripped_suffix
        }
    };
    mention
}

#[cfg(test)]
mod tests {
    use crate::guild::strip_mention;
    #[test]
    fn strip_mention_test() {
        let a = strip_mention("<@123>");
        assert_eq!(a, "123");
    }
    #[test]
    fn strip_mention_test_2() {
        let a = strip_mention("123");
        assert_eq!(a, "123");
    }
}
