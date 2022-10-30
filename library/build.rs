extern crate gl_generator;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&dest).join("gl_enums.rs")).unwrap();

    let gl_extensions = [
//        "GL_APPLE_client_storage",
//        "GL_APPLE_fence",
//        "GL_APPLE_texture_range",
//        "GL_APPLE_vertex_array_object",
//        "GL_ARB_blend_func_extended",
//        "GL_ARB_copy_image",
//        "GL_ARB_get_program_binary",
//        "GL_ARB_invalidate_subdata",
//        "GL_ARB_texture_rectangle",
//        "GL_ARB_texture_storage",
//        "GL_EXT_debug_marker",
//        "GL_EXT_texture_filter_anisotropic",
//        "GL_KHR_debug",
//        "GL_KHR_blend_equation_advanced",
//        "GL_KHR_blend_equation_advanced_coherent",
    ];
    let gl_reg = Registry::new(
        Api::Gl,
        (3, 3),
        Profile::Compatibility,
        Fallbacks::All,
        gl_extensions,
    );

    let _ = file_gl.write_all(
        "use gleam::gl::*;

"
        .as_bytes(),
    );

    let _ = file_gl.write_all(
        format!(
            "static GL_ENUMS: [(&str, &str, &str); {}] = [",
            gl_reg.enums.len()
        )
        .as_bytes(),
    );

    for each_enum in gl_reg.enums.iter() {
        let _ = file_gl.write_all(
            format!(
                "({:?}, {:?}, {:?}), ",
                each_enum.ident, each_enum.value, each_enum.ty
            )
            .as_bytes(),
        );
    }

    let _ = file_gl.write_all(
        "];\
"
        .as_bytes(),
    );

    let _ = file_gl.write_all("
#[no_mangle]
pub fn gleam_enum_get_at_gl(enum_ident: *mut ValueBox<StringBox>, enum_value: *mut ValueBox<StringBox>, enum_type: *mut ValueBox<StringBox>, index: usize) {
    let each_enum = GL_ENUMS[index];
    enum_ident.with_mut(|enum_ident| enum_ident.set_string(String::from(each_enum.0))).log();
    enum_value.with_mut(|enum_value| enum_value.set_string(String::from(each_enum.1))).log();
    enum_type.with_mut(|enum_type| enum_type.set_string(String::from(each_enum.2))).log();
}
".as_bytes());

    let _ = file_gl.write_all(
        "
#[no_mangle]
pub fn gleam_enum_get_amount_gl() -> usize {
    GL_ENUMS.len()
}
"
        .as_bytes(),
    );

    for each_enum in gl_reg.enums.iter() {
        let _ = file_gl.write_all(
            format!(
                "
#[no_mangle]
pub fn gleam_enum_gl_{}() -> {} {{ {} }}
",
                each_enum.ident.to_lowercase(),
                each_enum.ty,
                each_enum.ident
            )
            .as_bytes(),
        );

        let _ = file_gl.write_all(
            format!(
                "
#[no_mangle]
pub fn gleam_enum_type_gl_{}(string: *mut ValueBox<StringBox>) {{
    string.with_mut(|string| string.set_string(String::from({:?}))).log();
}}
",
                each_enum.ident.to_lowercase(),
                each_enum.ty
            )
            .as_bytes(),
        );
    }
}
