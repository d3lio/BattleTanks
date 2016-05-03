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

    pub fn attach<'b> (&'b self, child: &mut WindowHandle<'b>) {
        assert!(child.overlay.is_none());

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
        child.overlay = self.overlay;

        if let Some(ovl) = self.overlay {
            ovl.borrow_mut().should_reindex = true;
        }
    }

    pub fn detach(&self, child: &WindowHandle) -> WindowHandle {
        let mut found = None;

        self.window.borrow_mut().children.retain(|item| {
            if &***item as *const RefCell<_> != &**child.window as *const RefCell<_> {
                return true;
            }

            item.borrow_mut().parent = None;
            found = Some(item.clone());
            return false;
        });

        match found {
            Some(rc) => {
                if let Some(ovl) = self.overlay {
                    ovl.borrow_mut().should_reindex = true;
                }

                return WindowHandle {
                    overlay: None,
                    window: rc
                };
            },
            None => panic!(format!("Cannot detach window \"{}\" from \"{}\" because the first is not attached to the second",
                self.window.borrow().full_path(),
                child.window.borrow().full_path())),
        };
    }

    pub fn modify<F> (&self, modfn: F)
        where F: Fn(&mut WindowParams)
    {
        modfn(&mut self.window.borrow_mut().creation_data);

        if let Some(ovl) = self.overlay {
            ovl.borrow().update_subtree(self.window.clone());
        }
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
