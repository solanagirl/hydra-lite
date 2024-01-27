use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::Fanout;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeFanoutArgs {
    pub bump_seed: u8,
    pub native_account_bump_seed: u8,
    pub name: String,
    pub total_shares: u64,
}

#[derive(Accounts)]
#[instruction(args: InitializeFanoutArgs)]
pub struct InitializeFanout<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
    init,
    space = 300,
    seeds = [b"fanout-config", args.name.as_bytes()],
    bump,
    payer = authority
    )]
    pub fanout: Account<'info, Fanout>,
    #[account(
    init,
    space = 1,
    seeds = [b"fanout-native-account", fanout.key().as_ref()],
    bump,
    payer = authority
    )
    ]
    /// CHECK: Native Account
    pub holding_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub membership_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
pub fn init(ctx: Context<InitializeFanout>, args: InitializeFanoutArgs) -> Result<()> {
    let membership_mint = &ctx.accounts.membership_mint;
    let fanout = &mut ctx.accounts.fanout;
    fanout.authority = ctx.accounts.authority.to_account_info().key();
    fanout.account_key = ctx.accounts.holding_account.to_account_info().key();
    fanout.name = args.name;
    fanout.total_shares = args.total_shares;
    fanout.total_available_shares = args.total_shares;
    fanout.total_inflow = 0;
    fanout.last_snapshot_amount = fanout.total_inflow;
    fanout.bump_seed = args.bump_seed;
    fanout.membership_mint = if membership_mint.key() == spl_token::native_mint::id() {
        None
    } else {
        Some(membership_mint.key())
    };
    fanout.membership_mint = None;
    fanout.total_staked_shares = None;

    Ok(())
}
