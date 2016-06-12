extern crate glfw;

use std::cell::Cell;
use std::mem;
use std::ptr;
use std::rc::Rc;
use self::glfw::{Key, Action};

use super::{KeyListener, Manager};

#[test]
fn focus() {
    let mgr = unsafe { Manager::new(mem::transmute(ptr::null::<glfw::Window>())) };

    let data = Rc::new(Cell::new(0i32));
    let mut focus = KeyListener::new();
    let mut focus2 = KeyListener::new();

    {
        let data = data.clone();
        focus.on(key_mask![Key::Escape, Key::A, Key::B], Rc::new(move |_, _, _|  data.set(data.get() + 1)), false);
    }

    {
        let data = data.clone();
        focus2.on(key_mask![Key::Escape; Key::B .. Key::D], Rc::new(move |_, _, _| data.set(data.get() + 10)), false);
    }

    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 0);

    mgr.gain_key_focus(&mut focus);
    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 1);
    mgr.emit_key(Key::A, 0, Action::Press);
    assert_eq!(data.get(), 2);
    mgr.emit_key(Key::B, 0, Action::Press);
    assert_eq!(data.get(), 3);

    data.set(0);
    mgr.gain_key_focus(&mut focus2);

    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 10);
    mgr.emit_key(Key::B, 0, Action::Press);
    assert_eq!(data.get(), 20);
    mgr.emit_key(Key::C, 0, Action::Press);
    assert_eq!(data.get(), 30);
    mgr.emit_key(Key::D, 0, Action::Press);
    assert_eq!(data.get(), 40);
    mgr.emit_key(Key::A, 0, Action::Press);
    assert_eq!(data.get(), 41);

    data.set(0);
    mgr.lose_key_focus(&mut focus);

    mgr.emit_key(Key::A, 0, Action::Press);
    assert_eq!(data.get(), 0);

    mgr.lose_key_focus(&mut focus);
    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 10);

    data.set(0);
    mgr.lose_key_focus(&mut focus2);

    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 0);
}

#[test]
fn capture() {
    let mgr = unsafe { Manager::new(mem::transmute(ptr::null::<glfw::Window>())) };

    let data = Rc::new(Cell::new(0i32));
    let mut focus = KeyListener::new();
    let mut focus2 = KeyListener::new();
    let mut focus3 = KeyListener::new();

    {
        let data = data.clone();
        focus.on(key_mask![Key::Escape], Rc::new(move |_, _, _|  data.set(data.get() + 1)), true);
    }

    {
        let data = data.clone();
        focus2.on(key_mask![Key::Escape], Rc::new(move |_, _, _| data.set(data.get() + 10)), false);
    }

    {
        let data = data.clone();
        focus3.on(key_mask![Key::Escape], Rc::new(move |_, _, _| data.set(data.get() + 100)), true);
    }

    mgr.gain_key_focus(&mut focus);
    mgr.gain_key_focus(&mut focus2);
    mgr.gain_key_focus(&mut focus3);

    mgr.emit_key(Key::Escape, 0, Action::Press);
    assert_eq!(data.get(), 110);
}
