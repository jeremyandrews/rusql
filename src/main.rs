use std::io;
use std::io::Write;

struct InputBuffer {
    buffer: String,
    input_length: usize,
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PepareUnrecognizedStatement,
}

enum StatementType {
    None,
    StatementInsert,
    StatementSelect,
}

struct Statement {
    stype: StatementType,
}

fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    if input_buffer.buffer == ".exit" {
        std::process::exit(0);
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer.starts_with("insert") {
        statement.stype = StatementType::StatementInsert;
        PrepareResult::PrepareSuccess
    } else if input_buffer.buffer == "select" {
        statement.stype = StatementType::StatementSelect;
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PepareUnrecognizedStatement
    }
}

fn execute_statement(statement: &Statement) {
    match statement.stype {
        StatementType::None => {
            println!("This is an uninitialized statement.");
        }
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.");
        },
        StatementType::StatementSelect => {
            println!("This is where we would do a select.");
        }
    }
}

fn print_prompt() {
    print!("rusql> ");
    io::stdout().flush().unwrap();
}

fn read_input(mut input_buffer: InputBuffer) -> InputBuffer {
    input_buffer.buffer = String::new();
    match io::stdin().read_line(&mut input_buffer.buffer) {
        Ok(bytes_read) => {
            // Ignore trailing newline
            input_buffer.input_length = bytes_read - 1;
            input_buffer.buffer.pop();
        }
        Err(_error) => {
            println!("Error reading input.");
            std::process::exit(1);
        }
    }
    input_buffer
}

fn main() {
    let mut input_buffer = InputBuffer {
        buffer: String::new(),
        input_length: 0,
    };

    loop {
        print_prompt();
        input_buffer = read_input(input_buffer);
        if input_buffer.buffer.chars().next().unwrap() == '.' {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::MetaCommandSuccess => {},
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input_buffer.buffer);
                },
            }
        }
        else {
            let mut statement = Statement {
                stype: StatementType::None,
            };
            match prepare_statement(&input_buffer, &mut statement) {
                PrepareResult::PrepareSuccess => {
                    execute_statement(&statement);
                    println!("Executed.");
                }
                PrepareResult::PepareUnrecognizedStatement => {
                    println!("Unrecognized keyword at start of '{}'.", input_buffer.buffer);
                }
            }
        }
    }
}
