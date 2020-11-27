#[cfg(feature = "std")]
pub mod llmp;

use alloc::string::String;
use core::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
pub mod llmp_translated; // TODO: Abstract away.
#[cfg(feature = "std")]
pub mod shmem_translated;

#[cfg(feature = "std")]
pub use crate::events::llmp::LLMPEventManager;

#[cfg(feature = "std")]
use std::io::Write;

use crate::corpus::{Corpus, Testcase};
use crate::engines::State;
use crate::executors::Executor;
use crate::inputs::Input;
use crate::utils::Rand;
use crate::AflError;
/// Indicate if an event worked or not
enum BrokerEventResult {
    /// The broker haneled this. No need to pass it on.
    Handled,
    /// Pass this message along to the clients.
    Forward,
}

/*

/// A custom event, in case a user wants to extend the features (at compile time)
pub trait CustomEvent<S, C, E, I, R>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    E: Executor<I>,
    I: Input,
    R: Rand,
{
    /// Returns the name of this event
    fn name(&self) -> &str;
    /// This method will be called in the broker
    fn handle_in_broker(&self, broker: &dyn EventManager<S, C, E, I, R, Self>, state: &mut S, corpus: &mut C) -> Result<BrokerEventResult, AflError>;
    /// This method will be called in the clients after handle_in_broker (unless BrokerEventResult::Handled) was returned in handle_in_broker
    fn handle_in_client(&self, client: &dyn EventManager<S, C, E, I, R, Self>, state: &mut S, corpus: &mut C) -> Result<(), AflError>;
}

struct UnusedCustomEvent {}
impl<S, C, E, I, R> CustomEvent<S, C, E, I, R> for UnusedCustomEvent<S, C, E, I, R>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    E: Executor<I>,
    I: Input,
    R: Rand,
{
    fn name(&self) -> &str {"No custom events"}
    fn handle_in_broker(&self, broker: &dyn EventManager<S, C, E, I, R, Self>, state: &mut S, corpus: &mut C) {Ok(BrokerEventResult::Handled)}
    fn handle_in_client(&self, client: &dyn EventManager<S, C, E, I, R, Self>, state: &mut S, corpus: &mut C) {Ok(())}
}
*/

/// Events sent around in the library
#[derive(Serialize, Deserialize)]
pub enum Event<S, C, E, I, R>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    E: Executor<I>,
    I: Input,
    R: Rand,
    // CE: CustomEvent<S, C, E, I, R>,
{
    LoadInitial {
        sender_id: u64,
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    NewTestcase {
        sender_id: u64,
        testcase: Testcase<I>,
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    UpdateStats {
        sender_id: u64,
        new_execs: usize,
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    Crash {
        sender_id: u64,
        input: I,
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    Timeout {
        sender_id: u64,
        input: I,
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    Log {
        sender_id: u64,
        severity_level: u8,
        message: String,
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    None {
        phantom: PhantomData<(S, C, E, I, R)>,
    },
    //Custom {sender_id: u64, custom_event: CE},
}

impl<S, C, E, I, R> Event<S, C, E, I, R>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    E: Executor<I>,
    I: Input,
    R: Rand,
    //CE: CustomEvent<S, C, E, I, R>,
{
    pub fn name(&self) -> &str {
        match self {
            Event::LoadInitial {
                sender_id: _,
                phantom: _,
            } => "Initial",
            Event::NewTestcase {
                sender_id: _,
                testcase: _,
                phantom: _,
            } => "New Testcase",
            Event::UpdateStats {
                sender_id: _,
                new_execs: _,
                phantom: _,
            } => "Stats",
            Event::Crash {
                sender_id: _,
                input: _,
                phantom: _,
            } => "Crash",
            Event::Timeout {
                sender_id: _,
                input: _,
                phantom: _,
            } => "Timeout",
            Event::Log {
                sender_id: _,
                severity_level: _,
                message: _,
                phantom: _,
            } => "Log",
            Event::None { phantom: _ } => "None",
            //Event::Custom {sender_id, custom_event} => custom_event.name(),
        }
    }

    fn handle_in_broker(
        &self,
        /*broker: &dyn EventManager<S, C, E, I, R>,*/ _state: &mut S,
        _corpus: &mut C,
    ) -> Result<BrokerEventResult, AflError> {
        match self {
            Event::LoadInitial {
                sender_id: _,
                phantom: _,
            } => Ok(BrokerEventResult::Handled),
            Event::NewTestcase {
                sender_id: _,
                testcase: _,
                phantom: _,
            } => Ok(BrokerEventResult::Forward),
            Event::UpdateStats {
                sender_id: _,
                new_execs: _,
                phantom: _,
            } => {
                // TODO
                Ok(BrokerEventResult::Handled)
            }
            Event::Crash {
                sender_id: _,
                input: _,
                phantom: _,
            } => Ok(BrokerEventResult::Handled),
            Event::Timeout {
                sender_id: _,
                input: _,
                phantom: _,
            } => {
                // TODO
                Ok(BrokerEventResult::Handled)
            }
            Event::Log {
                sender_id,
                severity_level,
                message,
                phantom: _,
            } => {
                //TODO: broker.log()
                #[cfg(feature = "std")]
                println!("{}[{}]: {}", sender_id, severity_level, message);
                Ok(BrokerEventResult::Handled)
            },
            Event::None {
                phantom: _,
            } => Ok(BrokerEventResult::Handled)
            //Event::Custom {sender_id, custom_event} => custom_event.handle_in_broker(state, corpus),
            //_ => Ok(BrokerEventResult::Forward),
        }
    }

    fn handle_in_client(
        &self,
        /*client: &dyn EventManager<S, C, E, I, R>,*/ _state: &mut S,
        corpus: &mut C,
    ) -> Result<(), AflError> {
        match self {
            Event::NewTestcase {
                sender_id: _,
                testcase,
                phantom: _,
            } => {
                corpus.add(testcase.clone());
                Ok(())
            }
            _ => Err(AflError::Unknown(
                "Received illegal message that message should not have arrived.".into(),
            )),
        }
    }

    // TODO serialize and deserialize, defaults to serde
}

pub trait EventManager<S, C, E, I, R>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    E: Executor<I>,
    I: Input,
    R: Rand,
{
    /// Check if this EventaManager support a given Event type
    /// To compare events, use Event::name().as_ptr()
    fn enabled(&self) -> bool;

    /// Fire an Event
    fn fire(&mut self, event: Event<S, C, E, I, R>) -> Result<(), AflError>;

    /// Lookup for incoming events and process them.
    /// Return the number of processes events or an error
    fn process(&mut self, state: &mut S, corpus: &mut C) -> Result<usize, AflError>;

    fn on_recv(&self, _state: &mut S, _corpus: &mut C) -> Result<(), AflError> {
        // TODO: Better way to move out of testcase, or get ref
        //Ok(corpus.add(self.testcase.take().unwrap()))
        Ok(())
    }
}

/*TODO
    fn on_recv(&self, state: &mut S, _corpus: &mut C) -> Result<(), AflError> {
        println!(
            "#{}\t exec/s: {}",
            state.executions(),
            //TODO: Count corpus.entries().len(),
            state.executions_over_seconds()
        );
        Ok(())
    }
*/

#[cfg(feature = "std")]
pub struct LoggerEventManager<S, C, E, I, R, W>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    I: Input,
    E: Executor<I>,
    R: Rand,
    W: Write,
    //CE: CustomEvent<S, C, E, I, R>,
{
    events: Vec<Event<S, C, E, I, R>>,
    writer: W,
}

#[cfg(feature = "std")]
impl<S, C, E, I, R, W> EventManager<S, C, E, I, R> for LoggerEventManager<S, C, E, I, R, W>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    E: Executor<I>,
    I: Input,
    R: Rand,
    W: Write,
    //CE: CustomEvent<S, C, E, I, R>,
{
    fn enabled(&self) -> bool {
        true
    }

    fn fire(&mut self, event: Event<S, C, E, I, R>) -> Result<(), AflError> {
        self.events.push(event);
        Ok(())
    }

    fn process(&mut self, state: &mut S, corpus: &mut C) -> Result<usize, AflError> {
        // TODO: iterators
        let mut handled = vec![];
        for x in self.events.iter() {
            handled.push(x.handle_in_broker(state, corpus)?);
        }
        handled
            .iter()
            .zip(self.events.iter())
            .map(|(x, event)| match x {
                BrokerEventResult::Forward => event.handle_in_client(state, corpus),
                // Ignore broker-only events
                BrokerEventResult::Handled => Ok(()),
            })
            .for_each(drop);
        let count = self.events.len();
        dbg!("Handled {} events", count);
        self.events.clear();

        Ok(count)
    }
}

#[cfg(feature = "std")]
impl<S, C, E, I, R, W> LoggerEventManager<S, C, E, I, R, W>
where
    S: State<C, E, I, R>,
    C: Corpus<I, R>,
    I: Input,
    E: Executor<I>,
    R: Rand,
    W: Write,
    //TODO CE: CustomEvent,
{
    pub fn new(writer: W) -> Self {
        Self {
            events: vec![],
            writer: writer,
        }
    }
}
