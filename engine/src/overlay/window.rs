extern crate cgmath;

use overlay::{OverlayData, Window, WindowWeak, WindowData, WindowParams};

use self::cgmath::{Vector2, Vector3, Vector4, Vector};
use std::cell::RefCell;
use std::rc::Rc;
use std::ptr;
use std::fmt::{self, Debug, Formatter};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

impl Debug for WindowParams {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "WindowParams{{ ... }}")
    }
}

impl Default for WindowParams {
    fn default() -> WindowParams {
        WindowParams {
            pos: Vector2{x: Vector3::zero(), y: Vector3::zero()},
            size: Vector2{x: Vector3::new(1.0, 0.0, 0.0), y: Vector3::new(0.0, 1.0, 0.0)},
            color: [Vector4::zero(); 4],
            texcoord: [Vector2::new(-1.0, -1.0); 4],
            shown: true,
        }
    }
}

impl WindowData {
    pub fn new(name: &str, data: WindowParams) -> WindowData {
        return WindowData {
            name: String::from(name),
            creation_data: data,
            overlay: ptr::null_mut(),
            pos: Vec2::zero(),
            size: Vec2::zero(),
            children: Vec::new(),
            parent: WindowWeak(None),
            vbo_beg: -1,
            vbo_end: -1,
        };
    }

    pub fn overlay(&self) -> Option<&OverlayData> {
        unsafe {
            if self.overlay == ptr::null_mut() {
                return None;
            } else {
                return Some(&*self.overlay);
            }
        }
    }

    pub fn overlay_mut(&mut self) -> Option<&mut OverlayData> {
        unsafe {
            if self.overlay == ptr::null_mut() {
                return None;
            } else {
                return Some(&mut *self.overlay);
            }
        }
    }

    pub fn full_path(&self) -> String {
        match self.parent.upgrade() {
            Some(parent) => parent.borrow().full_path() + "." + &self.name,
            None => self.name.clone()
        }
    }
}

impl Window {
    pub fn new(name: &str, data: WindowParams) -> Window {
        Window(Rc::new(Box::new(RefCell::new(WindowData::new(name, data)))))
    }

    pub fn child(&self, path: &str) -> Option<Window> {
        let mut next_window = self.clone();
        let mut path = path;

        'outer: loop {
            let curr_window = next_window.clone();
            let window = curr_window.0.borrow();

            match path.find('.') {
                Some(seperator_pos) => {
                    let (curr_name, rest_path) = path.split_at(seperator_pos);
                    let rest_path = &rest_path[1..];

                    for child in &window.children {
                        if child.0.borrow().name == curr_name {
                            next_window = child.clone();
                            path = rest_path;
                            continue 'outer;
                        }
                    }
                    return None;
                },
                None => {
                    for child in &window.children {
                        if child.0.borrow().name == path {
                            return Some(child.clone());
                        }
                    }
                    return None;
                }
            }
        }
    }

    pub fn attach(&self, child: &Window) {
        {
            let window_ref = self.0.borrow();
            let child_ref = child.0.borrow();

            assert!(child_ref.overlay().is_none(), "child overlay is not none");

            if child_ref.parent.upgrade().is_some() {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because it is already attached",
                    child_ref.full_path(),
                    window_ref.full_path()));
            }

            if self.child(&child_ref.name).is_some() {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because the second already has a child with the same name",
                    child_ref.full_path(),
                    window_ref.full_path()));
            }
        }

        self.0.borrow_mut().children.push(child.clone());
        child.0.borrow_mut().parent = WindowWeak(Some(Rc::downgrade(&self.0)));
        child.0.borrow_mut().overlay = self.0.borrow().overlay;

        if let Some(ovl) = self.0.borrow_mut().overlay_mut() {
            ovl.should_reindex = true;
        }
    }

    pub fn detach(&self) {
        let parent: Window;
        if let Some(p) = self.0.borrow().parent.upgrade() {
            parent = Window(p);
        } else {
            panic!("window is not attached");
        }

        let mut parent_mut = parent.0.borrow_mut();
        parent_mut.children.retain(|item| !Window::same(self, item));

        let mut window_mut = self.0.borrow_mut();
        window_mut.parent = WindowWeak(None);

        if let Some(ovl) = window_mut.overlay_mut() {
            ovl.should_reindex = true;
        }

        // TODO: update recursively
        window_mut.overlay = ptr::null_mut();
    }

    // TODO: implement
    // pub fn detach_child(&self, path: &str) -> Window<'static> {
    // }

    pub fn modify<F> (&self, modfn: F)
        where F: Fn(&mut WindowParams)
    {
        modfn(&mut self.0.borrow_mut().creation_data);

        if let Some(ovl) = self.0.borrow().overlay() {
            ovl.update_subtree(self.clone());
        }
    }

    pub fn same(&self, other: &Window) -> bool {
        &*self.0 as *const Box<_> == &*other.0 as *const Box<_>
    }
}

impl WindowWeak {
    pub fn upgrade(&self) -> Option<Rc<Box<RefCell<WindowData>>>> {
        match self.0 {
            Some(ref weak) => weak.upgrade(),
            None => None,
        }
    }
}
