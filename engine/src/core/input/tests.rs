extern crate glfw;

use std::cell::Cell;
use std::rc::Rc;
use self::glfw::{Key, Action};

use super::{KeyListener, Manager};

/// Test that the event distribution logic works.
#[test]
fn events() {
    let mgr = Manager::new();
    let data = Rc::new(Cell::new(0i32));
    let mut kl1;
    let mut kl2;

    {
        let data = data.clone();
        kl1 = KeyListener::new(key_mask![Key::Escape, Key::A, Key::B], move |_, _, action| {
            if action == Action::Press {
                data.set(data.get() + 1);
            }
        });
    }

    {
        let data = data.clone();
        kl2 = KeyListener::new(key_mask![Key::Escape; Key::B .. Key::D], move |_, _, action| {
            if action == Action::Press {
                data.set(data.get() + 10);
            }
        });
    }

    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 0);

    kl1.gain_focus(&mgr);
    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 1);
    mgr.emit_key(Key::A, 0, Action::Press);
    mgr.emit_key(Key::A, 0, Action::Release);
    assert_eq!(data.get(), 2);
    mgr.emit_key(Key::B, 0, Action::Press);
    mgr.emit_key(Key::B, 0, Action::Release);
    assert_eq!(data.get(), 3);

    data.set(0);
    kl2.gain_focus(&mgr);

    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 10);
    mgr.emit_key(Key::B, 0, Action::Press);
    mgr.emit_key(Key::B, 0, Action::Release);
    assert_eq!(data.get(), 20);
    mgr.emit_key(Key::C, 0, Action::Press);
    mgr.emit_key(Key::C, 0, Action::Release);
    assert_eq!(data.get(), 30);
    mgr.emit_key(Key::D, 0, Action::Press);
    mgr.emit_key(Key::D, 0, Action::Release);
    assert_eq!(data.get(), 40);
    mgr.emit_key(Key::A, 0, Action::Press);
    mgr.emit_key(Key::A, 0, Action::Release);
    assert_eq!(data.get(), 41);

    data.set(0);
    kl1.lose_focus();

    mgr.emit_key(Key::A, 0, Action::Press);
    mgr.emit_key(Key::A, 0, Action::Release);
    assert_eq!(data.get(), 0);

    kl1.lose_focus();
    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 10);

    data.set(0);
    kl2.lose_focus();

    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 0);
}

/// Test that the `passtrough` parameter works.
#[test]
fn passtrough() {
    let mgr = Manager::new();
    let data = Rc::new(Cell::new(0i32));
    let mut kl1;
    let mut kl2;
    let mut kl3;

    {
        let data = data.clone();
        kl1 = KeyListener::with_passtrough(key_mask![Key::Escape], move |_, _, action| {
            if action == Action::Press {
                data.set(data.get() + 1);
            }
        });
    }

    {
        let data = data.clone();
        kl2 = KeyListener::new(key_mask![Key::Escape], move |_, _, action| {
            if action == Action::Press {
                data.set(data.get() + 10);
            }
        });
    }

    {
        let data = data.clone();
        kl3 = KeyListener::with_passtrough(key_mask![Key::Escape], move |_, _, action| {
            if action == Action::Press {
                data.set(data.get() + 100);
            }
        });
    }

    kl1.gain_focus(&mgr);
    kl2.gain_focus(&mgr);
    kl3.gain_focus(&mgr);

    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 110);
}

/// Test that events with incorrect order are ignored.
/// The correct order is (Press, (Repeat)*, Release)*
#[test]
fn order() {
    let mgr = Manager::new();
    let data = Rc::new(Cell::new(0i32));
    let mut kl1;

    {
        let data = data.clone();
        kl1 = KeyListener::new(key_mask![Key::Escape], move |_, _, action| {
            match action {
                Action::Press => data.set(data.get() + 1),
                Action::Repeat => data.set(data.get() - 1),
                Action::Release => data.set(data.get() * 2),
            }
        });
    }

    kl1.gain_focus(&mgr);

    data.set(10);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(data.get(), 10);

    mgr.emit_key(Key::Escape, 0, Action::Repeat);
    assert_eq!(data.get(), 10);


    data.set(0);
    mgr.emit_key(Key::Escape, 0, Action::Press);
    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 1);

    data.set(0);
    mgr.emit_key(Key::Escape, 0, Action::Repeat);
    mgr.emit_key(Key::Escape, 0, Action::Repeat);
    assert_eq!(data.get(), -2);

    data.set(10);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    mgr.emit_key(Key::Escape, 0, Action::Release);
    mgr.emit_key(Key::Escape, 0, Action::Repeat);
    assert_eq!(data.get(), 20);
}

// Test buffered input.
#[test]
fn buffered() {
    let mgr = Manager::new();
    let mut kl1 = KeyListener::new(key_mask![Key::Escape], move |_, _, _| ());
    let mut kl2 = KeyListener::new(key_mask![Key::Escape], move |_, _, _| ());

    kl1.gain_focus(&mgr);
    kl2.gain_focus(&mgr);

    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(kl1.key_pressed(Key::Escape), false);
    assert_eq!(kl2.key_pressed(Key::Escape), true);
    assert_eq!(kl2.key_pressed(Key::A), false);

    mgr.emit_key(Key::Escape, 0, Action::Repeat);
    assert_eq!(kl1.key_pressed(Key::Escape), false);
    assert_eq!(kl2.key_pressed(Key::Escape), true);

    mgr.emit_key(Key::Escape, 0, Action::Release);
    assert_eq!(kl1.key_pressed(Key::Escape), false);
    assert_eq!(kl2.key_pressed(Key::Escape), false);
}

// Test the situation when a user is holding a key and a listener for the same key gains focus.
#[test]
fn forced_release() {
    let mgr = Manager::new();
    let mut kl1 = KeyListener::new(key_mask![Key::A, Key::B], move |_, _, _| ());
    let mut kl2 = KeyListener::new(key_mask![Key::A], move |_, _, _| ());

    kl1.gain_focus(&mgr);

    mgr.emit_key(Key::A, 0, Action::Press);
    mgr.emit_key(Key::B, 0, Action::Press);
    assert_eq!(kl1.key_pressed(Key::A), true);
    assert_eq!(kl1.key_pressed(Key::B), true);
    assert_eq!(kl2.key_pressed(Key::A), false);

    kl2.gain_focus(&mgr);
    assert_eq!(kl1.key_pressed(Key::A), false);
    assert_eq!(kl1.key_pressed(Key::B), true);
    assert_eq!(kl2.key_pressed(Key::A), false);

    kl2.lose_focus();
    assert_eq!(kl1.key_pressed(Key::A), false);
    assert_eq!(kl1.key_pressed(Key::B), true);
    assert_eq!(kl2.key_pressed(Key::A), false);
}
