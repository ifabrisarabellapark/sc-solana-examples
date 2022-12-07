// DO NOT EDIT - automatically generated file
pub mod calculator_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        89u8, 162u8, 119u8, 162u8, 57u8, 107u8, 154u8, 54u8, 221u8, 229u8, 215u8, 68u8, 151u8,
        213u8, 56u8, 78u8, 229u8, 124u8, 124u8, 203u8, 80u8, 38u8, 222u8, 57u8, 128u8, 18u8, 80u8,
        254u8, 185u8, 58u8, 41u8, 118u8,
    ]);
    pub async fn create(
        client: &Client,
        i_init_message: String,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                calculator::instruction::Create {
                    init_message: i_init_message,
                },
                calculator::accounts::Create {
                    calculator: a_calculator,
                    user: a_user,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_ix(
        i_init_message: String,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: calculator::instruction::Create {
                init_message: i_init_message,
            }
            .data(),
            accounts: calculator::accounts::Create {
                calculator: a_calculator,
                user: a_user,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn add(
        client: &Client,
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                calculator::instruction::Add { n1: i_n1, n2: i_n2 },
                calculator::accounts::Addition {
                    calculator: a_calculator,
                },
                signers,
            )
            .await?)
    }
    pub fn add_ix(
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: calculator::instruction::Add { n1: i_n1, n2: i_n2 }.data(),
            accounts: calculator::accounts::Addition {
                calculator: a_calculator,
            }
            .to_account_metas(None),
        }
    }
    pub async fn multiply(
        client: &Client,
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                calculator::instruction::Multiply { n1: i_n1, n2: i_n2 },
                calculator::accounts::Multiplication {
                    calculator: a_calculator,
                },
                signers,
            )
            .await?)
    }
    pub fn multiply_ix(
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: calculator::instruction::Multiply { n1: i_n1, n2: i_n2 }.data(),
            accounts: calculator::accounts::Multiplication {
                calculator: a_calculator,
            }
            .to_account_metas(None),
        }
    }
    pub async fn subtract(
        client: &Client,
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                calculator::instruction::Subtract { n1: i_n1, n2: i_n2 },
                calculator::accounts::Subtraction {
                    calculator: a_calculator,
                },
                signers,
            )
            .await?)
    }
    pub fn subtract_ix(
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: calculator::instruction::Subtract { n1: i_n1, n2: i_n2 }.data(),
            accounts: calculator::accounts::Subtraction {
                calculator: a_calculator,
            }
            .to_account_metas(None),
        }
    }
    pub async fn divide(
        client: &Client,
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                calculator::instruction::Divide { n1: i_n1, n2: i_n2 },
                calculator::accounts::Division {
                    calculator: a_calculator,
                },
                signers,
            )
            .await?)
    }
    pub fn divide_ix(
        i_n1: i64,
        i_n2: i64,
        a_calculator: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: calculator::instruction::Divide { n1: i_n1, n2: i_n2 }.data(),
            accounts: calculator::accounts::Division {
                calculator: a_calculator,
            }
            .to_account_metas(None),
        }
    }
}
