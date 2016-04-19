use overlay::OverlayBase;
use overlay::WindowParams;

use std::cell::RefCell;

/// Single threaded Overlay.
pub struct Overlay {
    internal: RefCell<OverlayBase>
}

impl Overlay {
    /// Creates a new overlay containing the whole viewport.
    ///
    /// `width` and `height` are the dimentions of the viewport.
    pub fn new(width: u32, height: u32) -> Overlay {
        return Overlay {
            internal: RefCell::new(OverlayBase::new(width, height))
        };
    }

    /// Renders all windows in the Overlay attached to the root window.
    ///
    /// In order to be rendered correctly, alpha blending must be enabled
    /// and either depth test must be disabled or the depth buffer cleared.
    #[inline]
    pub fn draw(&self) {
        self.internal.borrow_mut().update();
        self.internal.borrow().draw();
    }

    /// Creates a new window and returns a handle to it.
    ///
    /// The new window is not attached to the existing window hierarchy and as such
    /// will not be rendered until attached.
    pub fn make_window(&self, name: &str, data: WindowParams) -> Window {
        let index = self.internal.borrow_mut().make_window(name, data);

        Window {
            ovl: &self.internal,
            index: index,
        }
    }

    /// Get a handle to the root window.
    pub fn root(&self) -> Window {
        Window {
            ovl: &self.internal,
            index: 0,
        }
    }

    /// Get a handle to a window from an absolute path, i.e. a path that starts at the root window.
    ///
    /// Returns `Some(window)` if a window with the given path exists, `None` otherwise. <br>
    /// If `path` is `""` or `"."` the root window is returned. The leading `.` in `path` can be omitted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use engine::overlay::Overlay;
    /// // Overlay initialized somewhere else.
    /// let ovl: Overlay;
    /// # unsafe { ovl = std::mem::uninitialized(); }
    ///
    /// // The following lines are equivalent.
    /// let wnd1 = ovl.window(".foo.bar.quuz");
    /// let wnd2 = ovl.window("foo.bar.quuz");
    /// let wnd3 = ovl.root().child("foo.bar.quuz");
    /// ```
    pub fn window(&self, path: &str) -> Option<Window> {
        if path == "" || path == "." {
            return Some(self.root());
        }

        if path.starts_with('.') {
            let (_, path) = path.split_at(1);
            return self.root().child(path);
        }

        return self.root().child(path);
    }
}

/// Handle for an Overlay Window
pub struct Window<'a> {
    ovl: &'a RefCell<OverlayBase>,
    index: usize,
}

impl<'a> Window<'a> {
    /// Get a child by relative path
    pub fn child(&self, path: &str) -> Option<Window<'a>> {
        let ovl = self.ovl.borrow();
        let window = ovl.window(self.index);

        match path.find('.') {
            Some(seperator_pos) => {
                let (curr_name, rest_path) = path.split_at(seperator_pos);

                for &index in &window.children {
                    if ovl.window(index).name == curr_name {
                        return Window{ovl: self.ovl, index: index}.child(rest_path);
                    }
                }
                return None;
            },
            None => {
                for &index in &window.children {
                    if ovl.window(index).name == path {
                        return Some(Window{ovl: self.ovl, index: index});
                    }
                }
                return None;
            }
        }
    }

    /// Attaches a new child window.
    ///
    /// Attaching or detaching a window will cause a full update of the window hierarchy and
    /// the rendering buffers before the next draw call, for a total amount of extra work
    /// proportional to the number of windows in the overlay. Thus it is best to minimize
    /// the number of attaches/detaches or combine them in a single frame whenever possible.
    ///
    /// # Panics
    /// If `self` and `child` belong to different `Overlay` objects. <br>
    /// If `child` is already attached to another window. <br>
    /// If `self` already contains a child window with the same name as `child`.
    ///
    pub fn attach_child(&self, child: &Window) {
        assert!(self.ovl as *const RefCell<OverlayBase> == child.ovl as *const RefCell<OverlayBase>,
            ERR_WINDOW_DIFF_OVERLAYS);

        {
            let ovl = self.ovl.borrow();
            let child = ovl.window(child.index);

            if let Some(parent_index) = child.parent {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because it is already attached to window \"{}\"",
                    child.name,
                    self.full_path(),
                    Window{ovl: self.ovl, index: parent_index}.full_path()));
            }

            if !self.child(&child.name).is_none() {
                panic!(format!("Cannot attach window \"{}\" to \"{}\" because the second already has a child with the same name",
                    child.name,
                    self.full_path()));
            }
        }

        let mut ovl = self.ovl.borrow_mut();

        ovl.window_mut(self.index).children.push(child.index);
        ovl.window_mut(child.index).parent = Some(self.index);

        ovl.should_update.push(child.index);
        ovl.should_reindex = true;
    }

    /// Detaches a child window.
    ///
    /// Attaching or detaching a window will cause a full update of the window hierarchy and
    /// the rendering buffers before the next draw call, for a total amount of extra work
    /// proportional to the number of windows in the overlay. Thus it is best to minimize
    /// the number of attaches/detaches or combine them in a single frame whenever possible.
    ///
    /// If you want to temporary disable the rendering of a window it might be better to hide it instead.
    /// See the `show` method (not currently implemented :( ).
    ///
    /// # Panics
    /// If `self` and `child` belong to different `Overlay` objects. <br>
    /// If `child` is not attached to `self`.
    pub fn detach_child(&self, child: &Window) {
        assert!(self.ovl as *const RefCell<OverlayBase> == child.ovl as *const RefCell<OverlayBase>,
            ERR_WINDOW_DIFF_OVERLAYS);

        let child_parent = self.ovl.borrow().window(child.index).parent;

        match child_parent {
            Some(parent) if parent == self.index => {
                let mut ovl = self.ovl.borrow_mut();
                ovl.window_mut(self.index).children.retain(|&index| index != child.index);
                ovl.window_mut(child.index).parent = None;

                ovl.should_reindex = true;
            },
            _ => {
                panic!(format!("Attempting to detach window \"{}\" from non-parent window \"{}\"",
                    self.full_path(),
                    child.full_path()));
            }
        };
    }

    /// Accepts a closure which can be used to modify the window parameters
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
    pub fn modify<F> (&self, mod_fn: F)
        where F: Fn(&mut WindowParams)
    {
        let mut ovl = self.ovl.borrow_mut();

        mod_fn(&mut ovl.window_mut(self.index).creation_data);
        ovl.should_update.push(self.index);
    }

    /// Returns the full path to the window
    ///
    /// If the window is attached to the overlay root the returned string is an absolute path.
    /// Such path will always begin with `.`.
    ///
    /// If not this function backtracks for as long as a parent exists and returns a relative path.
    pub fn full_path(&self) -> String {
        let ovl = self.ovl.borrow();
        let window = ovl.window(self.index);

        match window.parent {
            Some(index) => {
                Window{ovl: self.ovl, index: index}.full_path() + "." + &window.name
            },
            None => {
                window.name.clone()
            }
        }
    }
}

const ERR_WINDOW_DIFF_OVERLAYS: &'static str = "Windows belong to different Overlay objects";
