use borsh::{BorshDeserialize, BorshSerialize};

// Defines what the instruction is that gets called from the client app.
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum TokenInstruction {
    CreateToken,
    CreateTokenAccount,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}
