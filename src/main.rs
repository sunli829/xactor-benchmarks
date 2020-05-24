pub mod actix_test;
pub mod xactor_test;

#[derive(Debug, Clone)]
pub struct Spec {
    pub procs: u32,
    pub messages: u32,
    pub parallel: u32,
    pub size: u32,
}

impl std::fmt::Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} procs with {} messages {} in parallel of a size of {}",
            self.procs, self.messages, self.parallel, self.size
        )
    }
}

#[derive(Debug, Clone)]
pub struct Result {
    pub name: String,
    pub spec: Spec,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.name, self.spec.procs, self.spec.messages, self.spec.parallel, self.spec.size,
        )
    }
}

fn main() {}
