extern crate cgmath;

use overlay::overlay::OverlayBase;

use self::cgmath::{Vector2, Vector3, Vector4, Vector};
use std::usize;
use std::fmt::{self, Debug, Formatter};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

#[derive(Debug)]
pub struct WindowBase {
    pub creation_data: BuildParams,
    pub index: usize,

    pub pos: Vec2,
    pub size: Vec2,
    pub shown: bool,

    pub children: Vec<usize>,
    pub parent: Option<usize>,

    pub vbo_beg: isize,
    pub vbo_end: isize,
}

#[derive(Clone)]
pub struct BuildParams {
    pub name: String,
    pub pos: Vector2<Vec3>,
    pub size: Vector2<Vec3>,
    pub color: [Vec4; 4],
    pub texcoord: [Vec2; 4],
}

impl Debug for BuildParams {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "BuildParams {{ name: {:?}, ... }}", self.name)
    }
}

impl WindowBase {
    pub fn new(data: BuildParams) -> WindowBase {
        return WindowBase {
            creation_data: data,
            index: usize::MAX,
            pos: Vec2::zero(),
            size: Vec2::zero(),
            shown: true,
            children: Vec::new(),
            parent: None,
            vbo_beg: -1,
            vbo_end: -1,
        };
    }

    pub fn child(&self, ovl: &OverlayBase, name: &str) -> Option<usize> {
        match name.find('.') {
            Some(seperator_pos) => {
                let (curr_name, next_name) = name.split_at(seperator_pos);

                for &index in &self.children {
                    let child = ovl.window_from_index(index).borrow();
                    if child.creation_data.name == curr_name {
                        return child.child(ovl, next_name);
                    }
                }

                return None;
            },
            None => {
                for &index in &self.children {
                    let child = ovl.window_from_index(index).borrow();
                    if child.creation_data.name == name {
                        return Some(index);
                    }
                }

                return None;
            }
        };
    }

    pub fn add_child(&mut self, ovl: &OverlayBase, child_index: usize) {
        let mut child = ovl.window_from_index(child_index).borrow_mut();

        // assert!(child.parent == None);
        if let Some(parent_index) = child.parent {
            panic!(format!("Cannot attach window \"{}\" to \"{}\" because it is already attached to window \"{}\"",
                child.creation_data.name,
                self.full_name(ovl),
                ovl.window_from_index(parent_index).borrow().full_name(ovl)));
        }

        // assert!(self.child(ovl, &child.creation_data.name) == None);
        // TODO: `it` in the error message is ambigious
        if self.child(ovl, &child.creation_data.name) != None {
            panic!(format!("Cannot attach window \"{}\" to \"{}\" because the second already has a child with the same name",
                child.creation_data.name,
                self.full_name(ovl)));
        }

        self.children.push(child_index);
        child.parent = Some(self.index);
    }

    fn full_name(&self, ovl: &OverlayBase) -> String {
        match self.parent {
            Some(index) => {
                let parent = ovl.window_from_index(index).borrow();
                return parent.full_name(ovl) + "." + &self.creation_data.name;
            },
            None => {
                return self.creation_data.name.clone();
            }
        };
    }
}
