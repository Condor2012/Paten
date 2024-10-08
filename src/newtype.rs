use std::fmt;

fn main() {
    let buf = Buffer(std::array::from_fn(|idx| idx as u8));

    println!("{}", buf);
}

struct Buffer([u8; 16]);

impl Buffer {
    fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Buffer: 0x")?;
        for i in self.0 {
            write!(f, "{i:02x}")?;
        }

        Ok(())
    }
}
