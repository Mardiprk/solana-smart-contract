use anchor_lang::prelude::*;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod solana_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        if message.is_empty() {
            return Err(error!(ErrorMessage::EmptyMessage));
        }

        let data_account = &mut ctx.accounts.data;
        data_account.message = message.clone();
        data_account.bump = ctx.bumps.data;

        msg!("They said! {}", message);
        Ok(())
    }

    pub fn update_message(ctx: Context<Update>, new_message: String) -> Result<()> {
        if new_message.is_empty() {
            return Err(error!(ErrorMessage::EmptyMessage));
        }

        let data_account = &mut ctx.accounts.data;
        data_account.message = new_message.clone();

        msg!("New Message: {}", new_message);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 200,
        seeds = [b"data", user.key().as_ref()],
        bump
    )]
    pub data: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [b"data", user.key().as_ref()],
        bump = data.bump
    )]
    pub data: Account<'info, DataAccount>,

    pub user: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub message: String,
    pub bump: u8,
}

#[error_code]
pub enum ErrorMessage {
    #[msg("Message cannot be empty")]
    EmptyMessage,
    #[msg("Empty String")]
    EmptyString,
}
