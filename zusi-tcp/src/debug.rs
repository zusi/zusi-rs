use std::fmt;

use crate::node::{Attribute, Node};

fn write_node(n: &Node, indent: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..indent {
        write!(f, "  ")?;
    }
    writeln!(f, "Node, ID = {:#x} [", n.id)?;
    for a in &n.attributes {
        for _ in 0..(indent + 1) {
            write!(f, "  ")?;
        }
        writeln!(f, "{:?}", a)?;
    }
    for n in &n.children {
        write_node(n, indent + 1, f)?;
        writeln!(f, "")?;
    }
    for _ in 0..indent {
        write!(f, "  ")?;
    }
    write!(f, "]")
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_node(self, 0, f)
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Attribute, ID = {:#x}, Value = {:?}",
            self.id, self.value
        )?;
        if self.value.len() == 2 {
            match self.as_u16() {
                Ok(val) => write!(f, ", as_u16 = {}", val)?,
                Err(_) => (),
            };
        }
        if self.value.len() == 4 {
            match self.as_f32() {
                Ok(val) => write!(f, ", as_f32 = {}", val)?,
                Err(_) => (),
            };
        }
        match self.to_str() {
            Ok(val) => write!(f, ", as_str = {:?}", val)?,
            Err(_) => (),
        };
        Ok(())
    }
}
