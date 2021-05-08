use std::fmt;
use std::fmt::{Display, Formatter, Write};

#[derive(Default, Debug)]
pub struct NinjaWriter {
    buffer: String,
    indentation: usize,
}

impl NinjaWriter {
    pub fn new() -> Self {
        return NinjaWriter::default();
    }

    pub fn include(&mut self, path: &str) {
        writeln!(self.buffer, "include {}", path).expect("Failed to write to buffer.");
    }

    pub fn subninja(&mut self, path: &str) {
        writeln!(self.buffer, "subninja {}", path).expect("Failed to write to buffer.");
    }

    pub fn default_rule(&mut self, rules: &str) {
        writeln!(self.buffer, "default {}", rules).expect("Failed to write to buffer.");
    }

    pub fn comment(&mut self, line: &str) {
        writeln!(self.buffer, "# {}", line).expect("Failed to write to buffer.");
    }

    pub fn variable(&mut self, key: &str, value: &str) {
        writeln!(
            self.buffer,
            "{}{} = {}",
            "  ".repeat(self.indentation),
            key,
            value
        )
        .expect("Failed to write to buffer.");
    }

    fn indent(&mut self) {
        self.indentation += 1
    }

    fn unindent(&mut self) {
        self.indentation -= 1
    }

    pub fn rule(
        &mut self,
        name: &str,
        command: &str,
        description: Option<&str>,
        depfile: Option<&str>,
        deps: Option<&str>,
        pool: Option<&str>,
    ) {
        writeln!(self.buffer, "rule {}", name).expect("Failed to write to buffer.");
        self.indent();
        self.variable("command", command);
        if let Some(desc) = description {
            self.variable("description", desc);
        }
        if let Some(deps) = deps {
            self.variable("deps", deps);
        }
        if let Some(depfile) = depfile {
            self.variable("depfile", depfile);
        }
        if let Some(pool) = pool {
            self.variable("pool", pool);
        }
        self.unindent();
        write!(self.buffer, "\n").expect("Failed to write to buffer.");
    }

    pub fn build(
        &mut self,
        outputs: &[&str],
        rule: &str,
        inputs: &[&str],
        variables: &[(&str, &str)],
    ) {
        writeln!(
            self.buffer,
            "build {}: {} {}",
            outputs.join(" "),
            rule,
            inputs.join(" ")
        )
        .expect("Failed to write to buffer.");
        self.indent();
        for (key, value) in variables {
            self.variable(key, value);
        }
        self.unindent();
        write!(self.buffer, "\n").expect("Failed to write to buffer.");
    }

    pub fn pool(&mut self, name: &str, depth: usize) {
        writeln!(self.buffer, "pool {}", name).expect("Failed to write to buffer.");
        self.indent();
        self.variable("depth", depth.to_string().as_str());
        self.unindent();
        write!(self.buffer, "\n").expect("Failed to write to buffer.");
    }
}

impl Display for NinjaWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.buffer.trim_end())
    }
}
