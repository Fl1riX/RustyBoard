use std::io;
use sysinfo::{System, Pid};
use prettytable::{Table, Cell, Row, format};

/// The above Rust function retrieves system process information, displays it in a table format, and
/// allows the user to choose actions such as killing a process or quitting.
fn main() {
    
    let mut sys = System::new_all();
    sys.refresh_all();

    table(&sys);

    println!("Choose action(k - kill process, f - find pid by name, q - quit): ");

    let mut user_input = String::new();
    
    //get action from user
    io::stdin()
        .read_line(&mut user_input)
        .expect("Input Error!");

    let user_input = user_input.trim();

    if user_input == "q" {
        std::process::exit(0)
    } else if user_input == "k" {

        println!("Enter pid/name: ");
        
        let mut killing_proc = String::new();

        io::stdin()
            .read_line(&mut killing_proc)
            .expect("Input Error!");

        let killing_proc = match killing_proc.trim().parse() {
            Ok(pid) => pid,
            Err(_) => 0
        };

        if let Some(process) = sys.process(Pid::from(killing_proc)) {
            process.kill();
            println!("Process {:?} was killed!", process.name());
        }
       
    } else if user_input == "f" {
        let mut proc_name = String::new();

        println!("Enter process name: ");

        io::stdin()
            .read_line(&mut proc_name)
            .expect("Error!");

        if let Some(pid) = find_pid_by_name(&proc_name.trim(), &sys){
            println!("{}PID: {}", proc_name, pid);
        } else {
            println!("Process not found!");
        }

        let mut user_choice = String::new();
        println!("Do you want to continue?(Y/n)");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Error!");

        match user_choice.trim() {
            "Y" => main(),
            "n" => std::process::exit(0),
            _ => println!("Error!"), 
        }

    } else { println!("Wrong action!") }    

}


fn table(system:&System)
{
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
    for (pid, process) in system.processes() {
        table.add_row(Row::new(vec![
            Cell::new(&pid.to_string()),
            Cell::new(process.name()),
            Cell::new(&format!("{:.1}", process.cpu_usage())),
            Cell::new(&format!("{:.1}", process.memory() as f64 / 1024.0 / 1024.0)),
        ]));
    }
    
    //Print table
    table.printstd();

}

fn find_pid_by_name(name: &str, system: &System) -> Option<Pid>
{
    for (pid, process) in system.processes() 
    {
        if process.name() == name {
            return Some(*pid);
        }
    } None
    
}
