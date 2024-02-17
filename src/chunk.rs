use std::io::{Error, Write};
use std::vec::Vec;

use crate::Values;

#[derive(Debug)]
pub struct Chunk {
    name: String,
    // We're packing opcodes and arguments in this vector of binary data.
    // Since the data won't overlap with the Opcode enum, that needs some
    // wrangling to play nice.
    code: Vec<u8>,
    // Line number origin of each item in code.
    lines: Vec<usize>,

    constants: Vec<Values>,
}

impl Chunk {
    fn new(s: String) -> Self {
        return Chunk {
            name: s,
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        };
    }

    fn push_op(&mut self, c: super::Opcode, line: usize) {
        self.code.push(c as u8);
        self.lines.push(line);
    }

    fn push_const(&mut self, v: super::Values, line: usize) {
        self.constants.push(v);
        self.code.push(super::Opcode::Constant as u8);
        self.lines.push(line);
        self.code.push(self.constants.len() as u8 - 1);
        self.lines.push(line);
    }

    fn disassemble(&self, w: &mut impl Write) -> Result<(), Error> {
        writeln!(w, "== {} ==", self.name)?;

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(w, offset)?;
        }
        Ok(())
    }

    fn disassemble_instruction(&self, w: &mut impl Write, offset: usize) -> Result<usize, Error> {
        write!(w, "{:04} ", offset)?;
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            write!(w, "   | ")?;
        } else {
            write!(w, "{:4} ", self.lines[offset])?;
        }

        match self.code.get(offset) {
            Some(&u) => match super::Opcode::try_from(u) {
                Ok(crate::Opcode::Return) => self.simple_instruction(w, "RETURN", offset),
                Ok(crate::Opcode::Constant) => self.const_instruction(w, "CONSTANT", offset),
                Err(()) => {
                    writeln!(w, "unknown opcode {}", u)?;
                    Ok(offset + 1)
                }
            },
            None => {
                writeln!(w, "no more opcodes")?;
                Ok(offset + 1)
            }
        }
    }

    fn simple_instruction(
        &self,
        w: &mut impl Write,
        name: &str,
        offset: usize,
    ) -> Result<usize, Error> {
        writeln!(w, "{}", name)?;
        Ok(offset + 1)
    }

    fn const_instruction(
        &self,
        w: &mut impl Write,
        name: &str,
        offset: usize,
    ) -> Result<usize, Error> {
        let idx = self.code.get(offset + 1).unwrap();
        write!(w, "{} {:03} ", name, idx)?;
        write_value(w, self.constants.get(*idx as usize).unwrap())?;
        writeln!(w, "")?;
        Ok(offset + 2)
    }
}

fn write_value(w: &mut impl Write, v: &super::Values) -> Result<(), Error> {
    match v {
        Values::Double(d) => {
            write!(w, "'{}'", d)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_chunk() {
        let mut c = Chunk::new("test".to_string());
        c.push_op(crate::Opcode::Return, 0);

        let mut out = Vec::new();
        c.disassemble(&mut out).unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "== test ==\n\
             0000    0 RETURN\n\
             "
        )
    }

    #[test]
    fn test_const() {
        let mut c = Chunk::new("test".to_string());
        c.push_const(crate::Values::Double(1.23), 0);
        c.push_op(crate::Opcode::Return, 0);

        let mut out = Vec::new();
        c.disassemble(&mut out).unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "== test ==\n\
             0000    0 CONSTANT 000 '1.23'\n\
             0002    | RETURN\n\
             "
        )
    }
}
