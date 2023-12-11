pub trait Observable<Event> {
    fn observe(&mut self, observer: impl FnMut(&Event) + 'static);
}

pub struct Observers<Event> {
    observers: Vec<Box<dyn FnMut(&Event) + 'static>>,
}

impl<Event> Default for Observers<Event> {
    fn default() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<Event> Observers<Event> {
    pub fn notify(&mut self, event: &Event) {
        for observer in &mut self.observers {
            observer(event);
        }
    }
}

impl<Event> Observable<Event> for Observers<Event> {
    fn observe(&mut self, observer: impl FnMut(&Event) + 'static) {
        self.observers.push(Box::new(observer));
    }
}