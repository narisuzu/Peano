use std::{io, process::{Child, Command}};

pub struct StartConfig<'a> {
    pub server_jar: &'a str,
    pub jvm_args: &'a [&'a str],
}

pub fn start(cfg: StartConfig) -> io::Result<Child> {
    Command::new("java")
        .arg("-jar")
        .arg(cfg.server_jar)
        .args(cfg.jvm_args)
        .spawn()
}
