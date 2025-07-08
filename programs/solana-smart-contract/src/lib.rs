use anchor_lang::prelude::*;

declare_id!("FXF4PruD9YkoGuiGbRD8yAQSShtQDeYrhYYWZy4Nm7bS");

#[program]
pub mod solana_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        some_internal_function::internal_function();
        Ok(())
    }
    pub mod some_internal_function {
        pub fn internal_function() -> u64 {
            2
        }
    }
}

mod do_something { 
    use crate::func_test;

    pub fn inside_some_something() {
        func_test::some_internal_function::internal_function();
    }
}

#[derive(Accounts)]
pub struct Initialize {}