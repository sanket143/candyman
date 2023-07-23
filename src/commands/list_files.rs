use std::process::Command;

pub fn call(args: &Vec<&str>) {
    let mut command = Command::new("ls");

    if let Some((first, args)) = args.split_first() {
        for arg in args {
            command.arg(arg);
        }
    }

    let out = command.output().expect("ls command failed to start");
    println!("{}", String::from_utf8_lossy(&out.stdout));
}
