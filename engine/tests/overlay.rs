extern crate engine;
extern crate cgmath;

mod common;

use engine::overlay::{Window, WindowParams};

#[test]
/// Handles to the same window should be equal no matter how one acquires them
fn window_handle_eq() {
    common::init_gl();
    let params = WindowParams::default();

    let wnd_root = Window::new("wnd_root", params);
    assert_eq!(wnd_root, wnd_root);

    let wnd1_0 = Window::new("wnd1", params);
    wnd_root.attach(&wnd1_0);

    let wnd1_1 = wnd_root.child("wnd1").unwrap();
    let wnd1_2 = wnd_root.child("wnd1").unwrap();

    assert_eq!(wnd1_0, wnd1_1);
    assert_eq!(wnd1_1, wnd1_2);
}

#[test]
/// Tests attaching windows and getting windows by path
fn window_handle_paths() {
    common::init_gl();
    let params = WindowParams::default();

    let wnd_root = Window::new("wnd_root", params);
    let wnd1 = Window::new("wnd1", params);
    let wnd2 = Window::new("wnd2", params);
    let wnd3 = Window::new("wnd3", params);

    wnd1.attach(&wnd3);
    wnd_root.attach(&wnd1);
    wnd_root.attach(&wnd2);

    assert_eq!(&wnd1, &wnd_root.child("wnd1").unwrap());
    assert_eq!(&wnd2, &wnd_root.child("wnd2").unwrap());
    assert_eq!(&wnd3, &wnd_root.child("wnd1/wnd3").unwrap());
    assert_eq!(&wnd3, &wnd1.child("wnd3").unwrap());

    assert!(wnd_root.child("foo").is_none());
    assert!(wnd_root.child("wnd3").is_none());
    assert!(wnd1.child("wnd2").is_none());
}

// #[test]
// fn window_handle_multiple() {
//     common::init_gl();
//     let params = WindowParams::default();

//     let ovl = OverlayHandle::new(800, 600);

//     ovl.root().attach(Window::new("wnd1", params));
//     let wnd2 = ovl.root().attach(Window::new("wnd2", params));

//     let root = ovl.root();
//     let wnd1_1 = root.child("wnd1").unwrap();
//     let wnd1_2 = root.child("wnd1").unwrap();

//     let wnd1_1 = wnd1_1.detach();

//     assert!(Window::eq(&wnd1_1, &wnd1_2));

//     // wnd2.attach(wnd1_1);
//     wnd2.attach(wnd1_2);

//     assert!(Window::eq(&wnd1_1, &wnd1_2));
// }
