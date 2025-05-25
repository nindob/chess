use crate::app::AppResult;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

// terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    // terminal tick.
    Tick,
    // key press.
    Key(KeyEvent),
    // mouse click/scroll.
    Mouse(MouseEvent),
    // terminal resize.
    Resize(u16, u16),
}

// terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    // event sender channel.
    sender: mpsc::Sender<Event>,
    // event receiver channel.
    receiver: mpsc::Receiver<Event>,
    // event handler thread.
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    // constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    // receive the next event from the handler thread.
    //
    // this function will always block the current thread if
    // there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}