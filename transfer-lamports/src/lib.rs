#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_account::Account;
    use solana_address::Address;
    use solana_instruction::{AccountMeta, Instruction};

    #[test]
    fn test_transfer_lamports_success() {
        let program_id_keypair_bytes = std::fs::read("deploy/transfer-lamports-keypair.json")
            .unwrap()[..32]
            .try_into()
            .expect("slice with incorrect length");

        let program_id = Address::new_from_array(program_id_keypair_bytes);

        let sender = Address::new_unique();
        let receiver = Address::new_unique();

        let amount: u64 = 100;

        let instruction = Instruction::new_with_bytes(
            program_id,
            &amount.to_le_bytes(),
            vec![
                AccountMeta::new(sender, true),
                AccountMeta::new(receiver, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "deploy/transfer-lamports");

        let sender_account = Account {
            lamports: 1_000,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        };

        let receiver_account = Account {
            lamports: 500,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        };

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[(sender, sender_account), (receiver, receiver_account)],
            &[
                Check::success(),
                Check::account(&sender).lamports(900).build(),
                Check::account(&receiver).lamports(600).build(),
            ],
        );

        assert!(!result.program_result.is_err());
    }

    #[test]
    fn test_transfer_lamports_insufficient_funds() {
        let program_id_keypair_bytes = std::fs::read("deploy/transfer-lamports-keypair.json")
            .unwrap()[..32]
            .try_into()
            .expect("slice with incorrect length");

        let program_id = Address::new_from_array(program_id_keypair_bytes);

        let sender = Address::new_unique();
        let receiver = Address::new_unique();

        let amount: u64 = 2_000;

        let instruction = Instruction::new_with_bytes(
            program_id,
            &amount.to_le_bytes(),
            vec![
                AccountMeta::new(sender, true),
                AccountMeta::new(receiver, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "deploy/transfer-lamports");

        let sender_account = Account {
            lamports: 1_000,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        };

        let receiver_account = Account {
            lamports: 500,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        };

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[(sender, sender_account), (receiver, receiver_account)],
            &[Check::err(solana_program_error::ProgramError::Custom(1))],
        );

        assert!(result.program_result.is_err());
    }
}
