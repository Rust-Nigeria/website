use leptos::{
    html,
    prelude::{Get, NodeRef},
};

use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext as GL, WebGlTexture};

use crate::utils::{
    load_image::ImageFuture,
    webgl::{create_program, create_shader},
};

use super::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

fn create_texture(gl: &GL) -> WebGlTexture {
    let texture = gl.create_texture().unwrap();
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

    texture
}

pub struct Jigsaw {
    gl: GL,
}

impl Jigsaw {
    pub async fn new(canvas_ref: NodeRef<html::Canvas>) -> Self {
        let canvas = canvas_ref.get().unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();

        let vertex_shader = create_shader(&gl, GL::VERTEX_SHADER, VERTEX_SHADER).unwrap();
        let fragment_shader = create_shader(&gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER).unwrap();

        let program = create_program(&gl, &vertex_shader, &fragment_shader).unwrap();

        let canvas_projection_matrix_uniform_loc =
            gl.get_uniform_location(&program, "u_canvasProjectionMatrix");

        let texture_projection_matrix_uniform_loc =
            gl.get_uniform_location(&program, "u_textureProjectionMatrix");

        let main_image_uniform_loc = gl.get_uniform_location(&program, "u_mainImage");

        let mask_image_uniform_loc = gl.get_uniform_location(&program, "u_maskImage");

        let vertex_position_attribute_location = gl.get_attrib_location(&program, "a_position");

        let texture_position_attribute_location = gl.get_attrib_location(&program, "a_texCoord");

        let vertex_position_buffer = gl.create_buffer();

        let texture_position_buffer = gl.create_buffer();

        let main_image = ImageFuture::new("/images/puzzle.jpg").await;

        let main_image_mask = ImageFuture::new("/images/puzzle-mask.png").await;

        gl.use_program(Some(&program));

        Jigsaw { gl }
    }
}
