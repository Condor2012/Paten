use std::fs::File;
use std::io::Read;
use std::net::TcpStream;

fn main() {
    let mut manual_constructed = Transaction {
        hash: [0; 32],
        from: String::from("John"),
        to: String::default(),
        value: 100,
        nonce: 0,
        input: vec![1, 2, 3],
    };
    manual_constructed.hash = manual_constructed.calc_hash();

    let constructed_with_builder = TransactionBuilder::new()
        .with_from("John".into())
        .with_value(100)
        .with_input(vec![1, 2, 3])
        .build();
}

struct Transaction {
    pub hash: [u8; 32],
    pub from: String,
    pub to: String,
    pub value: u64,
    pub nonce: u64,
    pub input: Vec<u8>,
}

impl Transaction {
    fn calc_hash(&self) -> [u8; 32] {
        // calculate hash based on fields
        [0; 32]
    }
}

struct TransactionBuilder {
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: Option<u64>,
    pub nonce: Option<u64>,
    pub input: Option<Vec<u8>>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
            value: None,
            nonce: None,
            input: None,
        }
    }

    pub fn with_from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }

    pub fn with_value(mut self, value: u64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    pub fn with_input(mut self, input: Vec<u8>) -> Self {
        self.input = Some(input);
        self
    }

    pub fn build(self) -> Transaction {
        let mut tx = Transaction {
            hash: [0; 32],
            from: self.from.unwrap_or_default(),
            to: self.to.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            nonce: self.nonce.unwrap_or_else(|| next_nonce()),
            input: self.input.unwrap_or_default(),
        };

        tx.hash = tx.calc_hash();
        tx
    }
}

fn next_nonce() -> u64 {
    // calculate next nonce
    0
}