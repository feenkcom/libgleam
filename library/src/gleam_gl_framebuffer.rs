use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn gleam_gen_framebuffers(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    amount: GLsizei,
    array: *mut ValueBox<ArrayBox<GLuint>>,
) {
    gl.with_ref(|gl| array.with_mut_ok(|array| array.set_vector(gl.gen_framebuffers(amount))))
        .log();
}

#[no_mangle]
fn gleam_bind_framebuffer(gl: *mut ValueBox<Rc<dyn Gl>>, target: GLenum, framebuffer: GLuint) {
    gl.with_ref_ok(|gl| gl.bind_framebuffer(target, framebuffer))
        .log();
}

#[no_mangle]
fn gleam_framebuffer_texture_2d(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    gl.with_ref_ok(|gl| gl.framebuffer_texture_2d(target, attachment, textarget, texture, level))
        .log();
}

#[no_mangle]
fn gleam_framebuffer_renderbuffer(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    gl.with_ref_ok(|gl| {
        gl.framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer)
    })
    .log();
}

#[no_mangle]
pub fn gleam_check_frame_buffer_status(gl: *mut ValueBox<Rc<dyn Gl>>, target: GLenum) -> GLenum {
    gl.with_ref_ok(|gl| gl.check_frame_buffer_status(target))
        .or_log(0)
}

#[no_mangle]
fn gleam_delete_framebuffers(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    array: *mut ValueBox<ArrayBox<GLuint>>,
) {
    gl.with_ref(|gl| array.with_ref_ok(|array| gl.delete_framebuffers(array.to_slice())))
        .log();
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////  H E L P E R S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub fn gleam_gen_framebuffer(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref_ok(|gl| gl.gen_framebuffers(1)[0]).or_log(0)
}

#[no_mangle]
pub fn gleam_delete_framebuffer(gl: *mut ValueBox<Rc<dyn Gl>>, framebuffer: GLuint) {
    gl.with_ref_ok(|gl| gl.delete_framebuffers(&[framebuffer]))
        .log();
}
