extern crate engine;
extern crate cgmath;

mod common;

use engine::overlay::{OverlayHandle, WindowHandle, WindowParams};


#[test]
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

