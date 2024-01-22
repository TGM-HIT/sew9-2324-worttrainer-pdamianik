mod statistic;

use std::fmt::Debug;
use rand::prelude::*;
use rand::seq::index::sample;
use url::Url;
use serde::{Deserialize, Serialize};
use crate::model::statistic::Statistic;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub url: Url,
    pub credits: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trainer {
    words: Vec<Word>,
    selected: Option<usize>,
    statistic: Statistic,
    #[serde(skip)]
    generator: ThreadRng,
}

impl PartialEq for Trainer {
    fn eq(&self, other: &Self) -> bool {
        self.words == other.words &&
            self.selected == other.selected &&
            self.statistic == other.statistic
    }
}

impl Trainer {
    pub fn new(words: impl Into<Vec<Word>>) -> Self {
        Self {
            words: words.into(),
            ..Default::default()
        }
    }

    pub fn selected(&self) -> Option<&Word> {
        match self.selected {
            Some(idx) if idx < self.words.len() => Some(&self.words[idx]),
            _ => None,
        }
    }

    pub fn select(&mut self, idx: usize) -> Option<&Word> {
        self.selected = Some(idx);
        self.selected()
    }

    pub fn random(&mut self) -> Option<&Word> {
        if self.words.is_empty() {
            self.selected = None;
        } else {
            let idx = sample(&mut self.generator, self.words.len(), 1).index(0);
            self.select(idx);
        }

        self.selected()
    }

    pub fn guess(&mut self, guess: &str) -> bool {
        let correct = self.selected().is_some_and(|word| word.word == guess);
        if correct {
            self.statistic.increment_correct();
            self.selected = None;
        } else {
            self.statistic.increment_incorrect();
        }
        correct
    }

    pub fn statistic(&self) -> &Statistic {
        &self.statistic
    }

    pub fn reset_statistic(&mut self) {
        self.statistic = Statistic::default();
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use lazy_static::lazy_static;
    use super::*;

    lazy_static! {
        static ref WORDS: [Word; 2] = [
            Word {
                word: "Apple".to_owned(),
                url: Url::from_str("https://apple.com/").expect("valid url"),
                credits: "Apple".to_owned(),
            },
            Word {
                word: "Raspberry".to_owned(),
                url: Url::from_str("https://raspberry.org/").expect("valid url"),
                credits: "Raspberry".to_owned(),
            }
        ];
    }

    #[test]
    fn default_selected() {
        let trainer = Trainer::default();

        assert_eq!(trainer.selected(), None, "expect an empty selection");
    }

    #[test]
    fn select_from_empty_wordlist() {
        let mut trainer = Trainer::default();
        trainer.select(0);

        assert_eq!(trainer.selected(), None, "expect an empty selection");
    }

    #[test]
    fn select_invalid_index() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(2);

        assert_eq!(trainer.selected(), None, "expect an empty selection");
    }

    #[test]
    fn select_valid_index() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert_eq!(trainer.selected(), Some(&WORDS[0]), "expect the first word to be selected");
    }

    #[test]
    fn select_random_index_from_empty_wordlist() {
        let mut trainer = Trainer::default();
        trainer.random();

        assert_eq!(trainer.selected(), None, "expect an empty selection");
    }

    #[test]
    fn select_random_index_from_words() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.random();

        assert!(trainer.selected().is_some_and(|word| trainer.words.contains(&word)), "expect a random word to be selected");
    }

    #[test]
    fn correct_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert!(trainer.guess(&WORDS[0].word), "expect the guess to be correct");
        assert_eq!(trainer.selected(), None, "expect the selection to be cleared");
        assert_eq!(trainer.statistic().correct(), 1, "expect one correct guess to be counted");
        assert_eq!(trainer.statistic().incorrect(), 0, "expect no incorrect guess to be counted");
    }

    #[test]
    fn incorrect_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert!(!trainer.guess(&WORDS[1].word), "expect the guess to be incorrect");
        assert_eq!(trainer.selected(), Some(&WORDS[0]), "expect the first word to stay selected");
        assert_eq!(trainer.statistic().correct(), 0, "expect no correct guess to be counted");
        assert_eq!(trainer.statistic().incorrect(), 1, "expect one incorrect guess to be counted");
    }
}