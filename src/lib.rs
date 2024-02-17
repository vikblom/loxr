#![allow(dead_code)]

mod chunk;

#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    Return,
}

impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            // Nice...
            x if x == Opcode::Return as u8 => Ok(Opcode::Return),
            _ => Err(()),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        op as u8
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_repr() {
        // let op = 1u8 as Opcode;
        println!("{:?}", Opcode::Return as u8);
        println!("{:?}", Opcode::try_from(1u8));
    }
}
