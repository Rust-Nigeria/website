use web_sys::{js_sys::Float32Array, WebGlProgram, WebGlRenderingContext as GL, WebGlShader};

pub fn create_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type).unwrap();
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let err = Err(context.get_shader_info_log(&shader).unwrap());
        context.delete_shader(Some(&shader));
        err
    }
}

pub fn create_program(
    context: &GL,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context.create_program().unwrap();
    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);

    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let err = Err(context.get_program_info_log(&program).unwrap());
        context.delete_program(Some(&program));
        err
    }
}

pub fn set_quad(context: &GL, width: f32, height: f32) {
    let x1 = 0.0;
    let x2 = width;
    let y1 = 0.0;
    let y2 = height;

    let arr = Float32Array::new_with_length(12);

    arr.copy_from(&[x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]);

    context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &arr, GL::STATIC_DRAW);
}

#[rustfmt::skip]
pub fn get_canvas_to_clipspace_projection_matrix(width: f32, height: f32) -> [f32; 9] {
    return [
        2.0 / width,  0.0,            0.0,
        0.0,         (-2.0 / height), 0.0,
       -1.0,          1.0,            1.0
    ];
}

#[rustfmt::skip]
pub fn get_texture_to_clipspace_projection_matrix(width: f32, height: f32) -> [f32; 9] {
    return [
        1.0 / width,  0.0,            0.0,
        0.0,         (-1.0 / height), 0.0,
        0.0,          1.0,            1.0
    ];
}
