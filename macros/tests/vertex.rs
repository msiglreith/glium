#![feature(plugin)]
#![plugin(glium_macros)]

extern crate glium;

#[test]
fn verify_shader() {
    #[vertex_format]
    struct Vertex {
        position: [f32; 2]
    }

    //assert_eq!(<Vertex as >)
}
