use factory::compiler::Compiler;
use factory::parser::program as parse_program;
use factory::vm::VM;
use nom::Finish;
use nom_locate::LocatedSpan;
use std::io::{Read, Write};

fn main() {
    // read stdin until eof
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // parse input
    let program = parse_program(LocatedSpan::new(&input)).finish();
    if let Err(e) = program {
        println!("{:#?}", e);
        return;
    }
    let program = program.unwrap();
    println!("remaining: {:?}", program);
    let program = program.1;
    // let program = product_expression(&input).unwrap().1;

    for stmt in program.iter() {
        println!("{:#?}", stmt);
    }

    let mut compiler = Compiler::new();
    compiler.compile_top(&program[0]);
    let code = compiler.link();

    // write code to code.txt
    let mut file = std::fs::File::create("code.txt").unwrap();
    for (i, c) in code.iter().enumerate() {
        writeln!(file, "{}: {:#?}", i, c).unwrap();
    }

    // create VM
    let mut vm = VM::new(1024);
    println!("VM started");

    // register native functions
    let natives = vec![factory::extension::basic::BasicFunctions::default()];
    natives.iter().for_each(|f| {
        factory::extension::register_native(&mut vm, f);
    });

    // start user code
    vm.set_code(code);
    loop {
        vm.step_code();
        // vm.dump_stack();
        // if let Some(v) = vm.stack_top() {
        //     println!("{:?}", v);
        // }
        // vm.gc_debug();
    }
}
