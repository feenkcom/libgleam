use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn gleam_gen_renderbuffers(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    amount: GLsizei,
    array: *mut ValueBox<ArrayBox<GLuint>>,
) {
    gl.to_ref()
        .and_then(|gl| array.with_mut(|array| array.set_vector(gl.gen_renderbuffers(amount))))
        .log();
}

#[no_mangle]
fn gleam_bind_renderbuffer(gl: *mut ValueBox<Rc<dyn Gl>>, target: GLenum, renderbuffer: GLuint) {
    gl.with_ref(|gl| gl.bind_renderbuffer(target, renderbuffer))
        .log();
}

#[no_mangle]
fn gleam_renderbuffer_storage(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    gl.with_ref(|gl| gl.renderbuffer_storage(target, internalformat, width, height))
        .log();
}

#[no_mangle]
fn gleam_delete_renderbuffers(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    array: *mut ValueBox<ArrayBox<GLuint>>,
) {
    gl.to_ref()
        .and_then(|gl| array.with_ref(|array| gl.delete_renderbuffers(array.to_slice())))
        .log();
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////  H E L P E R S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub fn gleam_gen_renderbuffer(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref(|gl| gl.gen_renderbuffers(1)[0]).or_log(0)
}

#[no_mangle]
pub fn gleam_delete_renderbuffer(gl: *mut ValueBox<Rc<dyn Gl>>, renderbuffer: GLuint) {
    gl.with_ref(|gl| gl.delete_renderbuffers(&[renderbuffer]))
        .log();
}
