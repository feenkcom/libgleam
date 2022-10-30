#![allow(non_snake_case)]

use std::os::raw::c_void;
use std::rc::Rc;

use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

pub mod gleam_gl;
pub mod gleam_gl_framebuffer;
pub mod gleam_gl_program;
pub mod gleam_gl_renderbuffer;
pub mod gleam_gl_texture;
pub mod gleam_gl_uniform;

include!(concat!(env!("OUT_DIR"), "/gl_enums.rs"));

#[no_mangle]
pub fn gleam_load_gl(
    callback: extern "C" fn(*mut ValueBox<StringBox>) -> *const c_void,
) -> *mut ValueBox<Rc<dyn gleam::gl::Gl>> {
    let gl = unsafe {
        gleam::gl::GlFns::load_with(|symbol| {
            let boxer_string = ValueBox::new(StringBox::from_string(symbol.to_string())).into_raw();
            let func_ptr = callback(boxer_string);
            boxer_string.release();
            func_ptr
        })
    };
    ValueBox::new(gl).into_raw()
}

#[no_mangle]
pub fn gleam_load_gles(
    callback: extern "C" fn(*mut ValueBox<StringBox>) -> *const c_void,
) -> *mut ValueBox<Rc<dyn gleam::gl::Gl>> {
    let gl = unsafe {
        gleam::gl::GlFns::load_with(|symbol| {
            let boxer_string = ValueBox::new(StringBox::from_string(symbol.to_string())).into_raw();
            let func_ptr = callback(boxer_string);
            boxer_string.release();
            func_ptr
        })
    };
    ValueBox::new(gl).into_raw()
}

#[no_mangle]
pub fn gleam_drop(gl: *mut ValueBox<Rc<dyn gleam::gl::Gl>>) {
    gl.release();
}

#[no_mangle]
pub fn gleam_test() -> bool {
    true
}
