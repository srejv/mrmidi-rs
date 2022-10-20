use super::commands::*;

// Value
fn v(value: &str) -> CodeNode {
    (CommandType::Value, value.to_owned())
}

// Return
fn r() -> CodeNode {
    (CommandType::Return, "".to_owned())
}

// Set variable
fn sv(value: &str) -> CodeNode {
    (CommandType::SetVariable, value.to_owned())
}

// Preprocessor
fn pp(command: &str, value: &str) -> CodeNode {
    (CommandType::PreProcessor, format!("{} {}", command, value))
}

// Parenthesis
fn ps() -> CodeNode {
    (CommandType::Parenthesis, "".to_owned())
}

// Begin if
fn bi() -> CodeNode {
    (CommandType::BeginIf, "".to_owned())
}

// End if
fn ei() -> CodeNode {
    (CommandType::EndIf, "".to_owned())
}

// CallFunction
fn cf(value: &str) -> CodeNode {
    (CommandType::CallFunction, value.to_owned())
}

// Precision
fn pc(value: &str, return_type: &str) -> CodeNode {
    (
        CommandType::Value,
        format!("precision {} {}", value, return_type),
    )
}

fn dv(value: &str) -> CodeNode {
    (CommandType::DeclareVariable, "".to_owned())
}

fn c(p: &mut TreeProgram, parent: usize, func: CodeNode, args: &[CodeNode]) {
    let f = p.tree.add_child(func, parent);
    for arg in args {
        p.tree.add_child(arg.clone(), f);
    }
}

// Add child
/*
fn ac(p: &mut TreeProgram, func: CodeNode, parent: usize) -> usize {
    p.tree.add_child(func, parent)
}
*/

// Attribute
fn atrib(return_type: &str, value: &str) -> CodeNode {
    (
        CommandType::Value,
        format!("attribute {} {}", return_type, value),
    )
}

// Varying
fn vary(return_type: &str, value: &str) -> CodeNode {
    (
        CommandType::Value,
        format!("varying {} {}", return_type, value),
    )
}

fn uni(return_type: &str, value: &str) -> CodeNode {
    (
        CommandType::Value,
        format!("uniform {} {}", return_type, value),
    )
}

fn m() -> CodeNode {
    (CommandType::Multiply, "".to_owned())
}

fn d() -> CodeNode {
    (CommandType::Divide, "".to_owned())
}

fn a() -> CodeNode {
    (CommandType::Add, "".to_owned())
}

fn s() -> CodeNode {
    (CommandType::Subtract, "".to_owned())
}

fn o() -> CodeNode {
    (CommandType::Or, "".to_owned())
}

fn an() -> CodeNode {
    (CommandType::And, "".to_owned())
}

fn ef() -> CodeNode {
    (CommandType::EndFunction, "".to_owned())
}

fn bf() -> CodeNode {
    (CommandType::BeginFunction, "".to_owned())
}

fn lt() -> CodeNode {
    (CommandType::LessThan, "".to_owned())
}

fn mt() -> CodeNode {
    (CommandType::MoreThan, "".to_owned())
}

pub fn test_program() {
    println!("");
    println!(":: First test ::");
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("Rootnode", "Don't print."));
    let set_variable = p.tree.add_child(sv("gl_FragCoord"), root);
    let values = [v("1.0"), v("1.0"), v("0.0"), v("1.0")];
    c(&mut p, set_variable, cf("vec4"), &values);
    p.print();

    println!("");
    println!(":: Default Vertex ::");
    println!("::::::::::::::::::::");
    let mut p = create_default_vertex_shader();
    p.print();

    println!("");
    println!(":: Default Fragment ::");
    println!("::::::::::::::::::::::");
    let mut p = create_default_fragment_shader();
    p.print();

    println!("");
    println!(":: CRT Vertex ::");
    println!("::::::::::::::::::::");
    let mut p = create_crt_vertex_shader();
    p.print();

    println!("");
    println!(":: CRT Fragment ::");
    println!("::::::::::::::::::::");
    let mut p = create_crt_fragment_shader();
    p.print();
}

pub fn create_crt_fragment_shader() -> TreeProgram {
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("root", "don't print this"));
    p.ac(pp("version", "100"), root);
    p.ac(pc("lowp", "float"), root);

    p.ac(vary("vec4", "color"), root);
    p.ac(vary("vec2", "uv"), root);

    p.ac(uni("float", "iTime"), root);

    p.ac(uni("sampler2D", "Texture"), root);

    p.ac(uni("sampler2D", "iChannel0"), root);
    p.ac(uni("sampler2D", "iChannel1"), root);

    p.ac(uni("vec3", "iResolution"), root);

    let crt_curve_uv = p.ac(bf(), root);
    p.ac(v("vec2"), crt_curve_uv);
    p.ac(v("CRTCurveUV"), crt_curve_uv);
    p.ac(v("vec2 uv"), crt_curve_uv);

    let set_uv = p.ac(sv("uv"), root);
    let multiply = p.ac(m(), set_uv);
    p.ac(v("uv"), multiply);
    let subtract = p.tree.add_child(s(), multiply);
    p.ac(v("2.0"), subtract);
    p.ac(v("1.0"), subtract);

    let set_offset = p.ac(sv("vec2 offset"), root);
    let divide = p.ac(d(), set_offset);
    c(&mut p, divide, cf("abs"), &[v("uv.yx")]);
    c(&mut p, divide, cf("vec2"), &[v("6.0"), v("4.0")]);

    let set_uv = p.ac(sv("uv"), root);
    let add = p.ac(a(), set_uv);
    p.ac(v("uv"), add);
    let multiply = p.tree.add_child(m(), add);
    p.ac(v("uv"), multiply);
    p.ac(v("offset"), multiply);
    p.ac(v("offset"), multiply);

    let set_uv = p.ac(sv("uv"), root);
    let multiply = p.ac(m(), set_uv);
    p.ac(v("uv"), multiply);
    let add = p.ac(a(), multiply);
    p.ac(v("0.5"), add);
    p.ac(v("0.5"), add);

    let return_value = p.ac(r(), root);
    p.ac(v("uv"), return_value);
    p.ac(ef(), root);

    let draw_vignette = p.ac(bf(), root);
    p.ac(v("void"), draw_vignette);
    p.ac(v("DrawVignette"), draw_vignette);
    p.ac(v("inout vec3 color"), draw_vignette);
    p.ac(v("vec2 uv"), draw_vignette);

    let set_vignette = p.ac(sv("float vignette"), root);
    let multiply = p.ac(m(), set_vignette);
    p.ac(v("uv.x"), multiply);
    p.ac(v("uv.y"), multiply);

    let parenthesis = p.ac(ps(), multiply);
    let subtract = p.ac(s(), parenthesis);
    p.ac(v("1.0"), subtract);
    p.ac(v("uv.x"), subtract);

    let parenthesis = p.ac(ps(), multiply);
    let subtract = p.ac(s(), parenthesis);
    p.ac(v("1.0"), subtract);
    p.ac(v("uv.y"), subtract);

    let set_vignette = p.ac(sv("vignette"), root);
    let clamp_call = p.ac(cf("clamp"), set_vignette);
    let pow_call = p.ac(cf("pow"), clamp_call);
    let multiply = p.ac(m(), pow_call);
    p.ac(v("16.0"), multiply);
    p.ac(v("vignette"), multiply);
    p.ac(v("0.3"), pow_call);

    p.ac(v("0.0"), clamp_call);
    p.ac(v("1.0"), clamp_call);

    let set_color = p.ac(sv("color"), root);
    let multiply = p.ac(m(), set_color);
    p.ac(v("color"), multiply);
    p.ac(v("vignette"), multiply);

    p.ac(ef(), root);

    // void DrawScanline(inout vec3 color, vec2 uv)
    let scanline = p.ac(bf(), root);
    p.ac(v("void"), scanline);
    p.ac(v("DrawScanline"), scanline);
    p.ac(v("inout vec3 color"), scanline);
    p.ac(v("vec2 uv"), scanline);

    let set_scanline = p.ac(sv("float scanline"), root);
    let clamp_call = p.ac(cf("clamp"), set_scanline);
    let add = p.ac(a(), clamp_call);
    p.ac(v("0.0"), clamp_call);
    p.ac(v("1.0"), clamp_call);

    p.ac(v("0.95"), add);
    let multiply = p.ac(m(), add);
    p.ac(v("0.5"), multiply);
    let cos_call = p.ac(cf("cos"), multiply);
    let multiply = p.ac(m(), cos_call);
    p.ac(v("3.14"), multiply);
    let more = p.ac(ps(), multiply);
    p.ac(v("240.0"), multiply);
    p.ac(v("10.0"), multiply);

    let add = p.ac(a(), more);
    p.ac(v("uv.y"), add);
    let multiply = p.ac(m(), add);
    p.ac(v("0.008"), multiply);
    p.ac(v("iTime"), multiply);

    let set_grille = p.ac(sv("float grille"), root);
    let add = p.ac(a(), set_grille);
    p.ac(v("0.85"), add);
    let multiply = p.ac(m(), add);
    p.ac(v("1.5"), multiply);
    let clamp_call = p.ac(cf("clamp"), multiply);
    let cos_call = p.ac(cf("cos"), clamp_call);
    let multiply = p.ac(m(), cos_call);
    p.ac(v("3.14"), multiply);
    p.ac(v("uv.x"), multiply);
    p.ac(v("640.0"), multiply);
    p.ac(v("1.0"), multiply);

    p.ac(v("0.0"), clamp_call);
    p.ac(v("1.0"), clamp_call);

    let set_color = p.ac(sv("color"), root);
    let multiply = p.ac(m(), set_color);
    p.ac(v("color"), multiply);
    p.ac(v("scanline"), multiply);
    p.ac(v("graille"), multiply);
    p.ac(v("1.2"), multiply);
    p.ac(ef(), root);

    // Main
    let main = p.ac(bf(), root);
    p.ac(v("void"), main);
    p.ac(v("main"), main);

    let set_crt_uv = p.ac(sv("vec2 crtUV"), root);
    let crt_curve_call = p.ac(cf("CRTCurveUV"), set_crt_uv);
    p.ac(v("uv"), crt_curve_call);

    let set_mehuv = p.ac(sv("vec2 mehuv"), root);
    let vec2_call = p.ac(cf("vec2"), set_mehuv);
    let subtract = p.ac(s(), vec2_call);
    p.ac(v("1.0"), subtract);
    p.ac(v("crtUV.x"), subtract);
    let subtract = p.ac(s(), vec2_call);
    p.ac(v("1.0"), subtract);
    p.ac(v("crtUV.y"), subtract);

    let set_channel0 = p.ac(sv("vec3 channel0"), root);
    let tex_call = p.ac(cf("texture2D"), set_channel0);
    p.ac(v("iChannel0"), tex_call);
    p.ac(v("mehuv"), tex_call);

    let set_channel1 = p.ac(sv("vec3 channel1"), root);
    let tex_call = p.ac(cf("texture2D"), set_channel1);
    p.ac(v("iChannel1"), tex_call);
    p.ac(v("mehuv"), tex_call);

    let set_res = p.ac(sv("vec3 res"), root);
    let mix_call = p.ac(cf("mix"), set_res);
    p.ac(v("iChannel0"), mix_call);
    p.ac(v("iChannel1"), mix_call);
    let multiply = p.ac(m(), mix_call);
    let sin_call = p.ac(cf("sin"), multiply);
    p.ac(v("iTime"), sin_call);
    let add = p.ac(a(), multiply);
    p.ac(v("0.5"), add);
    p.ac(v("0.5"), add);

    let if_call = p.ac(bi(), root);
    let or_check = p.ac(o(), if_call);
    let less_check = p.ac(lt(), or_check);
    p.ac(v("crtUV.x"), less_check);
    p.ac(v("0.0"), less_check);

    let or_check = p.ac(o(), if_call);
    let more_check = p.ac(mt(), or_check);
    p.ac(v("crtUV.x"), more_check);
    p.ac(v("1.0"), more_check);

    let or_check = p.ac(o(), if_call);
    let less_check = p.ac(lt(), or_check);
    p.ac(v("crtUV.y"), less_check);
    p.ac(v("0.0"), less_check);

    let or_check = p.ac(o(), if_call);
    let more_check = p.ac(mt(), or_check);
    p.ac(v("crtUV.y"), more_check);
    p.ac(v("1.0"), more_check);

    let set_res = p.ac(sv("res"), root);
    let vec3_call = p.ac(cf("vec3"), set_res);
    p.ac(v("0.0"), vec3_call);
    p.ac(v("0.0"), vec3_call);
    p.ac(v("0.0"), vec3_call);

    p.ac(ei(), root);

    let draw_vignette_call = p.ac(cf("DrawVignette"), root);
    p.ac(v("res"), draw_vignette_call);
    p.ac(v("crtUV"), draw_vignette_call);

    let draw_scanline_call = p.ac(cf("DrawScanline"), root);
    p.ac(v("res"), draw_scanline_call);
    p.ac(v("uv"), draw_scanline_call);

    let set_output_color = p.ac(sv("gl_FragColor"), root);
    let vec4_call = p.ac(cf("vec4"), set_output_color);
    p.ac(v("res"), vec4_call);
    p.ac(v("1.0"), vec4_call);

    p.tree.add_child(ef(), root);
    p
}

pub fn create_default_fragment_shader() -> TreeProgram {
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("root", "don't print me"));
    p.ac(pp("version", "100"), root);
    p.ac(pc("lowp", "float"), root);

    // macroquad uniforms
    p.ac(vary("vec2", "uv"), root);
    p.ac(uni("sampler2D", "Texture"), root);

    // mrmidi
    p.ac(uni("float", "iTime"), root);
    p.ac(uni("float", "iTimeDelta"), root);
    p.ac(uni("int", "iFrame"), root);

    let main = p.tree.add_child(bf(), root);
    p.ac(v("void"), main);
    p.ac(v("main"), main);

    let set_color = p.ac(sv("gl_FragCoord"), root);
    let call_sample_texture = p.ac(cf("texture2D"), set_color);
    p.ac(v("Texture"), call_sample_texture);
    p.ac(v("uv"), call_sample_texture);

    p.ac(ef(), root);
    p
}

pub fn create_default_vertex_shader() -> TreeProgram {
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("Rootnode", "Don't print."));

    p.ac(pp("version", "100"), root);

    p.ac(pc("lowp", "float"), root);

    p.ac(atrib("vec3", "position"), root);
    p.ac(atrib("vec2", "texcoord"), root);

    // // macroquad uniforms
    p.ac(vary("vec2", "uv"), root);

    p.ac(uni("sampler2D", "Texture"), root);
    p.ac(uni("mat4", "Model"), root);
    p.ac(uni("mat4", "Projection"), root);

    // mrmidi
    p.ac(uni("float", "iTime"), root);
    p.ac(uni("float", "iTimeDelta"), root);
    p.ac(uni("int", "iFrame"), root);

    let main = p.tree.add_child(bf(), root);
    p.ac(v("void"), main);
    p.ac(v("main"), main);

    let set_position = p.ac(sv("gl_Position"), root);
    let multiply = p.ac(m(), set_position);
    p.ac(v("Projection"), multiply);
    p.ac(v("Model"), multiply);
    let call_conversion = p.ac(cf("vec4"), multiply);
    p.ac(v("position"), call_conversion);
    p.ac(v("1"), call_conversion);

    p.ac(ef(), root);
    p
}

pub fn create_crt_vertex_shader() -> TreeProgram {
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("root", "don't print this"));
    p.ac(pp("version", "100"), root);
    p.ac(pc("lowp", "float"), root);

    p.ac(atrib("vec3", "position"), root);
    p.ac(atrib("vec2", "texcoord"), root);
    p.ac(atrib("vec4", "color0"), root);

    // macroquad uniforms
    p.ac(vary("vec2", "uv"), root);
    p.ac(vary("vec4", "color"), root);

    p.ac(uni("mat4", "Model"), root);
    p.ac(uni("mat4", "Projection"), root);

    let main = p.ac(bf(), root);
    p.ac(v("void"), main);
    p.ac(v("main"), main);

    let set_position = p.ac(sv("gl_Position"), root);
    let multiply = p.ac(m(), set_position);
    p.ac(v("Projection"), multiply);
    p.ac(v("Model"), multiply);
    let position_func = p.ac(cf("vec4"), multiply);
    p.ac(v("position"), position_func);
    p.ac(v("1"), position_func);

    let set_color = p.ac(sv("color"), root);
    let divide = p.ac(d(), set_color);
    p.ac(v("color0"), divide);
    p.ac(v("255.0"), divide);

    let set_uv = p.ac(sv("uv"), root);
    p.ac(v("textcoord"), set_uv);

    p.ac(ef(), root);
    p
}
