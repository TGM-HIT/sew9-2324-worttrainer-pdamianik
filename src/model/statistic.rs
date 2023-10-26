use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use futures::StreamExt;
use pharos::{Events, Observable, Observe, ObserveConfig, PharErr, SharedPharos};
use tokio::spawn;
use tokio::task::JoinHandle;
use crate::model::TrainerEvent;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatisticEvent {
    Correct(usize),
    Incorrect(usize),
}

#[derive(Debug, Default)]
pub struct Statistic {
    correct: AtomicUsize,
    incorrect: AtomicUsize,
    events: SharedPharos<StatisticEvent>,
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

    pub async fn observe(&self, options: ObserveConfig<StatisticEvent>) -> Result<Events<StatisticEvent>, PharErr> {
        self.events.observe_shared(options).await
    }
}

impl Observable<StatisticEvent> for Statistic {
    type Error = PharErr;

    fn observe(&mut self, options: ObserveConfig<StatisticEvent>) -> Observe<'_, StatisticEvent, Self::Error> {
        self.events.observe(options)
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
                            let previous = self.statistic.correct.fetch_add(1, Ordering::Relaxed);
                            self.statistic.events.notify(StatisticEvent::Correct(previous + 1)).await.expect("notify observers");
                        } else {
                            let previous = self.statistic.incorrect.fetch_add(1, Ordering::Relaxed);
                            self.statistic.events.notify(StatisticEvent::Incorrect(previous + 1)).await.expect("notify observers");
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
        let statistic_events = statistic.observe(Channel::Unbounded.into()).await.expect("receive");
        let handle = receiver.receive();

        yield_now().await;

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 0, "no guess got received");
        handle.abort();
        drop(statistic);
        assert_eq!(statistic_events.count().await, 0, "no events left")
    }

    #[tokio::test]
    async fn correct_guess() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        let mut statistic_events = statistic.observe(Channel::Unbounded.into()).await.expect("receive");
        let handle = receiver.receive();

        sender.send(TrainerEvent::Guess { guess: "".to_owned(), correct: true }).await.expect("send correct guess");
        yield_now().await;
        assert_eq!(statistic_events.next().await, Some(StatisticEvent::Correct(1)), "correct got updated to 1");

        assert_eq!(statistic.correct(), 1, "a correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 1, "a guess got received");
        handle.abort();
        drop(statistic);
        assert_eq!(statistic_events.count().await, 0, "no events left")
    }

    #[tokio::test]
    async fn incorrect_guess() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        let mut statistic_events = statistic.observe(Channel::Unbounded.into()).await.expect("receive");
        let handle = receiver.receive();

        sender.send(TrainerEvent::Guess { guess: "".to_owned(), correct: false }).await.expect("send incorrect guess");
        yield_now().await;
        assert_eq!(statistic_events.next().await, Some(StatisticEvent::Incorrect(1)), "correct got updated to 1");

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 1, "an incorrect guess got counted");
        assert_eq!(statistic.total(), 1, "a guess got received");
        handle.abort();
        drop(statistic);
        assert_eq!(statistic_events.count().await, 0, "no events left")
    }

    #[tokio::test]
    async fn other_event() {
        let mut sender = Pharos::default();
        let receiver: StatisticReceiver = sender.observe(Channel::Unbounded.into()).await.expect("receive").into();
        let statistic = receiver.statistic.clone();
        let statistic_events = statistic.observe(Channel::Unbounded.into()).await.expect("receive");
        let handle = receiver.receive();

        sender.send(TrainerEvent::Select(None)).await.expect("send other event");
        yield_now().await;

        assert_eq!(statistic.correct(), 0, "no correct guess got counted");
        assert_eq!(statistic.incorrect(), 0, "no incorrect guess got counted");
        assert_eq!(statistic.total(), 0, "no guess got received");
        handle.abort();
        drop(statistic);
        assert_eq!(statistic_events.count().await, 0, "no events left")
    }
}