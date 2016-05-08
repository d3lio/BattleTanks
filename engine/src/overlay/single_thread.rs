use overlay::Overlay;
use overlay::window::{Window, WindowParams};
use overlay::unwrap_weak;

use std::cell::RefCell;
use std::rc::Rc;

pub struct OverlayHandle {
    internal: RefCell<Overlay>,
}

impl OverlayHandle {
    #[inline]
    pub fn new(width: u32, height: u32) -> OverlayHandle {
        OverlayHandle {
            internal: RefCell::new(Overlay::new(width, height)),
        }
    }

    #[inline]
    pub fn draw(&self) {
        let mut ovl = self.internal.borrow_mut();
        ovl.update();
        ovl.draw();
    }

    #[inline]
    pub fn root(&self) -> WindowHandle {
        WindowHandle {
            overlay: Some(&self.internal),
            window: self.internal.borrow().root.clone(),
        }
    }
}

// TODO: there could be a problem if I get two handles to the same window
// and change it trough one of them. The second will be invalid but won't know that!
pub struct WindowHandle<'a> {
    overlay: Option<&'a RefCell<Overlay>>,
    window: Rc<Box<RefCell<Window>>>,
}

impl<'a> WindowHandle<'a> {
    pub fn new(name: &str, data: WindowParams) -> WindowHandle {
        return WindowHandle {
            overlay: None,
            window: Rc::new(Box::new(RefCell::new(Window::new(name, data)))),
        }
    }

    pub fn child(&self, path: &str) -> Option<WindowHandle> {
        let mut next_window = self.window.clone();
        let mut path = path;

        'outer: loop {
            let curr_window = next_window.clone();
            let window = curr_window.borrow();

            match path.find('.') {
                Some(seperator_pos) => {
                    let (curr_name, rest_path) = path.split_at(seperator_pos);
                    let rest_path = &rest_path[1..];

                    for child in &window.children {
                        if child.borrow().name == curr_name {
                            next_window = child.clone();
                            path = rest_path;
                            continue 'outer;
                        }
                    }
                    return None;
                },
                None => {
                    for child in &window.children {
                        if child.borrow().name == path {
                            return Some(WindowHandle{overlay: self.overlay, window: child.clone()});
                        }
                    }
                    return None;
                }
            }
        }
    }

    pub fn attach(&self, child: WindowHandle) -> WindowHandle<'a> {
        assert!(child.overlay.is_none(), "");

        {
            let window = self.window.borrow();
            let child_window = child.window.borrow();

            if let Some(parent) = unwrap_weak(&child_window.parent) {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because it is already attached to window \"{}\"",
                    child_window.name,
                    window.full_path(),
                    parent.borrow().full_path()));
            }

            if !self.child(&child_window.name).is_none() {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because the second already has a child with the same name",
                    child_window.name,
                    window.full_path()));
            }
        }

        self.window.borrow_mut().children.push(child.window.clone());
        child.window.borrow_mut().parent = Some(Rc::downgrade(&self.window));

        if let Some(ovl) = self.overlay {
            ovl.borrow_mut().should_reindex = true;
        }

        return WindowHandle {
            overlay: self.overlay,
            window: child.window,
        };
    }

    pub fn detach(self) -> WindowHandle<'static> {
        let parent: Rc<Box<RefCell<Window>>>;
        {
            let parent_opt = unwrap_weak(&self.window.borrow().parent);
            match parent_opt {
                Some(p) => parent = p,
                None => {
                    match self.overlay {
                        None => return WindowHandle {
                            overlay: None,
                            window: self.window,
                        },
                        Some(_) => panic!("Cannot detach a root window"),
                    }
                },
            };
        }

        self.window.borrow_mut().parent = None;
        parent.borrow_mut().children.retain(|item| {
            &***item as *const RefCell<_> != &**self.window as *const RefCell<_> });

        if let Some(ovl) = self.overlay {
            ovl.borrow_mut().should_reindex = true;
        }

        return WindowHandle {
            overlay: None,
            window: self.window,
        };
    }

    // TODO: implement
    // pub fn detach_child(&self, path: &str) -> WindowHandle<'static> {
    // }

    pub fn modify<F> (&self, modfn: F)
        where F: Fn(&mut WindowParams)
    {
        modfn(&mut self.window.borrow_mut().creation_data);

        if let Some(ovl) = self.overlay {
            ovl.borrow().update_subtree(self.window.clone());
        }
    }

    pub fn same<'b> (&self, other: &WindowHandle<'b>) -> bool {
        &**self.window as *const RefCell<_> == &**other.window as *const RefCell<_>
    }
}

impl Window {
    fn full_path(&self) -> String {
        match unwrap_weak(&self.parent) {
            Some(parent) => parent.borrow().full_path() + "." + &self.name,
            None => self.name.clone()
        }
    }
}
