use anchor_lang::prelude::*;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod solana_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        let data_account = &mut ctx.accounts.data;
        
        if message.is_empty() {
            return Err(ErrorMessage::EmptyMessage.into());
        }
        
        data_account.message = message.clone();
        msg!("They said! {}", message);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 200 
    )]
    pub data: Account<'info, DataAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub message: String,
}

#[error_code]
pub enum ErrorMessage {
    #[msg("Message cannot be empty")]
    EmptyMessage,
}