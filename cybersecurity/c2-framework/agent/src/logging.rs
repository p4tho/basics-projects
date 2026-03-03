#[macro_export]
macro_rules! logln {
    () => {
        println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    };
    ($($arg:tt)*) => {
        {
            let now: DateTime<Local> = Local::now();
            println!(
                "[{}] {}",
                now.format("%Y-%m-%d %H:%M:%S"),
                format_args!($($arg)*)
            );
        }
    };
}   