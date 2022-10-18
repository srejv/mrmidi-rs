use super::commands::*;

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

pub fn test_program() {
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
    let p = create_crt_fragment_shader();
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

pub fn create_crt_fragment_shader() -> Program {
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
