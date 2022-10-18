// #[derive(Serialize, Deserialize)]
pub enum Command {
    PreProcessor(String),
    Value(String),
    ReadVariable(String),
    SetVariable(String, Box<Command>),
    CallFunction(String),
    CallFunction1(String, String),
    CallFunction2(String, String, String),
    CallFunction3(String, String, String, String),
    CallFunction4(String, String, String, String, String),
    BeginFunction(String, String),
    BeginFunction1(String, String, String),
    BeginFunction2(String, String, String, String),
    Return(String),
    EndFunction(),

    Multiply(Box<Command>, Box<Command>),
    Add(Box<Command>, Box<Command>),
    Divide(Box<Command>, Box<Command>),
    Subtract(Box<Command>, Box<Command>),

    Parenthesis(Box<Command>),

    BeginIf(Box<Command>),
    EndIf(),

    LessThan(Box<Command>, Box<Command>),
    MoreThan(Box<Command>, Box<Command>),
    Equals(Box<Command>, Box<Command>),

    Or(Box<Command>, Box<Command>),
    And(Box<Command>, Box<Command>),
}

// #[derive(Serialize, Deserialize)]
pub struct Program {
    pub commands: Vec<Command>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

pub fn to_string(command: &Command) -> String {
    match command {
        Command::PreProcessor(value) => format!("#{}", value.to_string()),
        Command::Value(value) => value.to_string(),
        Command::ReadVariable(name) => name.to_string(),
        Command::SetVariable(name, cmd) => format!("{} = {}", name, to_string(cmd)),
        Command::CallFunction(name) => format!("{}()", name),
        Command::CallFunction1(name, arg0) => format!("{}({})", name, arg0),
        Command::CallFunction2(name, arg0, arg1) => format!("{}({}, {})", name, arg0, arg1),
        Command::CallFunction3(name, arg0, arg1, arg2) => {
            format!("{}({}, {}, {})", name, arg0, arg1, arg2)
        }
        Command::CallFunction4(name, arg0, arg1, arg2, arg3) => {
            format!("{}({}, {}, {}, {})", name, arg0, arg1, arg2, arg3)
        }
        Command::BeginFunction(return_type, name) => {
            format!("{} {}() {{", return_type.to_string(), name.to_string())
        }
        Command::BeginFunction1(return_type, name, arg0) => {
            format!(
                "{} {}({}) {{",
                return_type.to_string(),
                name.to_string(),
                arg0.to_string()
            )
        }
        Command::BeginFunction2(return_type, name, arg0, arg1) => {
            format!(
                "{} {}({}, {}) {{",
                return_type.to_string(),
                name.to_string(),
                arg0.to_string(),
                arg1.to_string()
            )
        }
        Command::EndFunction() => format!("}}"),
        Command::Multiply(left, right) => format!("{} * {}", to_string(left), to_string(right)),
        Command::Add(left, right) => format!("{} + {}", to_string(left), to_string(right)),
        Command::Subtract(left, right) => format!("{} - {}", to_string(left), to_string(right)),
        Command::Divide(left, right) => format!("{} / {}", to_string(left), to_string(right)),

        Command::Return(value) => format!("return {};", value.to_string()),

        Command::Parenthesis(value) => format!("({})", to_string(value)),

        Command::BeginIf(value) => format!("if ({}) {{", to_string(value)),
        Command::EndIf() => format!("}}"),

        Command::LessThan(left, right) => format!("{} < {}", to_string(left), to_string(right)),
        Command::MoreThan(left, right) => format!("{} > {}", to_string(left), to_string(right)),
        Command::Equals(left, right) => format!("{} == {}", to_string(left), to_string(right)),

        Command::Or(left, right) => format!("{} || {}", to_string(left), to_string(right)),
        Command::And(left, right) => format!("{} && {}", to_string(left), to_string(right)),
    }
}

pub fn type_to_string(command: &Command) -> String {
    match command {
        Command::PreProcessor(_) => format!("PreProcessor"),
        Command::Value(_) => format!("Value"),
        Command::ReadVariable(_) => format!("Read var"),
        Command::SetVariable(_, _) => format!("Set var"),
        Command::CallFunction(_) => format!("Call"),
        Command::CallFunction1(_, _) => format!("Call"),
        Command::CallFunction2(_, _, _) => format!("Call"),
        Command::CallFunction3(_, _, _, _) => format!("Call"),
        Command::CallFunction4(_, _, _, _, _) => format!("Call"),
        Command::BeginFunction(_, _) => format!("Begin func"),
        Command::BeginFunction1(_, _, _) => format!("Begin func"),
        Command::BeginFunction2(_, _, _, _) => format!("Begin func"),
        
        Command::EndFunction() => format!("End func"),
        Command::Multiply(_, _) => format!("Multiply"),
        Command::Add(_, _) => format!("Add"),
        Command::Subtract(_,_) => format!("Subtract"),
        Command::Divide(_,_) => format!("Divide"),

        Command::Return(_) => format!("Return"),

        Command::Parenthesis(_) => format!("Parenthesis"),

        Command::BeginIf(_) => format!("Begin if"),
        Command::EndIf() => format!("End if"),

        Command::LessThan(_, _) => format!("Less Than"),
        Command::MoreThan(_, _) => format!("More Than"),
        Command::Equals(_, _) => format!("Equals"),

        Command::Or(_, _) => format!("Logical Or"),
        Command::And(_, _) => format!("Logical And"),
    }
}


fn root_command_should_end_on_semi(command: &Command) -> bool {
    match command {
        Command::Value(_) => true,
        Command::SetVariable(_, _) => true,
        Command::CallFunction(_) => true,
        Command::CallFunction1(_,_) => true,
        Command::CallFunction2(_,_,_) => true,
        Command::CallFunction3(_,_,_,_) => true,
        Command::CallFunction4(_,_,_,_,_) => true,
        _ => false,
    }
}

pub fn print_program(program: &Program) {
    for command in &program.commands {
        print!("{}", to_string(&command));
        if root_command_should_end_on_semi(&command) {
            print!(";");
        }
        println!("");
    }
}
