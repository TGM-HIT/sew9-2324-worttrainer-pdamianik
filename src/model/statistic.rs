use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use futures::StreamExt;
use pharos::Events;
use tokio::spawn;
use tokio::task::JoinHandle;
use crate::model::TrainerEvent;

#[derive(Debug, Default)]
pub struct Statistic {
    correct: AtomicUsize,
    incorrect: AtomicUsize,
}

impl Statistic {
    pub fn total(&self) -> usize {
        self.correct.load(Ordering::Relaxed) + self.incorrect.load(Ordering::Relaxed)
    }

    pub fn correct(&self) -> usize {
        self.correct.load(Ordering::Relaxed)
    }

    pub fn incorrect(&self) -> usize {
        self.incorrect.load(Ordering::Relaxed)
    }
}

pub struct StatisticReceiver {
    pub statistic: Arc<Statistic>,
    events: Events<TrainerEvent>,
}

impl From<Events<TrainerEvent>> for StatisticReceiver {
    fn from(events: Events<TrainerEvent>) -> Self {
        Self {
            statistic: Default::default(),
            events,
        }
    }
}

impl StatisticReceiver {
    pub fn receive(mut self) -> JoinHandle<()> {
        spawn(async move {
            loop {
                let event = self.events.next().await;

                match event {
                    Some(TrainerEvent::Guess { correct, .. }) => {
                        if correct {
                            self.statistic.correct.fetch_add(1, Ordering::Relaxed);
                        } else {
                            self.statistic.incorrect.fetch_add(1, Ordering::Relaxed);
                        }
                    },
                    None => break,
                    _ => (),
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use futures::SinkExt;
    use pharos::{Channel, Observable, Pharos};
    use tokio::task::yield_now;
    use super::*;

    #[tokio::test]
    async fn no_event() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        receiver.receive();

        yield_now().await;

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 0, "no guess got received");
    }

    #[tokio::test]
    async fn correct_guess() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        receiver.receive();

        sender.send(TrainerEvent::Guess { guess: "".to_owned(), correct: true }).await.expect("send correct guess");
        yield_now().await;

        assert_eq!(statistic.correct(), 1, "a correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 1, "a guess got received");
    }

    #[tokio::test]
    async fn incorrect_guess() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        receiver.receive();

        sender.send(TrainerEvent::Guess { guess: "".to_owned(), correct: false }).await.expect("send incorrect guess");
        yield_now().await;

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 1, "an incorrect guess got counted");
        assert_eq!(statistic.total(), 1, "a guess got received");
    }

    #[tokio::test]
    async fn other_event() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        receiver.receive();

        sender.send(TrainerEvent::Select(None)).await.expect("send other event");
        yield_now().await;

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 0, "no guess got received");
    }
}