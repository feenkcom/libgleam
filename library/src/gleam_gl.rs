use std::rc::Rc;

use array_box::ArrayBox;
use gleam::gl::*;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn gleam_enable(gl: *mut ValueBox<Rc<dyn Gl>>, cap: GLenum) {
    gl.with_ref(|gl| gl.enable(cap)).log();
}

#[no_mangle]
pub fn gleam_disable(gl: *mut ValueBox<Rc<dyn Gl>>, cap: GLenum) {
    gl.with_ref(|gl| gl.disable(cap)).log();
}

#[no_mangle]
pub fn gleam_clear_color(gl: *mut ValueBox<Rc<dyn Gl>>, r: f32, g: f32, b: f32, a: f32) {
    gl.with_ref(|gl| gl.clear_color(r, g, b, a)).log();
}

#[no_mangle]
pub fn gleam_clear_depth(gl: *mut ValueBox<Rc<dyn Gl>>, depth: f64) {
    gl.with_ref(|gl| gl.clear_depth(depth)).log();
}

#[no_mangle]
pub fn gleam_clear(gl: *mut ValueBox<Rc<dyn Gl>>, buffer_mask: GLbitfield) {
    gl.with_ref(|gl| gl.clear(buffer_mask)).log();
}

#[no_mangle]
pub fn gleam_get_error(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLenum {
    gl.with_ref(|gl| gl.get_error()).or_log(0)
}

#[no_mangle]
pub fn gleam_get_string(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    which: GLenum,
    string: *mut ValueBox<StringBox>,
) {
    gl.to_ref()
        .and_then(|gl| string.with_mut(|string| string.set_string(gl.get_string(which))))
        .log();
}

#[no_mangle]
pub fn gleam_read_pixels(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    pixel_type: GLenum,
    data: *mut ValueBox<ArrayBox<u8>>,
) {
    gl.to_ref()
        .and_then(|gl| {
            data.with_mut(|data| {
                let pixels = gl.read_pixels(x, y, width, height, format, pixel_type);
                data.set_vector(pixels)
            })
        })
        .log();
}

#[no_mangle]
pub fn gleam_get_tex_image_into_buffer(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    target: GLenum,
    level: GLint,
    format: GLenum,
    ty: GLenum,
    data: *mut ValueBox<ArrayBox<u8>>,
) {
    gl.to_ref()
        .and_then(|gl| {
            data.with_ref(|data| {
                gl.get_tex_image_into_buffer(target, level, format, ty, data.to_slice())
            })
        })
        .log();
}

#[no_mangle]
pub fn gleam_get_shader_version(gl: *mut ValueBox<Rc<dyn Gl>>) -> u32 {
    gl.with_ref(|gl| {
        let version = gl.get_string(SHADING_LANGUAGE_VERSION);

        let split = version.split_whitespace();
        let vec: Vec<&str> = split.collect();

        let number = vec[0].parse::<f32>();
        (number.unwrap() * 100.0) as u32
    })
    .or_log(0)
}

#[no_mangle]
pub fn gleam_create_vertex_shader(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref(|gl| gl.create_shader(VERTEX_SHADER)).or_log(0)
}

#[no_mangle]
pub fn gleam_create_fragment_shader(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref(|gl| gl.create_shader(FRAGMENT_SHADER))
        .or_log(0)
}

#[no_mangle]
pub fn gleam_compile_shader(gl: *mut ValueBox<Rc<dyn Gl>>, shader: GLuint) {
    gl.with_ref(|gl| {
        gl.compile_shader(shader);
        let log = gl.get_shader_info_log(shader);
        if !log.is_empty() {
            println!("shader log: {}", log);
        }
    })
    .log();
}

#[no_mangle]
pub fn gleam_shader_source(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    shader: GLuint,
    source: *mut ValueBox<StringBox>,
) {
    gl.to_ref()
        .and_then(|gl| {
            source.with_ref(|source| {
                let source_string = source.to_string();
                gl.shader_source(shader, &[source_string.as_bytes()]);
            })
        })
        .log();
}

#[no_mangle]
pub fn gleam_attach_shader(gl: *mut ValueBox<Rc<dyn Gl>>, program: GLuint, shader: GLuint) {
    gl.with_ref(|gl| gl.attach_shader(program, shader)).log();
}

#[no_mangle]
pub fn gleam_viewport(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    gl.with_ref(|gl| gl.viewport(x, y, width, height)).log();
}

#[no_mangle]
pub fn gleam_create_buffer(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref(|gl| gl.gen_buffers(1)[0]).or_log(0)
}

#[no_mangle]
pub fn gleam_bind_array_buffer(gl: *mut ValueBox<Rc<dyn Gl>>, buffer: GLuint) {
    gl.with_ref(|gl| gl.bind_buffer(ARRAY_BUFFER, buffer)).log();
}

#[no_mangle]
pub fn gleam_array_buffer_data_static_draw(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    array: *const f32,
    length: u32,
) {
    gl.with_ref(|gl| {
        let data: &[f32] = unsafe { std::slice::from_raw_parts(array, length as usize) };

        gl.buffer_data_untyped(
            ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            data.as_ptr() as *const GLvoid,
            STATIC_DRAW,
        );
    })
    .log();
}

#[no_mangle]
pub fn gleam_get_attribute_location(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    program: GLuint,
    location: *mut ValueBox<StringBox>,
) -> i32 {
    gl.to_ref()
        .and_then(|gl| {
            location
                .with_ref(|location| gl.get_attrib_location(program, location.to_string().as_ref()))
        })
        .or_log(0)
}

#[no_mangle]
pub fn gleam_get_uniform_location(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    program: GLuint,
    location: *mut ValueBox<StringBox>,
) -> i32 {
    gl.to_ref()
        .and_then(|gl| {
            location.with_ref(|location| {
                gl.get_uniform_location(program, location.to_string().as_ref())
            })
        })
        .or_log(0)
}

#[no_mangle]
pub fn gleam_gen_vertex_array(gl: *mut ValueBox<Rc<dyn Gl>>) -> GLuint {
    gl.with_ref(|gl| gl.gen_vertex_arrays(1)[0]).or_log(0)
}

#[no_mangle]
pub fn gleam_bind_vertex_array(gl: *mut ValueBox<Rc<dyn Gl>>, vao: GLuint) {
    gl.with_ref(|gl| gl.bind_vertex_array(vao)).log();
}

#[no_mangle]
pub fn gleam_enable_vertex_attrib_array(gl: *mut ValueBox<Rc<dyn Gl>>, index: GLuint) {
    gl.with_ref(|gl| gl.enable_vertex_attrib_array(index)).log();
}

#[no_mangle]
pub fn gleam_vertex_attrib_pointer(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: bool,
    stride: GLsizei,
    offset: GLuint,
) {
    gl.with_ref(|gl| gl.vertex_attrib_pointer(index, size, type_, normalized, stride, offset))
        .log();
}

#[no_mangle]
pub fn gleam_draw_arrays(
    gl: *mut ValueBox<Rc<dyn Gl>>,
    mode: GLenum,
    first: GLint,
    count: GLsizei,
) {
    gl.with_ref(|gl| gl.draw_arrays(mode, first, count)).log();
}

#[no_mangle]
pub fn gleam_get_integer(gl: *mut ValueBox<Rc<dyn Gl>>, name: GLenum) -> GLint {
    gl.with_ref(|gl| {
        let mut result: [GLint; 1] = [0; 1];
        unsafe { gl.get_integer_v(name, &mut result) };
        result[0]
    })
    .or_log(0)
}
