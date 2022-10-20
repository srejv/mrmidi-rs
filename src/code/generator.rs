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
    let p = create_default_vertex_shader();
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

    p.print();

    println!("");
    println!(":: Default Fragment ::");
    println!("::::::::::::::::::::::");
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

    p.print();

    println!("");
    println!(":: CRT Vertex ::");
    println!("::::::::::::::::::::");
    let mut p = TreeProgram::new();
    let root = p.tree.node(pp("root", "don't print this"));
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

pub fn test_program_old() {
    println!("");
    println!(":: First test ::");
    let mut p = Program::new();
    p.commands.push(set_variable(
        "gl_FragCoord",
        call4("vec4", val("1.0"), val("1.0"), val("0.0"), val("1.0")),
    ));
    crate::code::commands::print_program(&p);

    println!("");
    println!(":: Default Fragment ::");
    println!("::::::::::::::::::::::");
    let p = create_default_fragment_shader();
    crate::code::commands::print_program(&p);

    println!("");
    println!(":: Default Vertex ::");
    println!("::::::::::::::::::::");
    let p = create_default_vertex_shader();
    crate::code::commands::print_program(&p);

    println!("");
    println!(":: CRT Vertex ::");
    println!("::::::::::::::::::::");
    let p = create_crt_vertex_shader();
    crate::code::commands::print_program(&p);

    println!("");
    println!(":: CRT Fragment ::");
    println!("::::::::::::::::::::");
    let p = create_crt_fragment_shader_old();
    crate::code::commands::print_program(&p);
}

pub fn create_default_fragment_shader() -> Program {
    let mut p = Program::new();
    p.commands.push(declare_pre_processor("version", "100"));
    p.commands.push(declare_precision("lowp", "float"));

    // macroquad uniforms
    p.commands.push(declare_varying("vec2", "uv"));
    p.commands.push(declare_uniform("sampler2D", "Texture"));

    // mrmidi
    p.commands.push(declare_uniform("float", "iTime"));
    p.commands.push(declare_uniform("float", "iTimeDelta"));
    p.commands.push(declare_uniform("int", "iFrame"));

    p.commands.push(begin_func("void", "main"));
    p.commands.push(set_variable(
        "gl_FragCoord",
        call2("texture2D", var("Texture"), var("uv")),
    ));
    p.commands.push(end_func());
    p
}

pub fn create_default_vertex_shader() -> Program {
    let mut p = Program::new();
    p.commands.push(declare_pre_processor("version", "100"));
    p.commands.push(declare_precision("lowp", "float"));

    p.commands.push(declare_attribute("vec3", "position"));
    p.commands.push(declare_attribute("vec2", "texcoord"));

    // macroquad uniforms
    p.commands.push(declare_varying("vec2", "uv"));

    p.commands.push(declare_uniform("sampler2D", "Texture"));
    p.commands.push(declare_uniform("mat4", "Model"));
    p.commands.push(declare_uniform("mat4", "Projection"));

    // mrmidi
    p.commands.push(declare_uniform("float", "iTime"));
    p.commands.push(declare_uniform("float", "iTimeDelta"));
    p.commands.push(declare_uniform("int", "iFrame"));

    p.commands.push(begin_func("void", "main"));
    p.commands.push(set_variable(
        "gl_Position",
        mult(
            var("Projection"),
            mult(var("Model"), var("vec4(position, 1)")),
        ),
    ));

    p.commands.push(Command::EndFunction());
    p
}

pub fn create_crt_vertex_shader() -> Program {
    let mut p = Program::new();
    p.commands.push(declare_pre_processor("version", "100"));
    p.commands.push(declare_precision("lowp", "float"));

    p.commands.push(declare_attribute("vec3", "position"));
    p.commands.push(declare_attribute("vec2", "texcoord"));
    p.commands.push(declare_attribute("vec4", "color0"));

    // macroquad uniforms
    p.commands.push(declare_varying("vec2", "uv"));
    p.commands.push(declare_varying("vec4", "color"));

    p.commands.push(declare_uniform("mat4", "Model"));
    p.commands.push(declare_uniform("mat4", "Projection"));

    p.commands.push(begin_func("void", "main"));

    p.commands.push(set_variable(
        "gl_Position",
        mult(
            var("Projection"),
            mult(var("Model"), var("vec4(position, 1)")),
        ),
    ));
    p.commands
        .push(set_variable("color", div(var("color0"), val("255.0"))));
    p.commands.push(set_variable("uv", var("texcoord")));

    p.commands.push(end_func());
    p
}

fn declare_pre_processor(command: &str, value: &str) -> Command {
    Command::PreProcessor(format!("{} {}", command, value))
}

fn declare_uniform(return_type: &str, name: &str) -> Command {
    Command::Value(format!("uniform {} {}", return_type, name))
}

fn declare_varying(return_type: &str, name: &str) -> Command {
    Command::Value(format!("varying {} {}", return_type, name))
}

fn declare_precision(value: &str, return_type: &str) -> Command {
    Command::Value(format!("precision {} {}", value, return_type))
}

fn declare_attribute(value: &str, return_type: &str) -> Command {
    Command::Value(format!("attribute {} {}", value, return_type))
}

fn set_variable(name: &str, command: Command) -> Command {
    Command::SetVariable(name.to_owned(), Box::new(command))
}

fn var(name: &str) -> Command {
    Command::ReadVariable(name.to_owned())
}

fn val(name: &str) -> Command {
    Command::Value(name.to_owned())
}

fn mult(left: Command, right: Command) -> Command {
    Command::Multiply(Box::new(left), Box::new(right))
}

fn add(left: Command, right: Command) -> Command {
    Command::Add(Box::new(left), Box::new(right))
}

fn div(left: Command, right: Command) -> Command {
    Command::Divide(Box::new(left), Box::new(right))
}

fn sub(left: Command, right: Command) -> Command {
    Command::Subtract(Box::new(left), Box::new(right))
}

fn ret(name: &str) -> Command {
    Command::Return(name.to_owned())
}

fn parens(value: Command) -> Command {
    Command::Parenthesis(Box::new(value))
}

fn begin_func(return_type: &str, name: &str) -> Command {
    Command::BeginFunction(return_type.to_owned(), name.to_owned())
}

fn begin_func1(return_type: &str, name: &str, arg0: &str) -> Command {
    Command::BeginFunction1(return_type.to_owned(), name.to_owned(), arg0.to_owned())
}

fn begin_func2(return_type: &str, name: &str, arg0: &str, arg1: &str) -> Command {
    Command::BeginFunction2(
        return_type.to_owned(),
        name.to_owned(),
        arg0.to_owned(),
        arg1.to_owned(),
    )
}

fn begin_if(check: Command) -> Command {
    Command::BeginIf(Box::new(check))
}

fn less_than(left: Command, right: Command) -> Command {
    Command::LessThan(Box::new(left), Box::new(right))
}

fn more_than(left: Command, right: Command) -> Command {
    Command::MoreThan(Box::new(left), Box::new(right))
}

fn or(left: Command, right: Command) -> Command {
    Command::Or(Box::new(left), Box::new(right))
}

fn and(left: Command, right: Command) -> Command {
    Command::And(Box::new(left), Box::new(right))
}

fn equals(left: Command, right: Command) -> Command {
    Command::Equals(Box::new(left), Box::new(right))
}

fn end_func() -> Command {
    Command::EndFunction()
}

fn end_if() -> Command {
    Command::EndFunction()
}

fn call1(name: &str, arg0: Command) -> Command {
    Command::CallFunction1(name.to_owned(), to_string(&arg0).to_owned())
}

fn call2(name: &str, arg0: Command, arg1: Command) -> Command {
    Command::CallFunction2(
        name.to_owned(),
        to_string(&arg0).to_owned(),
        to_string(&arg1).to_owned(),
    )
}

fn call3(name: &str, arg0: Command, arg1: Command, arg2: Command) -> Command {
    Command::CallFunction3(
        name.to_owned(),
        to_string(&arg0).to_owned(),
        to_string(&arg1).to_owned(),
        to_string(&arg2).to_owned(),
    )
}

fn call4(name: &str, arg0: Command, arg1: Command, arg2: Command, arg3: Command) -> Command {
    Command::CallFunction4(
        name.to_owned(),
        to_string(&arg0).to_owned(),
        to_string(&arg1).to_owned(),
        to_string(&arg2).to_owned(),
        to_string(&arg3).to_owned(),
    )
}

fn program_header(commands: &mut Vec<Command>) {
    commands.push(declare_pre_processor("version", "100"));
    commands.push(declare_precision("lowp", "float"));

    commands.push(declare_varying("vec4", "color"));
    commands.push(declare_varying("vec2", "uv"));

    commands.push(declare_uniform("float", "iTime"));

    commands.push(declare_uniform("sampler2D", "Texture"));

    commands.push(declare_uniform("sampler2D", "iChannel0"));
    commands.push(declare_uniform("sampler2D", "iChannel1"));

    commands.push(declare_uniform("vec3", "iResolution"));
}

fn add_function_crt_curve_uv(commands: &mut Vec<Command>) {
    commands.push(begin_func1("vec2", "CRTCurveUV", "vec2 uv"));

    commands.push(set_variable(
        "uv",
        mult(var("uv"), sub(val("2.0"), val("1.0"))),
    ));

    commands.push(set_variable(
        "vec2 offset",
        div(var("abs(uv.yx)"), val("vec2(6.0,4.0)")),
    ));

    commands.push(set_variable(
        "uv",
        add(
            var("uv"),
            mult(var("uv"), mult(var("offset"), var("offset"))),
        ),
    ));

    commands.push(set_variable(
        "uv",
        mult(var("uv"), add(val("0.5"), val("0.5"))),
    ));

    commands.push(ret("uv"));

    commands.push(end_func());
}

fn add_function_draw_vignette(commands: &mut Vec<Command>) {
    commands.push(begin_func2(
        "void",
        "DrawVignette",
        "inout vec3 color",
        "vec2 uv",
    ));
    commands.push(set_variable(
        "float vignette",
        mult(
            var("uv.x"),
            mult(
                var("uv.y"),
                mult(
                    parens(sub(val("1.0"), var("uv.x"))),
                    parens(sub(val("1.0"), var("uv.y"))),
                ),
            ),
        ),
    ));
    commands.push(set_variable(
        "vignette",
        call3(
            "clamp",
            call2("pow", mult(val("16.0"), var("vignette")), val("0.3")),
            val("0.0"),
            val("1.0"),
        ),
    ));

    commands.push(set_variable("color", mult(var("color"), var("vignette"))));
    commands.push(end_func());
}

fn add_function_scanline(commands: &mut Vec<Command>) {
    commands.push(begin_func2(
        "void",
        "DrawScanline",
        "inout vec3 color",
        "vec2 uv",
    ));

    commands.push(set_variable(
        "float scanline",
        call3(
            "clamp",
            add(
                val("0.95"),
                mult(
                    val("0.5"),
                    call1(
                        "cos",
                        mult(
                            val("3.14"),
                            mult(
                                parens(add(var("uv.y"), mult(val("0.008"), var("iTime")))),
                                mult(val("240.0"), val("1.0")),
                            ),
                        ),
                    ),
                ),
            ),
            val("0.0"),
            val("1.0"),
        ),
    ));

    commands.push(set_variable(
        "float grille",
        add(
            val("0.85"),
            mult(
                val("0.15"),
                call3(
                    "clamp",
                    mult(
                        val("1.5"),
                        call1(
                            "cos",
                            mult(
                                val("3.14"),
                                mult(var("uv.x"), mult(val("640.0"), val("1.0"))),
                            ),
                        ),
                    ),
                    val("0.0"),
                    val("1.0"),
                ),
            ),
        ),
    ));

    commands.push(set_variable(
        "color",
        mult(
            var("color"),
            mult(var("scanline"), mult(var("graille"), val("1.2"))),
        ),
    ));

    commands.push(end_func());
}

pub fn create_crt_fragment_shader_old() -> Program {
    let mut p = Program::new();

    program_header(&mut p.commands);

    add_function_crt_curve_uv(&mut p.commands);
    add_function_draw_vignette(&mut p.commands);
    add_function_scanline(&mut p.commands);

    // Main
    p.commands.push(begin_func("void", "main"));

    p.commands
        .push(set_variable("vec2 crtUV", call1("CRTCurveUV", var("uv"))));

    p.commands.push(set_variable(
        "vec2 mehuv",
        call2(
            "vec2",
            sub(val("1.0"), var("crtUV.x")),
            sub(val("1.0"), var("crtUV.y")),
        ),
    ));

    p.commands.push(set_variable(
        "vec3 channel0",
        call2("texture2D", var("iChannel0"), var("mehuv")),
    ));
    p.commands.push(set_variable(
        "vec3 channel1",
        call2("texture2D", var("iChannel1"), var("mehuv")),
    ));
    p.commands.push(set_variable(
        "vec3 res",
        call3(
            "mix",
            var("iChannel0"),
            var("iChannel1"),
            mult(call1("sin", var("iTime")), add(val("0.5"), val("0.5"))),
        ),
    ));

    p.commands.push(begin_if(or(
        less_than(var("crtUV.x"), val("0.0")),
        or(
            more_than(var("crtUV.x"), val("1.0")),
            or(
                less_than(var("crtUV.y"), val("0.0")),
                more_than(var("crtUV.x"), val("1.0")),
            ),
        ),
    )));
    p.commands.push(set_variable(
        "res",
        call3("vec3", val("0.0"), val("0.0"), val("0.0")),
    ));
    p.commands.push(end_if());

    p.commands
        .push(call2("DrawVignette", var("res"), var("crtUV")));
    p.commands
        .push(call2("DrawScanline", var("res"), var("uv")));

    p.commands
        .push(set_variable("gl_FragColor", val("vec4(res, 1.0)")));
    p.commands.push(end_func());
    p
}
