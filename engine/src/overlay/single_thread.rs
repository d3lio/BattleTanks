use overlay::overlay::OverlayBase;
use overlay::WindowParams;

use std::cell::RefCell;

pub struct Overlay {
    internal: RefCell<OverlayBase>
}

impl Overlay {
    pub fn new(width: u32, height: u32) -> Overlay {
        return Overlay {
            internal: RefCell::new(OverlayBase::new(width, height))
        };
    }

    pub fn update(&self) {
        self.internal.borrow_mut().update();
    }

    pub fn draw(&self) {
        self.internal.borrow().draw();
    }

    pub fn make_window(&self, name: &str, data: WindowParams) -> Window {
        let index = self.internal.borrow_mut().make_window(name, data);
        return Window {
            ovl: &self.internal,
            index: index,
        };
    }

    pub fn root(&self) -> Window {
        return Window {
            ovl: &self.internal,
            index: 0,
        };
    }

    pub fn window(&self, name: &str) -> Option<Window> {
        self.root().child(name)
    }
}

pub struct Window<'a> {
    ovl: &'a RefCell<OverlayBase>,
    index: usize,
}

impl<'a> Window<'a> {
    /// Get a child by name
    pub fn child(&self, name: &str) -> Option<Window<'a>> {
        let ovl = self.ovl.borrow();
        let window = ovl.window_from_index(self.index).borrow();

        match window.child(&ovl, name) {
            Some(index) => return Some(Window{ovl: self.ovl, index: index}),
            None => return None,
        };
    }

    /// Attaches a new child window.
    ///
    /// # Panics
    /// If `self` and `child` belong to different `Overlay` objects. <br>
    /// If `child` is already attached to another window. <br>
    /// If `self` already contains a child window with the same name as `child`.
    ///
    pub fn attach_child(&self, child: &Window) {
        assert!(self.ovl as *const RefCell<OverlayBase> == child.ovl as *const RefCell<OverlayBase>,
            ERR_WINDOW_DIFF_OVERLAYS);

        let ovl = self.ovl.borrow();
        let mut window = ovl.window_from_index(self.index).borrow_mut();

        window.attach_child(&ovl, child.index);

        ovl.should_update.borrow_mut().push(child.index);
        ovl.should_reindex.set(true);
    }

    /// Detaches a child window.
    ///
    /// # Panics
    /// If `self` and `child` belong to different `Overlay` objects. <br>
    /// If `child` is not attached to `self`.
    pub fn detach_child(&self, child: &Window) {
        assert!(self.ovl as *const RefCell<OverlayBase> == child.ovl as *const RefCell<OverlayBase>,
            ERR_WINDOW_DIFF_OVERLAYS);

        let ovl = self.ovl.borrow();
        let mut child_window = ovl.window_from_index(child.index).borrow_mut();
        let child_parent = child_window.parent;

        match child_parent {
            Some(parent) if parent == self.index => {
                let mut window = ovl.window_from_index(self.index).borrow_mut();
                window.detach_child(&ovl, child.index);
                child_window.parent = None;

                ovl.should_reindex.set(true);
            },
            _ => {
                panic!(format!("Attempting to detach window \"{}\" from non-parent window \"{}\"",
                    ovl.window_from_index(self.index).borrow().full_name(&ovl),
                    child_window.full_name(&ovl)));
            }
        }
    }

    pub fn modify<F> (&self, mod_fn: F)
        where F: Fn(&mut WindowParams)
    {
        let ovl = self.ovl.borrow();
        let mut window = ovl.window_from_index(self.index).borrow_mut();

        ovl.should_update.borrow_mut().push(self.index);

        mod_fn(&mut window.creation_data);
    }
}

const ERR_WINDOW_DIFF_OVERLAYS: &'static str = "Windows belong to different Overlay objects";
