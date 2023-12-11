use std::fmt::{Debug, Formatter};
use rand::prelude::*;
use rand::seq::index::sample;
use url::Url;
use crate::observer::{Observable, Observers};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TrainerEvent {
    Selected(Option<Word>),
    Guessed {
        guess: String,
        correct: bool,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Word {
    pub word: String,
    pub url: Url,
}

#[derive(Default)]
pub struct Trainer {
    words: Vec<Word>,
    selected: Option<usize>,
    observers: Observers<TrainerEvent>,
}

impl Clone for Trainer {
    fn clone(&self) -> Self {
        Self {
            words: self.words.clone(),
            selected: self.selected.clone(),
            ..Default::default()
        }
    }
}

impl Debug for Trainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trainer")
            .field("words", &self.words)
            .field("selected", &self.selected.and_then(|selected| self.words.get(selected)))
            .finish()
    }
}

impl PartialEq for Trainer {
    fn eq(&self, other: &Self) -> bool {
        self.words == other.words && self.selected == other.selected
    }
}
impl Eq for Trainer {}

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

    pub fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
        self.observers.notify(&TrainerEvent::Selected(self.selected().cloned()));
    }

    pub fn random(&mut self) {
        if self.words.is_empty() {
            self.selected = None;
            self.observers.notify(&TrainerEvent::Selected(None));
            return;
        }

        let idx = sample(&mut thread_rng(), self.words.len(), 1).index(0);
        self.select(idx);
    }

    pub fn guess(&mut self, guess: &str) -> bool {
        let correct = self.selected().is_some_and(|word| word.word == guess);
        self.observers.notify(&TrainerEvent::Guessed {
            guess: guess.to_owned(),
            correct,
        });
        if correct {
            self.selected = None;
            self.observers.notify(&TrainerEvent::Selected(None));
        }
        correct
    }
}

impl Observable<TrainerEvent> for Trainer {
    fn observe(&mut self, observer: impl FnMut(&TrainerEvent) + 'static) {
        self.observers.observe(observer);
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::rc::Rc;
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
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::default();
        {
            let count = count.clone();
            trainer.observe(move |_event| {
                count.set(count.get() + 1);
                assert!(false, "expect no event to get fired");
            });
        }

        assert_eq!(trainer.selected, None, "expect an empty selection");
        assert_eq!(count.get(), 0, "expect no event to get fired");
    }

    #[test]
    fn select_from_empty_wordlist() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::default();
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &TrainerEvent::Selected(None), "expect an empty selection"),
                    _ => assert!(false, "expect at most one event to get fired"),
                }
            });
        }

        trainer.select(0);

        assert_eq!(trainer.selected(), None, "expect an empty selection");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }

    #[test]
    fn select_invalid_index() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::new(&WORDS[..]);
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &TrainerEvent::Selected(None), "expect an empty selection"),
                    _ => assert!(false, "expect at most one event to get fired"),
                }
            });
        }

        trainer.select(2);

        assert_eq!(trainer.selected(), None, "expect an empty selection");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }

    #[test]
    fn select_valid_index() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::new(&WORDS[..]);
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &TrainerEvent::Selected(Some(WORDS[0].clone())), "expect the first word to be selected"),
                    _ => assert!(false, "expect at most one event to get fired"),
                }
            });
        }

        trainer.select(0);

        assert_eq!(trainer.selected(), Some(&WORDS[0]), "expect the first word to be selected");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }

    #[test]
    fn select_random_index_from_empty_wordlist() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::default();
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &TrainerEvent::Selected(None), "expect an empty selection"),
                    _ => assert!(false, "expect at most one event to get fired"),
                }
            });
        }
        trainer.random();

        assert_eq!(trainer.selected(), None, "expect an empty selection");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }

    #[test]
    fn select_random_index_from_words() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::new(&WORDS[..]);
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => match event {
                        &TrainerEvent::Selected(Some(ref word)) => assert!(WORDS.contains(word), "expect a word from the wordlist to be selected"),
                        _ => assert!(false, "expect a word to be selected"),
                    },
                    _ => assert!(false, "expect at most one event to get fired"),
                }
            });
        }

        trainer.random();

        assert!(trainer.selected().is_some_and(|word| trainer.words.contains(&word)), "expect a random word to be selected");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }

    #[test]
    fn correct_guess() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::new(&WORDS[..]);
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &TrainerEvent::Selected(Some(WORDS[0].clone())), "expect the first word to be selected"),
                    2 => assert_eq!(event, &TrainerEvent::Guessed {
                        guess: WORDS[0].word.clone(),
                        correct: true,
                    }, "expect the guess to be correct"),
                    3 => assert_eq!(event, &TrainerEvent::Selected(None), "expect the selection to be reset"),
                    _ => assert!(false, "expect at most three events to get fired"),
                }
            });
        }

        trainer.select(0);

        assert!(trainer.guess(&WORDS[0].word), "expect the guess to be correct");
        assert_eq!(trainer.selected(), None, "expect the selection to be cleared");
        assert_eq!(count.get(), 3, "expect three events to get fired");
    }

    #[test]
    fn incorrect_guess() {
        let count = Rc::new(Cell::new(0));
        let mut trainer = Trainer::new(&WORDS[..]);
        {
            let count = count.clone();
            trainer.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &TrainerEvent::Selected(Some(WORDS[0].clone())), "expect the first word to be selected"),
                    2 => assert_eq!(event, &TrainerEvent::Guessed {
                        guess: WORDS[1].word.clone(),
                        correct: false,
                    }, "expect the guess to be incorrect"),
                    _ => assert!(false, "expect at most two events to get fired"),
                }
            });
        }

        trainer.select(0);

        assert!(!trainer.guess(&WORDS[1].word), "expect the guess to be incorrect");
        assert_eq!(trainer.selected(), Some(&WORDS[0]), "expect the first word to stay selected");
        assert_eq!(count.get(), 2, "expect two events to get fired");
    }
}