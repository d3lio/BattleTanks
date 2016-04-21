use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Represents an event for the `EventEmitter`.
///
/// See `EventEmitter`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Event(pub &'static str);

/// Single threaded event listener for the `EventEmitter`.
///
/// Unlike other event emitter APIs, this listener implementation holds a reference to a closure
/// *and* a reference to the data it will pass to the closure.<br>
/// This gives us flexibility to call listeners that listen to the same event
/// but operate on different data.
/// You can think of this as a bound function just like what JavaScript's `bind` would produce.
/// Also a single `Listener` may listen to multiple events.
///
/// See `EventEmitter`.
#[derive(Clone)]
pub struct Listener<T> {
    data: Weak<RefCell<T>>,
    closure: Rc<Fn(Event, Rc<RefCell<T>>)>
}

impl<T> Listener<T> {
    /// Create a new listener.
    pub fn new<F>(data: Weak<RefCell<T>>, closure: F) -> Listener<T>
        where F: Fn(Event, Rc<RefCell<T>>) + 'static
    {
        return Listener {
            data: data,
            closure: Rc::new(closure)
        };
    }

    /// Call the listener with an event.
    pub fn call(&self, event: Event) -> bool {
        match self.data.upgrade() {
            Some(data_rc) => {
                (*self.closure)(event, data_rc);
                return true;
            },
            None => return false
        }
    }
}

/// Single threaded non-parallel event emitter.
///
/// It sustains itself by removing any `Listener`s, holding invalid data, from the subscribers.
///
/// #Examples
/// ```
/// #[macro_use(wrap)]
/// extern crate engine;
/// # fn main() {
/// use engine::core::{Event, Listener, EventEmitter};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// // You can explicitry give the emitter the type of data it's listeners will hold
/// // or you can just let it find the type itself with the `on` function later.
/// let mut emitter = EventEmitter::new();
///
/// // A type of data, in this case a u32.
/// let val1 = wrap!(1u32);
/// let listener = Listener::new(Rc::downgrade(&val1), |event: Event, data: Rc<RefCell<u32>>| {
///     match event {
///         Event("move") => {
///             *data.borrow_mut() += 1;
///         },
///         Event("rotate") => {
///             *data.borrow_mut() *= 3;
///         },
///         _ => {}
///     }
/// });
/// // When cloning a listener it just clones the internal references to the same data.
/// // This is the way to subscribe a listener to multiple events.
/// emitter.on(Event("move"), listener.clone());
/// emitter.on(Event("rotate"), listener);
///
/// // A subscope.
/// {
///     // Same type of data as val1.
///     let val2 = wrap!(42u32);
///     emitter.on(Event("move"), Listener::new(Rc::downgrade(&val2),
///         |_: Event, _: Rc<RefCell<u32>>| {
///             // Should never be called since val2 will be gone before the emit.
///             assert!(false);
///         }
///     ));
///     // val2 is destroyed here and so will be the listener in the emitter when `emit` is called.
/// }
///
/// // Same type of data as val1.
/// let val3 = wrap!(21u32);
/// // Second listener to the "move" event but operates on val3 instead of val1
/// // This one is to show the advantages over giving the data to the `emit` function
/// // where all listeners would only be able to operate on the same given data.
/// // Also you can ignore the event or even the data as in val2's listener.
/// emitter.on(Event("move"), Listener::new(Rc::downgrade(&val3),
///     |_: Event, data: Rc<RefCell<u32>>| {
///         *data.borrow_mut() *= 2;
///     }
/// ));
///
/// emitter.emit(Event("move"));
/// assert_eq!(*val1.borrow(), 2);
/// assert_eq!(*val3.borrow(), 42);
///
/// emitter.emit(Event("rotate"));
/// assert_eq!(*val1.borrow(), 6);
/// # }
/// ```
pub struct EventEmitter<T> {
    subscribers: RefCell<HashMap<Event, Vec<Listener<T>>>>
}

impl<T> EventEmitter<T> {
    /// Create a new event emitter.
    pub fn new() -> EventEmitter<T> {
        return EventEmitter {
            subscribers: RefCell::new(HashMap::new())
        };
    }

    /// Subscribe a `Listener` to be triggered by the given `Event`.
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
    pub fn emit(&self, event: Event) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.retain(|ref listener| {
                listener.call(event)
            });
        }
    }
}
