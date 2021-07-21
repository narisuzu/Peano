use std::{process::Command};

fn jre_line() -> Result<String, &'static str> {
    let output = Command::new("java")
        .arg("--version")
        .output()
        .map_err(|_| "java is fail to execute")?;

    if output.status.success() {
        let mut line = String::new();
        for ch in output.stdout {
            if ch == b'\n' {
                break;
            }
            line.push(ch as char);
        }
        Ok(line)
    } else {
        Err("'java -version' is fail to execute")
    }
}

fn jre_raw_version() -> Result<String, &'static str> {
    let first_line = jre_line()?;
    first_line
        .split_ascii_whitespace()
        .nth(1)
        .map(|s| s.to_string())
        .ok_or("cannot parse java version")
}

fn jre_major_version() -> Result<u32, &'static str> {
    let version = jre_raw_version()?;
    let versions: Vec<&str> = version.split(".").collect();
    if versions[0] == "1" {
        versions[1]
    } else {
        versions[0]
    }
    .parse::<u32>()
    .or(Err("cannot parse number"))
}

#[cfg(test)]
mod tests {
    use crate::jre::{jre_major_version};
    #[test]
    fn check_jre_version() {
        let v = jre_major_version().unwrap();
        assert_eq!(v, 16);
    }
}
