use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn gleam_create_program(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref_ok(|gl| gl.create_program()).or_log(0)
}

#[no_mangle]
pub fn gleam_use_program(gl: *mut ValueBox<Rc<dyn Gl>>, program: GLuint) {
    gl.with_ref_ok(|gl| gl.use_program(program)).log();
}

#[no_mangle]
pub fn gleam_link_program(gl: *mut ValueBox<Rc<dyn Gl>>, program: GLuint) {
    gl.with_ref_ok(|gl| gl.link_program(program)).log();
}

#[no_mangle]
pub fn gleam_delete_program(gl: *mut ValueBox<Rc<dyn Gl>>, program: GLuint) {
    gl.with_ref_ok(|gl| gl.delete_program(program)).log();
}

#[no_mangle]
pub fn gleam_get_program_iv(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    program: GLuint,
    pname: GLenum,
    array: *mut ValueBox<ArrayBox<GLint>>,
) {
    gl.with_ref(|gl| {
        array.with_ref_ok(|array| unsafe { gl.get_program_iv(program, pname, array.to_slice()) })
    })
    .log();
}

#[no_mangle]
pub fn gleam_get_program_info_log(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    program: GLuint,
    string: *mut ValueBox<StringBox>,
) {
    gl.with_ref(|gl| {
        string.with_mut_ok(|string| string.set_string(gl.get_program_info_log(program)))
    })
    .log();
}

#[no_mangle]
pub fn gleam_validate_program(gl: *mut ValueBox<Rc<dyn Gl>>, program: GLuint) {
    gl.with_ref_ok(|gl| gl.validate_program(program)).log();
}
