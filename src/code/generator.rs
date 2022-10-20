use super::commands::*;

fn v(value: &str) -> CodeNode {
    (CommandType::Value, value.to_owned())
}

fn r() -> CodeNode {
    (CommandType::Return, "".to_owned())
}

fn sv(value: &str) -> CodeNode {
    (CommandType::SetVariable, value.to_owned())
}
fn pp(command: &str, value: &str) -> CodeNode {
    (CommandType::PreProcessor, format!("{} {}", command, value))
}
fn ps() -> CodeNode {
    (CommandType::Parenthesis, "".to_owned())
}

fn bi() -> CodeNode {
    (CommandType::BeginIf, "".to_owned())
}

fn ei() -> CodeNode {
    (CommandType::EndIf, "".to_owned())
}

fn cf(value: &str) -> CodeNode {
    (CommandType::CallFunction, value.to_owned())
}
fn pc(value: &str, return_type: &str) -> CodeNode {
    (
        CommandType::Value,
        format!("precision {} {}", value, return_type),
    )
}

fn c(p: &mut TreeProgram, parent: usize, func: CodeNode, args: &[CodeNode]) {
    let f = p.tree.add_child(func, parent);
    for arg in args {
        p.tree.add_child(arg.clone(), f);
    }
}

fn ac(p: &mut TreeProgram, func: CodeNode, parent: usize) -> usize {
    p.tree.add_child(func, parent)
}

fn atrib(return_type: &str, value: &str) -> CodeNode {
    (
        CommandType::Value,
        format!("attribute {} {}", return_type, value),
    )
}

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
    p.tree.add_child(pp("version", "100"), root);
    p.tree.add_child(pc("lowp", "float"), root);

    p.tree.add_child(vary("vec4", "color"), root);
    p.tree.add_child(vary("vec2", "uv"), root);

    p.tree.add_child(uni("float", "iTime"), root);

    p.tree.add_child(uni("sampler2D", "Texture"), root);

    p.tree.add_child(uni("sampler2D", "iChannel0"), root);
    p.tree.add_child(uni("sampler2D", "iChannel1"), root);

    p.tree.add_child(uni("vec3", "iResolution"), root);

    let crt_curve_uv = p.tree.add_child(bf(), root);
    p.tree.add_child(v("vec2"), crt_curve_uv);
    p.tree.add_child(v("CRTCurveUV"), crt_curve_uv);
    p.tree.add_child(v("vec2 uv"), crt_curve_uv);

    let set_uv = p.tree.add_child(sv("uv"), root);
    let multiply = p.tree.add_child(m(), set_uv);
    p.tree.add_child(v("uv"), multiply);
    let subtract = p.tree.add_child(s(), multiply);
    p.tree.add_child(v("2.0"), subtract);
    p.tree.add_child(v("1.0"), subtract);

    let set_offset = p.tree.add_child(sv("vec2 offset"), root);
    let divide = p.tree.add_child(d(), set_offset);
    c(&mut p, divide, cf("abs"), &[v("uv.yx")]);
    c(&mut p, divide, cf("vec2"), &[v("6.0"), v("4.0")]);

    let set_uv = p.tree.add_child(sv("uv"), root);
    let add = p.tree.add_child(a(), set_uv);
    p.tree.add_child(v("uv"), add);
    let multiply = p.tree.add_child(m(), add);
    p.tree.add_child(v("uv"), multiply);
    p.tree.add_child(v("offset"), multiply);
    p.tree.add_child(v("offset"), multiply);

    let set_uv = p.tree.add_child(sv("uv"), root);
    let multiply = p.tree.add_child(m(), set_uv);
    p.tree.add_child(v("uv"), multiply);
    let add = p.tree.add_child(a(), multiply);
    p.tree.add_child(v("0.5"), add);
    p.tree.add_child(v("0.5"), add);

    let return_value = p.tree.add_child(r(), root);
    p.tree.add_child(v("uv"), return_value);
    p.tree.add_child(ef(), root);

    let draw_vignette = p.tree.add_child(bf(), root);
    p.tree.add_child(v("void"), draw_vignette);
    p.tree.add_child(v("DrawVignette"), draw_vignette);
    p.tree.add_child(v("inout vec3 color"), draw_vignette);
    p.tree.add_child(v("vec2 uv"), draw_vignette);

    let set_vignette = p.tree.add_child(sv("float vignette"), root);
    let multiply = p.tree.add_child(m(), set_vignette);
    p.tree.add_child(v("uv.x"), multiply);
    p.tree.add_child(v("uv.y"), multiply);

    let parenthesis = p.tree.add_child(ps(), multiply);
    let subtract = p.tree.add_child(s(), parenthesis);
    p.tree.add_child(v("1.0"), subtract);
    p.tree.add_child(v("uv.x"), subtract);

    let parenthesis = p.tree.add_child(ps(), multiply);
    let subtract = p.tree.add_child(s(), parenthesis);
    p.tree.add_child(v("1.0"), subtract);
    p.tree.add_child(v("uv.y"), subtract);

    let set_vignette = p.tree.add_child(sv("vignette"), root);
    let clamp_call = p.tree.add_child(cf("clamp"), set_vignette);
    let pow_call = p.tree.add_child(cf("pow"), clamp_call);
    let multiply = p.tree.add_child(m(), pow_call);
    p.tree.add_child(v("16.0"), multiply);
    p.tree.add_child(v("vignette"), multiply);
    p.tree.add_child(v("0.3"), pow_call);

    p.tree.add_child(v("0.0"), clamp_call);
    p.tree.add_child(v("1.0"), clamp_call);

    let set_color = p.tree.add_child(sv("color"), root);
    let multiply = p.tree.add_child(m(), set_color);
    p.tree.add_child(v("color"), multiply);
    p.tree.add_child(v("vignette"), multiply);

    p.tree.add_child(ef(), root);

    // void DrawScanline(inout vec3 color, vec2 uv)
    let scanline = p.tree.add_child(bf(), root);
    p.tree.add_child(v("void"), scanline);
    p.tree.add_child(v("DrawScanline"), scanline);
    p.tree.add_child(v("inout vec3 color"), scanline);
    p.tree.add_child(v("vec2 uv"), scanline);

    let set_scanline = p.tree.add_child(sv("float scanline"), root);
    let clamp_call = p.tree.add_child(cf("clamp"), set_scanline);
    let add = ac(&mut p, a(), clamp_call);
    ac(&mut p, v("0.0"), clamp_call);
    ac(&mut p, v("1.0"), clamp_call);

    ac(&mut p, v("0.95"), add);
    let multiply = ac(&mut p, m(), add);
    ac(&mut p, v("0.5"), multiply);
    let cos_call = ac(&mut p, cf("cos"), multiply);
    let multiply = ac(&mut p, m(), cos_call);
    ac(&mut p, v("3.14"), multiply);
    let more = ac(&mut p, ps(), multiply);
    ac(&mut p, v("240.0"), multiply);
    ac(&mut p, v("10.0"), multiply);

    let add = ac(&mut p, a(), more);
    ac(&mut p, v("uv.y"), add);
    let multiply = ac(&mut p, m(), add);
    ac(&mut p, v("0.008"), multiply);
    ac(&mut p, v("iTime"), multiply);

    let set_grille = ac(&mut p, sv("float grille"), root);
    let add = ac(&mut p, a(), set_grille);
    ac(&mut p, v("0.85"), add);
    let multiply = ac(&mut p, m(), add);
    ac(&mut p, v("1.5"), multiply);
    let clamp_call = ac(&mut p, cf("clamp"), multiply);
    let cos_call = ac(&mut p, cf("cos"), clamp_call);
    let multiply = ac(&mut p, m(), cos_call);
    ac(&mut p, v("3.14"), multiply);
    ac(&mut p, v("uv.x"), multiply);
    p.ac(v("640.0"), multiply);
    p.ac(v("1.0"), multiply);

    p.ac(v("0.0"), clamp_call);
    p.ac(v("1.0"), clamp_call);

    let set_color = ac(&mut p, sv("color"), root);
    let multiply = ac(&mut p, m(), set_color);
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
    p.tree.add_child(pp("version", "100"), root);
    p.tree.add_child(pc("lowp", "float"), root);

    // macroquad uniforms
    p.tree.add_child(vary("vec2", "uv"), root);
    p.tree.add_child(uni("sampler2D", "Texture"), root);

    // mrmidi
    p.tree.add_child(uni("float", "iTime"), root);
    p.tree.add_child(uni("float", "iTimeDelta"), root);
    p.tree.add_child(uni("int", "iFrame"), root);

    let main = p.tree.add_child(bf(), root);
    p.tree.add_child(v("void"), main);
    p.tree.add_child(v("main"), main);

    let set_color = p.tree.add_child(sv("gl_FragCoord"), root);
    let call_sample_texture = p.tree.add_child(cf("texture2D"), set_color);
    p.tree.add_child(v("Texture"), call_sample_texture);
    p.tree.add_child(v("uv"), call_sample_texture);

    p.tree.add_child(ef(), root);
    p
}

pub fn create_default_vertex_shader() -> TreeProgram {
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("Rootnode", "Don't print."));

    p.tree.add_child(pp("version", "100"), root);

    p.tree.add_child(pc("lowp", "float"), root);

    p.tree.add_child(atrib("vec3", "position"), root);
    p.tree.add_child(atrib("vec2", "texcoord"), root);

    // // macroquad uniforms
    p.tree.add_child(vary("vec2", "uv"), root);

    p.tree.add_child(uni("sampler2D", "Texture"), root);
    p.tree.add_child(uni("mat4", "Model"), root);
    p.tree.add_child(uni("mat4", "Projection"), root);

    // mrmidi
    p.tree.add_child(uni("float", "iTime"), root);
    p.tree.add_child(uni("float", "iTimeDelta"), root);
    p.tree.add_child(uni("int", "iFrame"), root);

    let main = p.tree.add_child(bf(), root);
    p.tree.add_child(v("void"), main);
    p.tree.add_child(v("main"), main);

    let set_position = p.tree.add_child(sv("gl_Position"), root);
    let multiply = p.tree.add_child(m(), set_position);
    p.tree.add_child(v("Projection"), multiply);
    p.tree.add_child(v("Model"), multiply);
    let call_conversion = p.tree.add_child(cf("vec4"), multiply);
    p.tree.add_child(v("position"), call_conversion);
    p.tree.add_child(v("1"), call_conversion);

    p.tree.add_child(ef(), root);
    p
}

pub fn create_crt_vertex_shader() -> TreeProgram {
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("root", "don't print this");
    p.tree.add_child(pp("version", "100"), root);
    p.tree.add_child(pc("lowp", "float"), root);

    p.tree.add_child(atrib("vec3", "position"), root);
    p.tree.add_child(atrib("vec2", "texcoord"), root);
    p.tree.add_child(atrib("vec4", "color0"), root);

    // macroquad uniforms
    p.tree.add_child(vary("vec2", "uv"), root);
    p.tree.add_child(vary("vec4", "color"), root);

    p.tree.add_child(uni("mat4", "Model"), root);
    p.tree.add_child(uni("mat4", "Projection"), root);

    let main = p.tree.add_child(bf(), root);
    p.tree.add_child(v("void"), main);
    p.tree.add_child(v("main"), main);

    let set_position = p.tree.add_child(sv("gl_Position"), root);
    let multiply = p.tree.add_child(m(), set_position);
    p.tree.add_child(v("Projection"), multiply);
    p.tree.add_child(v("Model"), multiply);
    let position_func = p.tree.add_child(cf("vec4"), multiply);
    p.tree.add_child(v("position"), position_func);
    p.tree.add_child(v("1"), position_func);

    let set_color = p.tree.add_child(sv("color"), root);
    let divide = p.tree.add_child(d(), set_color);
    p.tree.add_child(v("color0"), divide);
    p.tree.add_child(v("255.0"), divide);

    let set_uv = p.tree.add_child(sv("uv"), root);
    p.tree.add_child(v("textcoord"), set_uv);

    p.tree.add_child(ef(), root);
    p
}
