use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;


    static VS_SRC: &'static str = "#version 300 es
    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }";

    static FS_SRC: &'static str = "#version 300 es
    precision mediump float;
    out vec4 out_color;

    void main() {
        out_color = vec4(gl_FragCoord.xyz / 300.0, 1.0);
    }";


    let vert_shader = compile_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, VS_SRC)?;
    let frag_shader = compile_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, FS_SRC)?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (vertices.len() / 3) as i32);

    Ok(())
}

pub fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32,
                      source: &str) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Could not compile shader.")))
    }
}

pub fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader,
                    frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}