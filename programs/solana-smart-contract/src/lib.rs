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
        data_account.owner = ctx.accounts.user.key();
        data_account.created_at = Clock::get()?.unix_timestamp;
        data_account.updated_at = Clock::get()?.unix_timestamp;
        data_account.update_count = 0;
        data_account.is_active = true;
        data_account.bump = ctx.bumps.data;

        msg!("They said! {}", message);
        Ok(())
    }

    pub fn update_message(ctx: Context<Update>, new_message: String) -> Result<()> {
        if new_message.is_empty() {
            return Err(error!(ErrorMessage::EmptyMessage));
        }

        let data_account = &mut ctx.accounts.data;
        
        // Check if account is active
        if !data_account.is_active {
            return Err(error!(ErrorMessage::AccountDeactivated));
        }

        data_account.message = new_message.clone();
        data_account.updated_at = Clock::get()?.unix_timestamp;
        data_account.update_count += 1;

        msg!("New Message: {}", new_message);

        Ok(())
    }

    pub fn delete_message(ctx: Context<Delete>) -> Result<()> {
        let data_account = &mut ctx.accounts.data;
        
        if !data_account.is_active {
            return Err(error!(ErrorMessage::AccountDeactivated));
        }

        data_account.message = String::new();
        data_account.updated_at = Clock::get()?.unix_timestamp;
        data_account.update_count += 1;

        msg!("Message deleted by owner");
        Ok(())
    }

    pub fn deactivate_account(ctx: Context<Delete>) -> Result<()> {
        let data_account = &mut ctx.accounts.data;
        
        if !data_account.is_active {
            return Err(error!(ErrorMessage::AccountDeactivated));
        }

        data_account.is_active = false;
        data_account.updated_at = Clock::get()?.unix_timestamp;

        msg!("Account deactivated by owner");
        Ok(())
    }

    pub fn reactivate_account(ctx: Context<Delete>) -> Result<()> {
        let data_account = &mut ctx.accounts.data;
        
        if data_account.is_active {
            return Err(error!(ErrorMessage::AccountAlreadyActive));
        }

        data_account.is_active = true;
        data_account.updated_at = Clock::get()?.unix_timestamp;

        msg!("Account reactivated by owner");
        Ok(())
    }

    pub fn get_message_stats(ctx: Context<GetStats>) -> Result<()> {
        let data_account = &ctx.accounts.data;
        
        msg!("Message Stats:");
        msg!("Owner: {}", data_account.owner);
        msg!("Message: {}", data_account.message);
        msg!("Created: {}", data_account.created_at);
        msg!("Last Updated: {}", data_account.updated_at);
        msg!("Update Count: {}", data_account.update_count);
        msg!("Is Active: {}", data_account.is_active);
        
        Ok(())
    }

    pub fn append_message(ctx: Context<Update>, additional_text: String) -> Result<()> {
        if additional_text.is_empty() {
            return Err(error!(ErrorMessage::EmptyMessage));
        }

        let data_account = &mut ctx.accounts.data;
        
        if !data_account.is_active {
            return Err(error!(ErrorMessage::AccountDeactivated));
        }

        // Check if combined message would exceed max length
        let combined_length = data_account.message.len() + additional_text.len() + 1; // +1 for space
        if combined_length > 200 {
            return Err(error!(ErrorMessage::MessageTooLong));
        }

        if !data_account.message.is_empty() {
            data_account.message.push(' ');
        }
        data_account.message.push_str(&additional_text);
        data_account.updated_at = Clock::get()?.unix_timestamp;
        data_account.update_count += 1;

        msg!("Appended text: {}", additional_text);
        msg!("New full message: {}", data_account.message);

        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_owner: Pubkey) -> Result<()> {
        let data_account = &mut ctx.accounts.data;
        
        if !data_account.is_active {
            return Err(error!(ErrorMessage::AccountDeactivated));
        }

        if new_owner == data_account.owner {
            return Err(error!(ErrorMessage::SameOwner));
        }

        let old_owner = data_account.owner;
        data_account.owner = new_owner;
        data_account.updated_at = Clock::get()?.unix_timestamp;

        msg!("Ownership transferred from {} to {}", old_owner, new_owner);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 4 + 200 + 8 + 8 + 4 + 1 + 1, // discriminator + pubkey + string + timestamps + counter + bool + bump
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
        bump = data.bump,
        constraint = data.owner == user.key() @ ErrorMessage::UnauthorizedAccess
    )]
    pub data: Account<'info, DataAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(
        mut,
        seeds = [b"data", user.key().as_ref()],
        bump = data.bump,
        constraint = data.owner == user.key() @ ErrorMessage::UnauthorizedAccess
    )]
    pub data: Account<'info, DataAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetStats<'info> {
    #[account(
        seeds = [b"data", owner.key().as_ref()],
        bump = data.bump
    )]
    pub data: Account<'info, DataAccount>,

    /// CHECK: This is safe because we're only reading the account
    pub owner: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [b"data", user.key().as_ref()],
        bump = data.bump,
        constraint = data.owner == user.key() @ ErrorMessage::UnauthorizedAccess
    )]
    pub data: Account<'info, DataAccount>,

    pub user: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub message: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub update_count: u32,
    pub is_active: bool,
    pub bump: u8,
}

#[error_code]
pub enum ErrorMessage {
    #[msg("Message cannot be empty")]
    EmptyMessage,
    #[msg("Empty String")]
    EmptyString,
    #[msg("Account is deactivated")]
    AccountDeactivated,
    #[msg("Account is already active")]
    AccountAlreadyActive,
    #[msg("Unauthorized access - you are not the owner")]
    UnauthorizedAccess,
    #[msg("Message too long - would exceed maximum length")]
    MessageTooLong,
    #[msg("Cannot transfer to the same owner")]
    SameOwner,
}