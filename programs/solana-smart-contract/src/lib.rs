use anchor_lang::prelude::*;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod solana_smart_contract {
    use super::*;

    pub fn add_two_numbers(ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let result = calculate::add(x, y);
        msg!("Addition of  {} + {} : {}", x, y, result);
        Ok(())
    }

}

mod calculate {
    pub fn add(x: u64, y: u64) -> u64 {
        x + y
    }
}

#[derive(Accounts)]
pub struct Initialize {}