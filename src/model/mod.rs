mod statistic;

use std::fmt::Debug;
use rand::prelude::*;
use rand::seq::index::sample;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Word {
    pub word: String,
    pub url: Url,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Trainer {
    words: Vec<Word>,
    selected: Option<usize>,
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
            let idx = sample(&mut thread_rng(), self.words.len(), 1).index(0);
            self.select(idx);
        }

        self.selected()
    }

    pub fn guess(&mut self, guess: &str) -> bool {
        let correct = self.selected().is_some_and(|word| word.word == guess);
        if correct {
            self.selected = None;
        }
        correct
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use lazy_static::lazy_static;
    use super::*;

    lazy_static!{
        static ref WORDS: [Word; 2] = [
            Word {
                word: "Apple".to_owned(),
                url: Url::from_str("https://apple.com/").expect("valid url"),
            },
            Word {
                word: "Raspberry".to_owned(),
                url: Url::from_str("https://raspberry.org/").expect("valid url"),
            }
        ];
    }

    #[test]
    fn default_selected() {
        let trainer = Trainer::default();

        assert_eq!(trainer.selected, None, "expect an empty selection");
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
    }

    #[test]
    fn incorrect_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert!(!trainer.guess(&WORDS[1].word), "expect the guess to be incorrect");
        assert_eq!(trainer.selected(), Some(&WORDS[0]), "expect the first word to stay selected");
    }
}