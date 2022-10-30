use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn gleam_gen_textures(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    amount: GLsizei,
    array: *mut ValueBox<ArrayBox<GLuint>>,
) {
    gl.to_ref()
        .and_then(|gl| array.with_mut(|array| array.set_vector(gl.gen_textures(amount))))
        .log();
}

#[no_mangle]
fn gleam_bind_texture(gl: *mut ValueBox<Rc<dyn Gl>>, target: GLenum, texture: GLuint) {
    gl.with_ref(|gl| gl.bind_texture(target, texture)).log();
}

#[no_mangle]
fn gleam_active_texture(gl: *mut ValueBox<Rc<dyn Gl>>, texture: GLenum) {
    gl.with_ref(|gl| gl.active_texture(texture)).log();
}

#[no_mangle]
pub fn gleam_tex_parameter_i(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    pname: GLenum,
    param: GLint,
) {
    gl.with_ref(|gl| gl.tex_parameter_i(target, pname, param))
        .log();
}

#[no_mangle]
fn gleam_delete_textures(gl: *mut ValueBox<Rc<dyn Gl>>, array: *mut ValueBox<ArrayBox<GLuint>>) {
    gl.to_ref()
        .and_then(|gl| array.with_ref(|array| gl.delete_textures(array.to_slice())))
        .log();
}

#[no_mangle]
pub fn gleam_tex_image_2d(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    level: GLint,
    internal_format: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    ty: GLenum,
    data: *mut ValueBox<ArrayBox<u8>>,
) {
    gl.with_ref(|gl| {
        let data = data.to_ref();
        let data_slice = match &data {
            Ok(t) => Some(t.to_slice() as &[u8]),
            Err(_) => None,
        };

        gl.tex_image_2d(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            data_slice,
        )
    })
    .log();
}

#[no_mangle]
pub fn gleam_tex_sub_image_2d(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    ty: GLenum,
    data: *mut ValueBox<ArrayBox<u8>>,
) {
    gl.to_ref()
        .and_then(|gl| {
            data.with_ref(|data| {
                gl.tex_sub_image_2d(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    ty,
                    data.to_slice(),
                )
            })
        })
        .log();
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////  H E L P E R S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn gleam_gen_texture(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref(|gl| gl.gen_textures(1)[0]).or_log(0)
}

#[no_mangle]
pub fn gleam_delete_texture(gl: *mut ValueBox<Rc<dyn Gl>>, texture: GLuint) {
    gl.with_ref(|gl| gl.delete_textures(&[texture])).log();
}

#[no_mangle]
pub fn gleam_enable_texture_2d(gl: *mut ValueBox<Rc<dyn Gl>>) {
    gl.with_ref(|gl| gl.enable(TEXTURE_2D)).log();
}

#[no_mangle]
pub fn gleam_disable_texture_2d(gl: *mut ValueBox<Rc<dyn Gl>>) {
    gl.with_ref(|gl| gl.disable(TEXTURE_2D)).log();
}

#[no_mangle]
pub fn gleam_bind_texture_2d(gl: *mut ValueBox<Rc<dyn Gl>>, texture: GLuint) {
    gl.with_ref(|gl| gl.bind_texture(TEXTURE_2D, texture)).log();
}
