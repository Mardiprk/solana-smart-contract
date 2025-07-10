use anchor_lang::prelude::*;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod compute_unit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + size_of::<MyStorage>(),
        seeds = [],
        bump,
    )]
    pub my_storeage: Account<'info, MyStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}
