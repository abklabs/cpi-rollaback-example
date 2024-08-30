use anchor_lang::prelude::*;
use caller_dummy::{
    cpi::{accounts::Setup, setup},
    program::CallerDummy,
    Dummy,
};

declare_id!("xNyCJ1gu9qgj134fgRggGDYA6LrmyA8RzwxwfaJy5v8");

#[program]
pub mod cpi_rollback_example {
    use std::result;

    use super::*;

    pub fn fallback(ctx: Context<Fallback>) -> Result<()> {
        let result = setup(CpiContext::new(
            ctx.accounts.dummy_program.to_account_info(),
            Setup {
                user: ctx.accounts.sender.to_account_info(),
                recipient: ctx.accounts.recipient.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        ));

        if result.is_err() {
            setup(CpiContext::new(
                ctx.accounts.dummy_program.to_account_info(),
                Setup {
                    user: ctx.accounts.fallback.to_account_info(),
                    recipient: ctx.accounts.recipient.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ))?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Fallback<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: AccountInfo<'info>,
    #[account(mut)]
    fallback: Signer<'info>,
    system_program: Program<'info, System>,
    dummy_program: Program<'info, CallerDummy>,
}
