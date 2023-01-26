use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
fn gleam_uniform_2f(gl: *mut ValueBox<Rc<dyn Gl>>, location: GLint, v0: GLfloat, v1: GLfloat) {
    gl.with_ref_ok(|gl| gl.uniform_2f(location, v0, v1)).log();
}

#[no_mangle]
fn gleam_uniform_matrix_4fv(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    location: GLint,
    transpose: bool,
    data: *mut ValueBox<ArrayBox<f32>>,
) {
    gl.with_ref(|gl| {
        data.with_ref_ok(|data| gl.uniform_matrix_4fv(location, transpose, data.to_slice()))
    })
    .log();
}
