use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
}

type Sender = UnboundedSender<Event>;
type Receiver = UnboundedReceiver<Event>;

#[derive(Debug)]
pub struct EventHandler {
    sender: Sender,
    receiver: Receiver,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let sender = tx.clone();

        tokio::spawn(async move {
            loop {
                if event::poll(tick_rate).expect("Something went wrong when polling for events") {
                    match event::read().expect("Something went wrong when reading events") {
                        CrosstermEvent::Key(e) => {
                            if e.kind == event::KeyEventKind::Press {
                                sender.send(Event::Key(e))
                            } else {
                                Ok(())
                            }
                        }
                        _ => Ok(()),
                    }
                    .expect("Something went wrong when sending events")
                }
            }
        });

        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or_else(|| {
            anyhow::anyhow!("Something went wrong when receiving events from events handler")
        })
    }
}
