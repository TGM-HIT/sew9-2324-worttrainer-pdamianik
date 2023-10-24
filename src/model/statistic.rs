use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Statistic {
    correct: usize,
    incorrect: usize,
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
    }

    pub fn increment_incorrect(&mut self) {
        self.incorrect += 1;
}

pub struct StatisticReceiver {
    pub statistic: Arc<Statistic>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    async fn no_guess() {
        let statistic = Statistic::default();

        assert_eq!(statistic.correct(), 0, "expect no correct guess to get counted");
        assert_eq!(statistic.incorrect(), 0, "expect no incorrect guess to get counted");
        assert_eq!(statistic.total(), 0, "expect no guess to get counted");
    }

    #[test]
    async fn correct_guess() {
        let mut statistic = Statistic::default();

        statistic.increment_correct();

        assert_eq!(statistic.correct(), 1, "expect one correct guess to get counted");
        assert_eq!(statistic.incorrect(), 0, "expect no incorrect guess to get counted");
        assert_eq!(statistic.total(), 1, "expect one guess to get counted");
    }

    #[test]
    async fn incorrect_guess() {
        let mut statistic = Statistic::default();

        statistic.increment_incorrect();

        assert_eq!(statistic.correct(), 0, "expect no correct guess to get counted");
        assert_eq!(statistic.incorrect(), 1, "expect one incorrect guess to get counted");
        assert_eq!(statistic.total(), 1, "expect one guess to get counted");
    }
}