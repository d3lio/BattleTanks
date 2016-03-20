use core::renderable::Renderable;

/// A scene structure holding `Renderable` objects.
pub struct Scene<'a> {
    render_queue: Vec<&'a Renderable>
}

impl<'a> Scene<'a> {
    /// Create a new `Scene`.
    pub fn new() -> Scene<'a> {
        return Scene {
            render_queue: Vec::new()
        }
    }

    /// Add a `Renderable` object to the scene.
    pub fn add<R: Renderable>(&mut self, ent: &'a R) -> &mut Self {
        self.render_queue.push(ent);
        return self;
    }

    /// Draw all `Renderable` objects.
    pub fn draw(&self) {
        for ent in &self.render_queue {
            ent.draw();
        }
    }
}
