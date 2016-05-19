extern crate cgmath;

use overlay::{OverlayData, Window, WindowWeak, WindowData, WindowParams};

use self::cgmath::{Vector2, Vector3, Vector4, VectorSpace};
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
    pub fn new(name: &str, params: WindowParams) -> WindowData {
        return WindowData {
            name: String::from(name),
            params: params,
            overlay: ptr::null_mut(),
            pos: Vec2::zero(),
            size: Vec2::zero(),
            children: Vec::new(),
            parent: WindowWeak(None),
            index_beg: 0,
            index_end: 0,
        };
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
            Some(parent) => parent.borrow().full_path() + SEPR + &self.name,
            None => self.name.clone()
        }
    }
}

impl Window {
    /// Create a new window with the given name and parameters.
    pub fn new(name: &str, params: WindowParams) -> Window {
        Window(Rc::new(Box::new(RefCell::new(WindowData::new(name, params)))))
    }

    /// Get a child by relative path
    pub fn child(&self, path: &str) -> Option<Window> {
        let mut next_window = self.clone();
        let mut path = path;

        'outer: loop {
            let curr_window = next_window.clone();
            let window = curr_window.0.borrow();

            match path.find(SEPR) {
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

    /// Attaches a new window as a child.
    ///
    /// If the window is attached to an overlay, attaching or detaching windows
    /// will cause a full update to the window hierarchy tree in order to update
    /// the rendering buffers.
    ///
    /// # Panics
    /// If `child` is already attached to another window. <br>
    /// If `self` already contains a child window with the same name as `child`.
    ///
    pub fn attach(&self, child: &Window) {
        {
            let child_ref = child.0.borrow();

            if child_ref.parent.upgrade().is_some() {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because it is already attached",
                    child_ref.full_path(),
                    self.0.borrow().full_path()));
            }

            if self.child(&child_ref.name).is_some() {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because the second already has a child with the same name",
                    child_ref.full_path(),
                    self.0.borrow().full_path()));
            }
        }

        self.0.borrow_mut().children.push(child.clone());
        child.0.borrow_mut().parent = WindowWeak(Some(Rc::downgrade(&self.0)));
        child.0.borrow_mut().overlay = self.0.borrow().overlay;

        if let Some(ovl) = self.0.borrow_mut().overlay_mut() {
            ovl.should_reindex = true;
        }
    }

    /// Detaches the window from its parent.
    ///
    /// If the window is attached to an overlay, attaching or detaching windows
    /// will cause a full update to the window hierarchy tree in order to update
    /// the rendering buffers.
    ///
    /// # Panics
    /// If `self` is not attached to another window.
    pub fn detach(&self) {
        let parent: Window;
        if let Some(p) = self.0.borrow().parent.upgrade() {
            parent = Window(p);
        } else {
            panic!(format!("Cannot detach window \"{}\" because it is not attached no anything",
                self.0.borrow().full_path()));
        }

        self.0.borrow_mut().parent = WindowWeak(None);
        parent.0.borrow_mut().children.retain(|item| self != item);

        if let Some(ovl) = self.0.borrow_mut().overlay_mut() {
            ovl.should_reindex = true;
        }

        // recursively update all children to set overlay to null
        helper(self);

        fn helper(window: &Window) {
            window.0.borrow_mut().overlay = ptr::null_mut();

            for child in &window.0.borrow().children {
                helper(child);
            }
        }
    }

    // TODO: implement
    // pub fn detach_child(&self, path: &str) -> Window<'static> {
    // }

    /// Executes a closure which can be used to modify the window parameters.
    ///
    /// # Example
    /// ```no_run
    /// # extern crate cgmath;
    /// # extern crate engine;
    /// # use engine::overlay::Window;
    /// # use cgmath::{vec2, vec3};
    /// # fn main () {
    /// // A window initialized somewhere else
    /// let window: Window;
    /// # unsafe { window = std::mem::uninitialized(); }
    ///
    /// window.modify(|params| {
    ///     params.pos.x = vec3(0.5, 0.0, 50.0);
    ///     params.texcoord = [vec2(0.0, 0.0), vec2(5.0, 0.0), vec2(5.0, 5.0), vec2(0.0, 5.0)];
    /// });
    /// # }
    /// ```
    pub fn modify<F> (&self, modfn: F)
        where F: Fn(&mut WindowParams)
    {
        modfn(&mut self.0.borrow_mut().params);

        unsafe {
            let ovl = self.0.borrow().overlay;
            if ovl != ptr::null_mut() {
                (*ovl).update_subtree(self.clone());
            }
        }
    }
}

impl Eq for Window {}
impl PartialEq for Window {
    fn eq(&self, other: &Window) -> bool {
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

/// Separator character for window paths
const SEPR: &'static str = "/";
