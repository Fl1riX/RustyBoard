use std::os::unix::process;
use std::io;
use sysinfo::{System, Pid};
use prettytable::{Table, Cell, Row, format};


/// The above Rust function retrieves system process information, displays it in a table format, and
/// allows the user to choose actions such as killing a process or quitting.
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);  

    //creating a table
    table.add_row(Row::new(vec![
        Cell::new("PID"),
        Cell::new("Name"),
        Cell::new("CPU Usage"),
        Cell::new("Memory Usage (MB)"),
    ]));
    
    //Filling the table 
    for (pid, process) in sys.processes() {
        table.add_row(Row::new(vec![
            Cell::new(&pid.to_string()),
            Cell::new(process.name()),
            Cell::new(&format!("{:.1}", process.cpu_usage())),
            Cell::new(&format!("{:.1}", process.memory() as f64 / 1024.0 / 1024.0)),
        ]));
    }
    
    //Print table
    table.printstd();


    println!("Choose action(k - kill process, q - quit): ");


    let mut user_input = String::new();
    
    //get action from user
    io::stdin()
        .read_line(&mut user_input)
        .expect("Input Error!");
    let user_input = user_input.trim();

    if user_input == "q" {
        std::process::exit(200)
    } else if user_input == "k" {

        println!("Enter pid: ");
        
        let mut proc_pid = String::new();

        io::stdin()
            .read_line(&mut proc_pid)
            .expect("Input Error!");

        let proc_pid = match proc_pid.trim().parse() {
            Ok(pid) => pid,
            Err(_) => 0
        };

        if let Some(process) = sys.process(Pid::from(proc_pid)) {
            process.kill();
            println!("Process {:?} was killed!", process.name());
        }


       
    } else { println!("Wrong action!") }    

    

}
