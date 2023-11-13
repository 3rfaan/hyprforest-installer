#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        println!("\n{}", $msg.green().bold())
    };
}

#[macro_export]
macro_rules! prompt {
    ($msg:expr) => {
        print!("\n{} ", $msg.yellow().bold())
    };
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        println!("\n{}", $msg.red().bold())
    };
}

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        println!("{}", $msg.green())
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr,$err:ident) => {
        eprintln!("{} {} {}", $msg.red().bold(), "->".red().bold(), $err)
    };
    ($msg:expr) => {
        eprintln!("{}", $msg.red().bold())
    };
}

#[macro_export]
macro_rules! tip {
    ($msg:expr) => {
        println!("{}", $msg.bright_black().bold())
    };
}
