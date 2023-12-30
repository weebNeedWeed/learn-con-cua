pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: u32,
    max: u32,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: u32) -> Self {
        Self {
            value: 0,
            messenger,
            max,
        }
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;

        let percentage_of_max = (value as f64) / (self.max as f64);
        if percentage_of_max >= 1.0 {
            self.messenger.send("U are over your quotas!");
            return;
        }

        if percentage_of_max >= 0.9 {
            self.messenger
                .send("U have used up over 90% of your quotas");
            return;
        }

        if percentage_of_max >= 0.75 {
            self.messenger
                .send("U have used up over 75% of your quotas");
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        pub fn new() -> Self {
            Self {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn should_send_over_75_msg() {
        let msger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&msger, 100);

        tracker.set_value(78);

        assert_eq!(msger.messages.borrow().len(), 1);
    }

    #[test]
    fn should_send_no_msg() {
        let msger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&msger, 100);

        tracker.set_value(50);

        assert_eq!(msger.messages.borrow().len(), 0);
    }
}
