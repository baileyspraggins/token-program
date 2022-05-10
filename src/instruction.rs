use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum TokenInstruction {
    CreateToken,
    CreateTokenAccount,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}
