use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account does not belong to this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if instruction_data.len() > account.data_len() {
        msg!("Journal entry is too long");
        return Err(ProgramError::InvalidInstructionData);
    }

    let mut data = account.try_borrow_mut_data()?;
    data[..instruction_data.len()].copy_from_slice(&instruction_data);
    Ok(())
}
