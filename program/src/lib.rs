pub mod error;
pub mod processors;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use processors::*;

declare_id!("Bte42WiWWCy6uWtMqRZ13er18p9Kden4a29DNrEUZ8sh");
#[program]
pub mod hydra {
    use super::*;

    pub fn process_init(ctx: Context<InitializeFanout>, args: InitializeFanoutArgs) -> Result<()> {
        init(ctx, args)
    }

    pub fn process_init_for_mint(
        ctx: Context<InitializeFanoutForMint>,
        bump_seed: u8,
    ) -> Result<()> {
        init_for_mint(ctx, bump_seed)
    }

    pub fn process_add_member_wallet(
        ctx: Context<AddMemberWallet>,
        args: AddMemberArgs,
    ) -> Result<()> {
        add_member_wallet(ctx, args)
    }

    pub fn process_distribute_wallet(
        ctx: Context<DistributeWalletMember>,
        distribute_for_mint: bool,
    ) -> Result<()> {
        distribute_for_wallet(ctx, distribute_for_mint)
    }

    pub fn process_transfer_shares(ctx: Context<TransferShares>, shares: u64) -> Result<()> {
        transfer_shares(ctx, shares)
    }

    pub fn process_remove_member(ctx: Context<RemoveMember>) -> Result<()> {
        remove_member(ctx)
    }
}
