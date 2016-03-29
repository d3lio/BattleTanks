use overlay::overlay::OverlayBase;
use overlay::WindowParams;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Overlay {
    internal: Rc<RefCell<OverlayBase>>
}

pub struct Window {
    ovl: Weak<RefCell<OverlayBase>>,
    index: usize,
}

impl Overlay {
    pub fn new(width: u32, height: u32) -> Overlay {
        return Overlay {
            internal: Rc::new(RefCell::new(OverlayBase::new(width, height)))
        };
    }

    pub fn update(&self) {
        self.internal.borrow_mut().update();
    }

    pub fn draw(&self) {
        self.internal.borrow().draw();
    }

    pub fn make_window(&self, data: WindowParams) -> Window {
        let index = self.internal.borrow_mut().make_window(data);
        return Window {
            ovl: Rc::downgrade(&self.internal),
            index: index,
        };
    }

    pub fn root(&self) -> Window {
        return Window {
            ovl: Rc::downgrade(&self.internal),
            index: 0,
        };
    }

    pub fn window(&self, name: &str) -> Option<Window> {
        self.root().child(name)
    }
}

impl Window {
    pub fn child(&self, name: &str) -> Option<Window> {
        let ovl_ref = self.ovl.upgrade().unwrap();
        let ovl = ovl_ref.borrow();
        let window = ovl.window_from_index(self.index).borrow();

        match window.child(&ovl, name) {
            Some(index) => return Some(Window{ovl: self.ovl.clone(), index: index}),
            None => return None,
        };
    }

    /// Attaches a new child window.
    ///
    /// # Panics
    /// If `self` and `child` belong to different `Overlay` objects or the `Overlay` containing them has been destroyed. <br>
    /// If `child` is already attached to another window. <br>
    /// If `self` already contains a child with the same name as `child`.
    ///
    pub fn add_child(&self, child: &Window) {
        let window_ovl = self.ovl.upgrade().unwrap();
        let child_ovl = child.ovl.upgrade().unwrap();
        assert!(&*window_ovl as *const _ == &*child_ovl as *const _, ERR_WINDOW_DIFF_OVERLAYS);

        let ovl = window_ovl.borrow();
        let mut window = ovl.window_from_index(self.index).borrow_mut();

        window.add_child(&ovl, child.index);

        ovl.invalid_data.borrow_mut().push(child.index);
        ovl.invalid_topol.set(true);
    }

    // pub fn rem_child(&self, name: &str) -> Option<Window> {
    //     None
    // }
}

const ERR_WINDOW_DIFF_OVERLAYS: &'static str = "Windows belong to different Overlay objects";
