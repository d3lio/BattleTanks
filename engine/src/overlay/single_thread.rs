use overlay::Overlay;
use overlay::window::{Window, WindowParams};

use std::cell::RefCell;
use std::rc::Rc;

pub struct OverlayHandle {
    internal: RefCell<Overlay>,
}

impl OverlayHandle {
    pub fn draw(&self) {
        let mut ovl = self.internal.borrow_mut();
        ovl.update();
        ovl.draw();
    }

    pub fn root(&self) -> WindowHandle {
        return WindowHandle {
            overlay: Some(&self.internal),
            window: self.internal.borrow().root.clone(),
        };
    }
}

pub struct WindowHandle<'a> {
    overlay: Option<&'a RefCell<Overlay>>,
    window: Rc<Box<RefCell<Window>>>,
}

impl<'a> WindowHandle<'a> {
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
            let child_window = child.window.borrow();
            let window = self.window.borrow();

            if let Some(ref parent_weak) = child_window.parent {
                if let Some(parent) = parent_weak.upgrade() {
                    panic!(format!("Cannot attach window \"{}\" to \"{}\" because it is already attached to window \"{}\"",
                        child_window.name,
                        window.full_path(),
                        parent.borrow().full_path()));
                }
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
        String::from("")
    }
}
