use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Error,
}

pub type Sender = UnboundedSender<Event>;
pub type Receiver = UnboundedReceiver<Event>;

#[derive(Debug)]
pub struct EventHandler {
    pub sender: Sender,
    receiver: Receiver,
    _handle: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let sender = tx.clone();

        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                // initiate the tick
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  _ = tick_delay => {
                    sender.send(Event::Tick).unwrap();
                  }

                  Some(Ok(evt)) = crossterm_event => {
                    match evt {
                      CrosstermEvent::Key(key) => {
                        if key.kind == event::KeyEventKind::Press {
                          sender.send(Event::Key(key)).unwrap();
                        }
                      },
                      CrosstermEvent::Mouse(mouse) => {
                        sender.send(Event::Mouse(mouse)).unwrap();
                      },
                      CrosstermEvent::Resize(_, _) => {},
                      CrosstermEvent::FocusLost => {},
                      CrosstermEvent::FocusGained => {},
                      CrosstermEvent::Paste(_) => {},
                    }
                  }
                };
            }
        });

        Self {
            sender: tx,
            receiver: rx,
            _handle: handler,
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or_else(|| {
            anyhow::anyhow!("Something went wrong when receiving events from events handler")
        })
    }
}
