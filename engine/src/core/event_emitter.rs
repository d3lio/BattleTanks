use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;

/// A structure representing an event.
///
/// See `EventEmitter`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Event(pub &'static str);

impl Deref for Event {
    type Target = str;

    fn deref(&self) -> &'static str {
        return &self.0;
    }
}

/// A structure representing an event listener.
///
/// Holds a reference to a closure and a reference to the data it will pass to the closure.<br>
/// This gives us flexibility to call listeners that listen to the same event
/// but operate on different data. Also a single `Listener` may listen to multiple events.
///
/// See `EventEmitter`.
#[derive(Clone)]
pub struct Listener<T> {
    data: Weak<RefCell<T>>,
    closure: Rc<Fn(Rc<RefCell<T>>, Event)>
}

impl<T> Listener<T> {
    /// Create a new listener.
    pub fn new<F>(data: Weak<RefCell<T>>, closure: F) -> Listener<T>
        where F: Fn(Rc<RefCell<T>>, Event) + 'static
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
                (*self.closure)(data_rc, event);
                return true;
            },
            None => return false
        }
    }
}

/// A structure representing an event emitter.
///
/// It sustains itself by removing any `Listener`s, holding invalid data, from the subscribers.
///
/// #Examples
/// ```
/// use engine::core::{Event, Listener, EventEmitter};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// // You can explicitry give the emitter the type of data it will hold
/// // or you can just let it find the type itself when `add`ing later.
/// let mut emitter = EventEmitter::new();
///
/// // A type of data in this case a u32.
/// let val1 = Rc::new(RefCell::new(1u32));
/// let listener = Listener::new(Rc::downgrade(&val1), |data: Rc<RefCell<u32>>, event: Event| {
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
/// emitter.add(Event("move"), listener.clone());
/// emitter.add(Event("rotate"), listener);
///
/// {
///     // Same type of data as val1.
///     let val2 = Rc::new(RefCell::new(42u32));
///     emitter.add(Event("move"), Listener::new(Rc::downgrade(&val2),
///         |_: Rc<RefCell<u32>>, _: Event| {
///             // Should never be called since val2 will be gone before the emit.
///             assert!(false);
///         }
///     ));
///     // val2 is destroyed here and so will be the listener in the emitter when `emit` is called.
/// }
///
/// // Same type of data as val1.
/// let val3 = Rc::new(RefCell::new(21u32));
/// // Second listener to the "move" event but operates on val3 instead of val1
/// // This one is to show the advantages over giving the data to the `emit` function
/// // where all listeners would only be able to operate on the same given data.
/// // Also you can ignore the event or even the data as in val2.
/// emitter.add(Event("move"), Listener::new(Rc::downgrade(&val3),
///     |data: Rc<RefCell<u32>>, _: Event| {
///         *data.borrow_mut() *= 2;
///     }
/// ));
///
/// emitter.emit(Event("move"));
/// assert_eq!(*val1.borrow_mut(), 2);
/// assert_eq!(*val3.borrow_mut(), 42);
///
/// emitter.emit(Event("rotate"));
/// assert_eq!(*val1.borrow_mut(), 6);
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

    /// Subscribe a `Listener` to the given `Event`.
    pub fn add(&mut self, event: Event, listener: Listener<T>) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.push(listener);
            return;
        }

        self.subscribers.borrow_mut().insert(event, vec![listener]);
    }

    /// Emits an event that triggers all of it's subscribers.
    pub fn emit(&self, event: Event) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.retain(|ref listener| {
                listener.call(event)
            });
        }
    }
}
