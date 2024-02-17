use std::io::{Error, Write};
use std::vec::Vec;

#[derive(Debug)]
pub struct Chunk {
    name: String,
    // We're packing opcodes and arguments in this vector of binary data.
    // Since the data won't overlap with the Opcode enum, that needs some
    // wrangling to play nice.
    code: Vec<u8>,
}

impl Chunk {
    fn new(s: String) -> Self {
        return Chunk {
            name: s,
            code: Vec::new(),
        };
    }

    fn push(&mut self, c: super::Opcode) {
        self.code.push(c as u8)
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
        match self.code.get(offset) {
            Some(&u) => match super::Opcode::try_from(u) {
                Ok(super::Opcode::Return) => {
                    simple_instruction(w, "RETURN", offset)?;
                    Ok(offset + 1)
                }
                _ => {
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
}

fn simple_instruction(w: &mut impl Write, name: &str, offset: usize) -> Result<usize, Error> {
    writeln!(w, "{}", name)?;
    Ok(offset + 1)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_chunk() {
        let mut c = Chunk::new("test".to_string());
        c.push(crate::Opcode::Return);

        let mut out = Vec::new();
        c.disassemble(&mut out).unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "== test ==\n\
             0000 RETURN\n\
             "
        )
    }
}
