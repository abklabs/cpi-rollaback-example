use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::Space;

declare_id!("CbWDKSadpzo8kFJKGZtb4VYL7uzcPrisGffYsKhCNFhP");

#[program]
pub mod caller_dummy {
    use anchor_lang::solana_program::system_program;

    use super::*;

    pub fn setup(ctx: Context<Setup>) -> Result<()> {
        let from_pubkey = ctx.accounts.user.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let system_program_id = ctx.accounts.system_program.to_account_info();

        let transfer_context = CpiContext::new(
            system_program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(transfer_context, 10 * 1_000_000_000)
    }
}

#[account]
pub struct Dummy {}

impl Space for Dummy {
    const INIT_SPACE: usize = 0;
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    recipient: AccountInfo<'info>,
    system_program: Program<'info, System>,
}
