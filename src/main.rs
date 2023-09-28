mod triangles;
use triangles::triangles as triangle;
mod utils;
mod wireframe;
fn main() {
    // wireframe::draw("model.obj");
    triangle::draw();
}
