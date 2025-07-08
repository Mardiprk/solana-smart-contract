use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake_history::StakeHistory;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod sysvar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let stake_history = &ctx.accounts.stake_history;
        msg!("Accessed Stake History sysvar account successfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub stake_history: Sysvar<'info, StakeHistory>,
}