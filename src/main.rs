mod triangles;
use triangles::triangles as triangle;
mod utils;
mod wireframe;

extern crate itertools;
fn main() {
    // wireframe::draw("model.obj");
    triangle::draw();
}
