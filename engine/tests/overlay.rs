extern crate engine;
extern crate cgmath;

mod common;

use engine::overlay::{OverlayHandle, WindowHandle, WindowParams};

#[test]
/// Handles to the same window should be equal no matter how one acquires them
fn window_handle_same() {
    common::init_gl();
    let params = WindowParams::default();

    let wnd_root = WindowHandle::new("wnd_root", params);
    assert!(WindowHandle::same(&wnd_root, &wnd_root));

    let wnd1_0 = wnd_root.attach(WindowHandle::new("wnd1", params));
    let wnd1_1 = wnd_root.child("wnd1").unwrap();
    let wnd1_2 = wnd_root.child("wnd1").unwrap();

    assert!(WindowHandle::same(&wnd1_0, &wnd1_1));
    assert!(WindowHandle::same(&wnd1_1, &wnd1_2));
}

#[test]
/// Tests attaching windows and getting windows by path
fn window_handle_paths() {
    common::init_gl();
    let params = WindowParams::default();

    let wnd_root = WindowHandle::new("wnd_root", params);
    let wnd1 = WindowHandle::new("wnd1", params);
    let wnd2 = WindowHandle::new("wnd2", params);
    let wnd3 = WindowHandle::new("wnd3", params);

    let wnd3 = wnd1.attach(wnd3);
    let wnd1 = wnd_root.attach(wnd1);
    let wnd2 = wnd_root.attach(wnd2);

    assert!(WindowHandle::same(&wnd1, &wnd_root.child("wnd1").unwrap()));
    assert!(WindowHandle::same(&wnd2, &wnd_root.child("wnd2").unwrap()));
    assert!(WindowHandle::same(&wnd3, &wnd_root.child("wnd1.wnd3").unwrap()));
    assert!(WindowHandle::same(&wnd3, &wnd1.child("wnd3").unwrap()));

    assert!(wnd_root.child("foo").is_none());
    assert!(wnd_root.child("wnd3").is_none());
    assert!(wnd1.child("wnd2").is_none());
}

// #[test]
// fn window_handle_multiple() {
//     common::init_gl();
//     let params = WindowParams::default();

//     let ovl = OverlayHandle::new(800, 600);

//     ovl.root().attach(WindowHandle::new("wnd1", params));
//     let wnd2 = ovl.root().attach(WindowHandle::new("wnd2", params));

//     let root = ovl.root();
//     let wnd1_1 = root.child("wnd1").unwrap();
//     let wnd1_2 = root.child("wnd1").unwrap();

//     let wnd1_1 = wnd1_1.detach();

//     assert!(WindowHandle::same(&wnd1_1, &wnd1_2));

//     // wnd2.attach(wnd1_1);
//     wnd2.attach(wnd1_2);

//     assert!(WindowHandle::same(&wnd1_1, &wnd1_2));
// }
