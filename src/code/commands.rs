#[derive(PartialEq, Clone, Copy)]
pub enum Type{
    Float,
    Vec2,
    Vec3,
    Vec4,
    Int,
    Sampler2D, // How do the rest work? Sampler 1d? sampler 3D? We probably want 2 channel input on audio. Midi is also 2 channel. 
}

#[derive(PartialEq, Clone, Copy)]
pub enum CommandType {
    PreProcessor,
    Value,
    ReadVariable,
    SetVariable,
    CallFunction,
    BeginFunction,
    Return,
    EndFunction,

    Multiply,
    Add,
    Divide,
    Subtract,

    Parenthesis,

    BeginIf,
    EndIf,

    LessThan,
    MoreThan,
    Equals,

    Or,
    And,

    DeclareVariable,
}
impl Default for CommandType {
    fn default() -> Self {
        CommandType::Value
    }
}

use crate::code::arena::ArenaTree;
use crate::code::arena::Node;
pub type CodeNode = (CommandType, String);

pub struct TreeProgram {
    pub tree: ArenaTree<CodeNode>,
}

impl TreeProgram {
    pub fn new() -> Self {
        Self {
            tree: crate::code::arena::ArenaTree::default(),
        }
    }

    pub fn ac(&mut self, func: CodeNode, parent: usize) -> usize {
        self.tree.add_child(func, parent)
    }

    pub fn dv(&mut self, typename: CodeNode, name: CodeNode, parent: usize) -> usize {
        let func = self
            .tree
            .add_child((CommandType::DeclareVariable, "".to_owned()), parent);
        self.tree.add_child(typename, func);
        self.tree.add_child(name, func);
        func
    }

    pub fn root_command_should_end_on_semi2(command: &CodeNode) -> bool {
        match command.0 {
            CommandType::Value => true,
            CommandType::SetVariable => true,
            CommandType::CallFunction => true,
            _ => false,
        }
    }

    pub fn print(&mut self) {
        let root = &self.tree.arena[0];
        for childIdx in &root.children {
            let idx = *childIdx as usize;
            print!("{}", to_strang(&self.tree.arena[idx], &self.tree));
            let cmd_type = &self.tree.arena[idx].val;
            if TreeProgram::root_command_should_end_on_semi2(&cmd_type) {
                print!(";");
            }
            println!("");
        }
    }

    pub fn to_string(&mut self) -> String {
        let mut strings: Vec<String> = Vec::new();
        for child_idx in &self.tree.arena[0].children {
            let idx = *child_idx;
            let row = to_strang(&self.tree.arena[idx], &self.tree);
            let cmd_type = &self.tree.arena[idx].val;
            let mut ending = "";
            if TreeProgram::root_command_should_end_on_semi2(&cmd_type) {
                ending = ";";
            }

            strings.push(format!("{}{}", &row, &ending));
        }

        return strings.join("\n");
    }
}

pub fn to_strang(node: &Node<CodeNode>, arena: &ArenaTree<CodeNode>) -> String {
    match node.val.0 {
        CommandType::PreProcessor => format!("#{}", node.val.1),
        CommandType::Value => format!("{}", node.val.1),
        CommandType::ReadVariable => format!("{}", node.val.1),
        CommandType::SetVariable => format!(
            "{} = {}",
            node.val.1,
            to_strang(&arena.arena[node.children[0]], &arena)
        ),
        CommandType::CallFunction => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            format!("{}({})", node.val.1, arguments.join(", "))
        }
        CommandType::BeginFunction => {
            let return_type = &arena.arena[node.children[0]].val.1;
            let name = &arena.arena[node.children[1]].val.1;
            let arguments: Vec<String> = node
                .children
                .iter()
                .skip(2)
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            format!("{} {}({}) {{", return_type, name, arguments.join(", "))
        }
        CommandType::EndFunction => format!("}}"),

        CommandType::Multiply => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" * ")
        }
        CommandType::Add => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" + ")
        }
        CommandType::Subtract => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" - ")
        }
        CommandType::Divide => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" / ")
        }

        CommandType::Return => {
            format!(
                "return {};",
                to_strang(&arena.arena[node.children[0]], &arena)
            )
        }

        CommandType::Parenthesis => {
            format!("({})", to_strang(&arena.arena[node.children[0]], &arena))
        }

        CommandType::BeginIf => format!(
            "if ({}) {{",
            to_strang(&arena.arena[node.children[0]], &arena)
        ),
        CommandType::EndIf => format!("}}"),

        CommandType::LessThan => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" < ")
        }
        CommandType::MoreThan => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" > ")
        }
        CommandType::Equals => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" == ")
        }

        CommandType::Or => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" || ")
        }
        CommandType::And => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" && ")
        }

        CommandType::DeclareVariable => {
            let arguments: Vec<String> = node
                .children
                .iter()
                .map(|x| to_strang(&arena.arena[*x], &arena))
                .collect();
            arguments.join(" = ")
        }
    }
}

pub fn type_to_strang(command: &Node<CodeNode>) -> String {
    match command.val.0 {
        CommandType::PreProcessor => format!("PreProcessor"),
        CommandType::Value => format!("Value"),
        CommandType::ReadVariable => format!("Read var"),
        CommandType::SetVariable => format!("Set var"),
        CommandType::CallFunction => format!("Call"),
        CommandType::BeginFunction => format!("Begin func"),

        CommandType::EndFunction => format!("End func"),
        CommandType::Multiply => format!("Multiply"),
        CommandType::Add => format!("Add"),
        CommandType::Subtract => format!("Subtract"),
        CommandType::Divide => format!("Divide"),

        CommandType::Return => format!("Return"),

        CommandType::Parenthesis => format!("Parenthesis"),

        CommandType::BeginIf => format!("Begin if"),
        CommandType::EndIf => format!("End if"),

        CommandType::LessThan => format!("Less Than"),
        CommandType::MoreThan => format!("More Than"),
        CommandType::Equals => format!("Equals"),

        CommandType::Or => format!("Logical Or"),
        CommandType::And => format!("Logical And"),

        CommandType::DeclareVariable => format!("Declare Variable"),
    }
}
