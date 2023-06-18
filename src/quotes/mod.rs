use rand::{self, Rng};
use tokio::sync::RwLock;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
#[derive(Debug)]
pub struct Quotes {
    mean_quotes: Vec<String>,
    neutral_quotes: Vec<String>,
    pleasant_quotes: Vec<String>,
    mean_quotes_len: usize,
    neutral_quotes_len: usize,
    pleasant_quotes_len: usize,
}

impl Quotes {
    pub fn new(mean_quotes: Vec<String>, neutral_quotes: Vec<String>, pleasant_quotes: Vec<String>) -> Quotes {
        Quotes { 
            mean_quotes_len: mean_quotes.len(),
            neutral_quotes_len: neutral_quotes.len(),
            pleasant_quotes_len: pleasant_quotes.len(),
            mean_quotes, 
            neutral_quotes, 
            pleasant_quotes,
        }
    }
    pub fn random_pleasant_quote(&self) -> &str {
        let random_quote = rand::thread_rng().gen_range(0..self.pleasant_quotes_len);
        &self.pleasant_quotes[random_quote]
    }
    pub fn random_neutral_quote(&self) -> &str {
        let random_quote = rand::thread_rng().gen_range(0..self.neutral_quotes_len);
        &self.neutral_quotes[random_quote]
    }
    pub fn random_mean_quote(&self) -> &str {
        let random_quote = rand::thread_rng().gen_range(0..self.mean_quotes_len);
        &self.mean_quotes[random_quote]
    }
}

impl TypeMapKey for Quotes {
    type Value = Arc<RwLock<Quotes>>;
}

#[cfg(test)]
mod tests {
    use crate::quotes::Quotes;
    #[test]
    fn random_pleasant_quote() {
        let mean_quotes = vec!["You stink".to_string()];
        let pleasant_quotes = vec!["I love you".to_string()];
        let neutral_quotes = vec!["Just chillin'".to_string()];
        let quotes = Quotes::new(mean_quotes, neutral_quotes, pleasant_quotes);
        assert_eq!(quotes.random_pleasant_quote(), "I love you");
    }
    #[test]
    fn random_mean_quote() {
        let mean_quotes = vec!["You stink".to_string()];
        let pleasant_quotes = vec!["I love you".to_string()];
        let neutral_quotes = vec!["Just chillin'".to_string()];
        let quotes = Quotes::new(mean_quotes, neutral_quotes, pleasant_quotes);
        assert_eq!(quotes.random_mean_quote(), "You stink");
    }
    #[test]
    fn random_neutral_quote() {
        let mean_quotes = vec!["You stink".to_string()];
        let pleasant_quotes = vec!["I love you".to_string()];
        let neutral_quotes = vec!["Just chillin'".to_string()];
        let quotes = Quotes::new(mean_quotes, neutral_quotes, pleasant_quotes);
        assert_eq!(quotes.random_neutral_quote(), "Just chillin'");
    }
}

