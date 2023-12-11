use std::fmt::Debug;
use crate::observer::{Observable, Observers};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatisticEvent {
    Correct {
        count: usize,
        total: usize
    },
    Incorrect {
        count: usize,
        total: usize
    },
}

#[derive(Default)]
pub struct Statistic {
    correct: usize,
    incorrect: usize,
    observers: Observers<StatisticEvent>,
}

impl Debug for Statistic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Statistic")
            .field("correct", &self.correct)
            .field("incorrect", &self.incorrect)
            .finish()
    }
}

impl Statistic {
    pub fn total(&self) -> usize {
        self.correct + self.incorrect
    }

    pub fn correct(&self) -> usize {
        self.correct
    }

    pub fn incorrect(&self) -> usize {
        self.incorrect
    }

    pub fn increment_correct(&mut self) {
        self.correct += 1;
        self.observers.notify(&StatisticEvent::Correct { count: self.correct, total: self.total() });
    }

    pub fn increment_incorrect(&mut self) {
        self.incorrect += 1;
        self.observers.notify(&StatisticEvent::Incorrect { count: self.incorrect, total: self.total() });
    }
}

impl Observable<StatisticEvent> for Statistic {
    fn observe(&mut self, observer: impl FnMut(&StatisticEvent) + 'static) {
        self.observers.observe(observer);
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::rc::Rc;
    use super::*;

    #[test]
    fn no_event() {
        let count = Rc::new(Cell::new(0));
        let mut statistic = Statistic::default();
        {
            let count = count.clone();
            statistic.observe(move |_event| {
                count.set(count.get() + 1);
                assert!(false, "expect no event to get fired");
            });
        }

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 0, "no guess got received");
        assert_eq!(count.get(), 0, "expect no event to get fired");
    }

    #[test]
    fn correct_guess() {
        let count = Rc::new(Cell::new(0));
        let mut statistic = Statistic::default();
        {
            let count = count.clone();
            statistic.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &StatisticEvent::Correct { count: 1, total: 1 }, "expect a single correct guess"),
                    _ => assert!(false, "expect at most one to get fired"),
                }
            });
        }

        statistic.increment_correct();

        assert_eq!(statistic.correct(), 1, "expect one correct guess to get counted");
        assert_eq!(statistic.incorrect(), 0, "expect no incorrect guess to get counted");
        assert_eq!(statistic.total(), 1, "expect one guess to get counted");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }

    #[test]
    fn incorrect_guess() {
        let count = Rc::new(Cell::new(0));
        let mut statistic = Statistic::default();
        {
            let count = count.clone();
            statistic.observe(move |event| {
                count.set(count.get() + 1);
                match count.get() {
                    1 => assert_eq!(event, &StatisticEvent::Incorrect { count: 1, total: 1 }, "expect a single incorrect guess"),
                    _ => assert!(false, "expect at most one to get fired"),
                }
            });
        }

        statistic.increment_incorrect();

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 1, "an incorrect guess got counted");
        assert_eq!(statistic.total(), 1, "a guess got received");
        assert_eq!(count.get(), 1, "expect one event to get fired");
    }
}