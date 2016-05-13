use core::Data;

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Represents an event.
///
/// It's nothing more than a string literal wrapper.
///
/// See `EventEmitter`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Event(pub &'static str);

/// Creates a vector of events from string literals.
///
/// # Examples
///
/// ```
/// #[macro_use(events)]
/// extern crate engine;
/// # fn main() {
/// use engine::core::Event;
///
/// let events = events!("move", "rotate");
/// assert_eq!(events, vec!(Event("move"), Event("rotate")));
/// # }
/// ```
#[macro_export]
macro_rules! events {
    ( $( $x: expr ),* ) => {
        vec![ $( Event($x), )* ]
    }
}

/// Single threaded event listener for the `EventEmitter`.
///
/// Unlike other event emitter APIs, this listener implementation holds a closure and
/// a contex that it will pass to the closure under the form of `&T`.<br>
/// You can think of this as a bound function just like what JavaScript's `bind` would produce.
/// It gives us flexibility to call listeners that listen to the same event
/// but operate on different data on top of the flat emitted data.
/// Also a single listener may listen to multiple events.
///
/// See `EventEmitter` for more info.
pub struct Listener<T: ?Sized> {
    data: Weak<T>,
    closure: Rc<Box<Fn(&T, &Event, &Data)>>
}

impl<T: ?Sized> Listener<T> {
    /// Create a new listener.
    pub fn new(data: Weak<T>, closure: Box<Fn(&T, &Event, &Data)>) -> Listener<T> {
        return Listener {
            data: data,
            closure: Rc::new(closure)
        };
    }

    /// Call the listener with an event and some data.
    ///
    /// Usually this will be the emitted data from an `EventEmitter` but you can `call` manually.
    fn call(&self, event: &Event, event_data: &Data) -> bool {
        match self.data.upgrade() {
            Some(this_rc) => {
                (*self.closure)(&this_rc, event, event_data);
                return true;
            },
            None => return false
        }
    }
}

impl<T: ?Sized> Clone for Listener<T> {
    fn clone(&self) -> Listener<T> {
        return Listener {
            data: self.data.clone(),
            closure: self.closure.clone()
        };
    }
}

/// Single threaded non-parallel event emitter.
///
/// It sustains itself by removing any subscribed `Listener`s, holding invalid data.
///
/// # Examples
///
/// ```
/// #[macro_use(wrap)]
/// extern crate engine;
/// # fn main() {
/// use engine::core::{Data, Event, EventEmitter, Listener};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// // You can explicitry give the emitter the type of data it's listeners will hold
/// // or you can just let it infer the types itself from its listeners later.
/// let mut emitter = EventEmitter::new();
///
/// // A type of data, in this case a u32.
/// let val1 = wrap!(1u32);
/// let listener = Listener::new(Rc::downgrade(&val1),
///     Box::new(|this: &RefCell<u32>, event: &Event, event_data: &Data| {
///         match *event {
///             Event("move") => {
///                 *this.borrow_mut() += *event_data.to::<u32>();
///             },
///             Event("rotate") => {
///                 *this.borrow_mut() *= 3;
///             },
///             _ => {}
///         }
///     })
/// );
/// // When cloning a listener it just clones the internal references to the same data.
/// // This is the way to subscribe a listener to multiple events.
/// emitter.on(Event("move"), listener.clone());
/// emitter.on(Event("rotate"), listener);
///
/// // A subscope.
/// {
///     let val2 = wrap!(42u32);
///     // Something good to know is that we can ignore arguments and their types in the closure.
///     emitter.on(Event("move"), Listener::new(Rc::downgrade(&val2),
///         Box::new(|_, _, _| {
///             // Should never be called since val2 will be gone before the emit.
///             assert!(false);
///         })
///     ));
///     // `val2` is destroyed here.
///     // As for the listener, it will get destroyed when `emit(Event("move"), ...)` is called.
/// }
///
/// let val3 = wrap!(21u32);
/// // Another listener to the "move" event but operates on val3 instead of val1.
/// emitter.on(Event("move"), Listener::new(Rc::downgrade(&val3),
///     Box::new(|this: &RefCell<u32>, _: &Event, _: &Data| {
///         *this.borrow_mut() *= 2;
///     })
/// ));
///
///
/// emitter.emit(Event("move"), Data::from(&mut 1u32));
/// assert_eq!(*val1.borrow(), 2);
/// assert_eq!(*val3.borrow(), 42);
///
/// emitter.emit(Event("rotate"), Data::from(&mut 0u32));
/// assert_eq!(*val1.borrow(), 6);
/// # }
/// ```
pub struct EventEmitter<T: ?Sized> {
    subscribers: RefCell<HashMap<Event, Vec<Listener<T>>>>
}

impl<T: ?Sized> EventEmitter<T> {
    /// Create a new event emitter.
    pub fn new() -> EventEmitter<T> {
        return EventEmitter {
            subscribers: RefCell::new(HashMap::new())
        };
    }

    /// Subscribe a listener to be triggered by a given event.
    pub fn on(&mut self, event: Event, listener: Listener<T>) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.push(listener);
            return;
        }

        self.subscribers.borrow_mut().insert(event, vec![listener]);
    }

    /// Emits an event that triggers all of it's subscribers.
    ///
    /// Listeners will be called in the order they were added.
    pub fn emit(&self, event: Event, event_data: Data) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.retain(|ref listener| {
                listener.call(&event, &event_data)
            });
        }
    }
}
