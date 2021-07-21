pub mod jre;
pub mod op;

#[cfg(test)]
mod tests {
    use crate::op::{self, StartConfig};
    #[test]
    fn it_works() {
        let cfg = StartConfig {
            server_jar: "xxx.jar",
            jvm_args: &[],
        };
        op::start(cfg);
    }
}
