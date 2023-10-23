use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use rand::prelude::*;
use rand::seq::index::sample;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Word {
    pub word: String,
    pub url: Url,
}

#[derive(Default)]
pub struct Trainer<'words> {
    words: Cow<'words, [Word]>,
    selected: Option<usize>,
    rng: ThreadRng,
}

impl Clone for Trainer<'_> {
    fn clone(&self) -> Self {
        Self {
            words: self.words.clone(),
            selected: self.selected.clone(),
            ..Default::default()
        }
    }
}

impl Debug for Trainer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trainer")
            .field("words", &self.words)
            .field("selected", &self.selected.and_then(|selected| self.words.get(selected)))
            .finish()
    }
}

impl PartialEq for Trainer<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.words == other.words && self.selected == other.selected
    }
}
impl Eq for Trainer<'_> {}

impl<'words> Trainer<'words> {
    pub fn new(words: impl Into<Cow<'words, [Word]>>) -> Self {
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

    pub fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
    }

    pub fn random(&mut self) {
        if self.words.is_empty() {
            self.selected = None;
            return;
        }

        let idx = sample(&mut self.rng, self.words.len(), 1).index(0);
        self.select(idx);
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

        assert_eq!(trainer.selected, None, "no default selected word");
    }

    #[test]
    fn select_from_empty_wordlist() {
        let mut trainer = Trainer::default();
        trainer.select(0);

        assert_eq!(trainer.selected(), None, "cannot select from empty word list");
    }

    #[test]
    fn select_invalid_index() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(2);

        assert_eq!(trainer.selected(), None, "cannot select invalid word index");
    }

    #[test]
    fn select_valid_index() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert_eq!(trainer.selected(), Some(&WORDS[0]), "valid word is selected");
    }

    #[test]
    fn select_random_index_from_empty_wordlist() {
        let mut trainer = Trainer::default();
        trainer.random();

        assert_eq!(trainer.selected(), None, "random word can be selected from empty wordlist");
    }

    #[test]
    fn select_random_index_from_words() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.random();

        assert_ne!(trainer.selected(), None, "random word can be selected");
    }

    #[test]
    fn correct_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert!(trainer.guess(&WORDS[0].word), "the guess is correct");
        assert_eq!(trainer.selected(), None, "reset selected word after correct guess");
    }

    #[test]
    fn incorrect_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        trainer.select(0);

        assert!(!trainer.guess(&WORDS[1].word), "the guess is incorrect");
        assert_eq!(trainer.selected(), Some(&WORDS[0]), "the previous word stays selected");
    }
}