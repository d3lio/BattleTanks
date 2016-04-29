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

/// Single threaded event listener for the `EventEmitter`.
///
/// Unlike other event emitter APIs, this listener implementation holds an `Rc` to a closure
/// *and* a `Weak` to the data it will pass to the closure.<br>
/// This gives us flexibility to call listeners that listen to the same event
/// but operate on different data on top of the flat emitted data.
/// You can think of this as a bound function just like what JavaScript's `bind` would produce.
/// Also a single `Listener` may listen to multiple events.
///
/// See `EventEmitter` for more info.
#[derive(Clone)]
pub struct Listener<T: ?Sized, Q> {
    data: Weak<T>,
    closure: Rc<Fn(&T, &Event, &mut Q, &EventEmitter<T, Q>)>
}

impl<T: ?Sized, Q> Listener<T, Q> {
    /// Create a new listener.
    pub fn new<F>(data: Weak<T>, closure: F) -> Listener<T, Q>
        where F: Fn(&T, &Event, &mut Q, &EventEmitter<T, Q>) + 'static
    {
        return Listener {
            data: data,
            closure: Rc::new(closure)
        };
    }

    /// Call the listener with an event and some data.
    ///
    /// Usually this will be the emitted data from an `EventEmitter` but you can `call` manually.
    pub fn call(&self, event: &Event, event_data: &mut Q, emitter: &EventEmitter<T, Q>) -> bool {
        match self.data.upgrade() {
            Some(this_rc) => {
                (*self.closure)(&this_rc, event, event_data, emitter);
                return true;
            },
            None => return false
        }
    }

    fn is_valid(&self) -> bool {
        self.data.upgrade().is_some()
    }
}

/// Single threaded non-parallel event emitter.
///
/// It sustains itself by removing any subscribed `Listener`s, holding invalid data.<br>
///
/// # Examples
///
/// TODO: revisit this example
///
/// ```
/// #[macro_use(wrap)]
/// extern crate engine;
/// # fn main() {
/// use engine::core::{Event, Listener, EventEmitter};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// // Values that will be emitted later and since we will use references
/// // the bindings need to live longer than the emitter.
/// let mut some_data = Some(1);
/// let mut none_data = None;
///
/// // You can explicitry give the emitter the type of data it's listeners will hold and
/// // the type of the emitted data or you can just let it find the types itself
/// // from its listeners later.
/// let mut emitter = EventEmitter::new();
///
/// // A type of data, in this case a u32.
/// let val1 = wrap!(1u32);
/// let listener = Listener::new(Rc::downgrade(&val1),
///     |this: &RefCell<u32>, event: &Event, event_data: &mut Option<i32>, emitter| {
///         match *event {
///             Event("move") => {
///                 *this.borrow_mut() += event_data.unwrap() as u32;
///             },
///             Event("rotate") => {
///                 *this.borrow_mut() *= 3;
///             },
///             _ => {}
///         }
///     }
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
///         |_: &RefCell<u32>, _: &Event, _: &mut Option<i32>, _| {
///             // Should never be called since val2 will be gone before the emit.
///             assert!(false);
///         }
///     ));
///     // `val2` is destroyed here.
///     // As for the listener, it will get destroyed when `emit(Event("move"), ...)` is called.
/// }
///
/// let val3 = wrap!(21u32);
/// // Another listener to the "move" event but operates on val3 instead of val1
/// // This one is to show the advantages over solely giving the data to the `emit` function
/// // where all listeners would only be able to operate on the same given data.
/// // Also you can ignore any given value to the closure as in val2's listener.
/// emitter.on(Event("move"), Listener::new(Rc::downgrade(&val3),
///     |this: &RefCell<u32>, _: &Event, _: &mut Option<i32>, _| {
///         *this.borrow_mut() *= 2;
///     }
/// ));
///
///
/// emitter.emit(Event("move"), &mut some_data);
/// assert_eq!(*val1.borrow(), 2);
/// assert_eq!(*val3.borrow(), 42);
///
/// emitter.emit(Event("rotate"), &mut none_data);
/// assert_eq!(*val1.borrow(), 6);
/// # }
/// ```
pub struct EventEmitter<T: ?Sized, Q> {
    subscribers: RefCell<HashMap<Event, Vec<Listener<T, Q>>>>
}

impl<T: ?Sized, Q> EventEmitter<T, Q> {
    /// Create a new event emitter.
    pub fn new() -> EventEmitter<T, Q> {
        return EventEmitter {
            subscribers: RefCell::new(HashMap::new())
        };
    }

    /// Subscribe a listener to be triggered by a given event.
    pub fn on(&mut self, event: Event, listener: Listener<T, Q>) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.push(listener);
            return;
        }

        self.subscribers.borrow_mut().insert(event, vec![listener]);
    }

    /// Emits an event that triggers all of it's subscribers.
    ///
    /// Listeners will be called in the order they were added.
    pub fn emit(&self, event: Event, event_data: &mut Q) {
        if let Some(ref mut list) = self.subscribers.borrow_mut().get_mut(&event) {
            list.retain(|ref listener| {
                listener.is_valid()
            });

            for listener in list.iter() {
                listener.call(&event, event_data, self);
            }
        }
    }
}
