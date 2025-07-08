use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake_history::StakeHistory;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod sysvar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 42 });
        emit!(MySecondEvent {
            value: 25,
            message: *b"2026 is the year I'm gonna get a 100k job",
        });
        let _stake_history = &ctx.accounts.stake_history;
        msg!("Accessed Stake History sysvar account successfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub stake_history: Sysvar<'info, StakeHistory>,
}

#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct MySecondEvent {
    pub value: u64,
    pub message: [u8; 44],
}
