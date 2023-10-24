use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use pharos::*;
use rand::prelude::*;
use rand::seq::index::sample;
use url::Url;
use futures::SinkExt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Word {
    pub word: String,
    pub url: Url,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TrainerEvent {
    Select(Option<Word>),
    Guess{
        guess: String,
        correct: bool,
    },
}

#[derive(Default)]
pub struct Trainer<'words> {
    words: Cow<'words, [Word]>,
    selected: Option<usize>,
    rng: ThreadRng,
    events: Pharos<TrainerEvent>,
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

    pub async fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
        self.events.send(TrainerEvent::Select(self.selected().cloned())).await.expect("notify observers");
    }

    pub async fn random(&mut self) {
        if self.words.is_empty() {
            self.selected = None;
            self.events.send(TrainerEvent::Select(self.selected().cloned())).await.expect("notify observers");
            return;
        }

        let idx = sample(&mut self.rng, self.words.len(), 1).index(0);
        self.select(idx).await;
    }

    pub async fn guess(&mut self, guess: &str) -> bool {
        let correct = self.selected().is_some_and(|word| word.word == guess);
        self.events.send(TrainerEvent::Guess { guess: guess.to_string(), correct }).await.expect("notify observers");
        if correct {
            self.selected = None;
            self.events.send(TrainerEvent::Select(self.selected().cloned())).await.expect("notify observers");
        }
        correct
    }
}

impl Observable<TrainerEvent> for Trainer<'_> {
    type Error = PharErr;

    fn observe(&mut self, options: ObserveConfig<TrainerEvent>) -> Observe<'_, TrainerEvent, Self::Error> {
        self.events.observe(options)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use futures::StreamExt;
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

    #[tokio::test]
    async fn default_selected() {
        let mut trainer = Trainer::default();
        let events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        assert_eq!(trainer.selected(), None, "no default selected word");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no events");
    }

    #[tokio::test]
    async fn select_from_empty_wordlist() {
        let mut trainer = Trainer::default();
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.select(0).await;

        assert_eq!(trainer.selected(), None, "cannot select from empty word list");
        assert_eq!(events.next().await, Some(TrainerEvent::Select(None)), "None got selected");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }

    #[tokio::test]
    async fn select_invalid_index() {
        let mut trainer = Trainer::new(&WORDS[..]);
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.select(2).await;

        assert_eq!(trainer.selected(), None, "cannot select invalid word index");
        assert_eq!(events.next().await, Some(TrainerEvent::Select(None)), "None got selected");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }

    #[tokio::test]
    async fn select_valid_index() {
        let mut trainer = Trainer::new(&WORDS[..]);
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.select(0).await;

        assert_eq!(trainer.selected(), Some(&WORDS[0]), "valid word is selected");
        assert_eq!(events.next().await, Some(TrainerEvent::Select(Some(WORDS[0].clone()))), "valid word got selected");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }

    #[tokio::test]
    async fn select_random_index_from_empty_wordlist() {
        let mut trainer = Trainer::default();
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.random().await;

        assert_eq!(trainer.selected(), None, "random word can be selected from empty wordlist");
        assert_eq!(events.next().await, Some(TrainerEvent::Select(None)), "None got selected");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }

    #[tokio::test]
    async fn select_random_index_from_words() {
        let mut trainer = Trainer::new(&WORDS[..]);
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.random().await;

        let valid_word = |word: Option<&Word>| {
            word.is_some_and(|word| WORDS.contains(word))
        };
        assert!(valid_word(trainer.selected()), "valid random word is selected");
        assert!(if let Some(TrainerEvent::Select(word)) = events.next().await {
            valid_word(word.as_ref())
        } else {
            false
        }, "valid random word got selected");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }

    #[tokio::test]
    async fn correct_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.select(0).await;
        let guess = &WORDS[0];
        assert_eq!(events.next().await, Some(TrainerEvent::Select(Some(guess.to_owned()))), "a word got selected");

        assert!(trainer.guess(&guess.word).await, "the guess is correct");
        assert_eq!(trainer.selected(), None, "reset selected word after correct guess");
        assert_eq!(events.next().await, Some(TrainerEvent::Guess{guess: guess.word.to_owned(), correct: true}), "a word got guessed correctly");
        assert_eq!(events.next().await, Some(TrainerEvent::Select(None)), "None got selected");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }

    #[tokio::test]
    async fn incorrect_guess() {
        let mut trainer = Trainer::new(&WORDS[..]);
        let mut events = trainer.observe(Channel::Unbounded.into()).await.expect("observe");

        trainer.select(0).await;
        let selected = &WORDS[0];
        let guess = &WORDS[1];
        assert_eq!(events.next().await, Some(TrainerEvent::Select(Some(selected.to_owned()))), "a word got selected");

        assert!(!trainer.guess(&guess.word).await, "the guess is incorrect");
        assert_eq!(trainer.selected(), Some(selected), "the previous word stays selected");
        assert_eq!(events.next().await, Some(TrainerEvent::Guess{guess: guess.word.to_owned(), correct: false}), "a word got guessed incorrectly");

        drop(trainer);
        assert_eq!(events.count().await, 0, "no remaining events");
    }
}