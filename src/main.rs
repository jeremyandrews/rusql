use std::io;
use std::io::Write;

struct InputBuffer {
    buffer: String,
    input_length: usize,
}

fn print_prompt() {
    print!("rusql> ");
    io::stdout().flush().unwrap();
}

fn read_input(mut input_buffer: InputBuffer) -> InputBuffer {
    input_buffer.buffer = String::from("");
    match io::stdin().read_line(&mut input_buffer.buffer) {
        Ok(bytes_read) => {
            // Ignore trailing newline
            input_buffer.input_length = bytes_read - 1;
            input_buffer.buffer.pop();
        }
        Err(_error) => {
            println!("Error reading input.\n");
            std::process::exit(1);
        }
    }
    input_buffer
}

fn main() {
    let mut input_buffer = InputBuffer {
        buffer: String::from(""),
        input_length: 0,
    };

    loop {
        print_prompt();
        input_buffer = read_input(input_buffer);
        if input_buffer.buffer == ".exit" {
            std::process::exit(0);
        } else {
            println!("Unrecognized command '{}'.\n", input_buffer.buffer);
        }
    }
}
