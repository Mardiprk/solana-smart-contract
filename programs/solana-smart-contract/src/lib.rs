use anchor_lang::prelude::*;
use chrono;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod solana_smart_contract {
    use super::*;

    pub fn add_two_numbers(ctx: Context<Initialize>) -> Result<()> {
        let rent_var = Rent::get()?;
        msg("Rent {:?}", rent_var);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}