use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
fn gleam_uniform_2f(_ptr_gl: *mut ValueBox<Rc<dyn Gl>>, location: GLint, v0: GLfloat, v1: GLfloat) {
    _ptr_gl.with_not_null(|gl| gl.uniform_2f(location, v0, v1));
}

#[no_mangle]
fn gleam_uniform_matrix_4fv(
    _ptr_gl: *mut ValueBox<Rc<dyn Gl>>,
    location: GLint,
    transpose: bool,
    _value_ptr: *mut ValueBox<ArrayBox<f32>>,
) {
    _ptr_gl.with_not_null(|gl| {
        _value_ptr
            .with_not_null(|values| gl.uniform_matrix_4fv(location, transpose, values.to_slice()))
    });
}
