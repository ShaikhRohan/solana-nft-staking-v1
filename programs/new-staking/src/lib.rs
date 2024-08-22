use anchor_lang::prelude::*;

declare_id!("GPDqEdD9ZMH34g3i9YRGpQaa5BoANSbbkj63TuSECWrs");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake_nft(ctx: Context<StakeNFT>, nft_mint: Pubkey) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        let nft_owner = &ctx.accounts.nft_owner;

        if stake_account.nft_mint != nft_mint {
            return Err(ErrorCode::InvalidNFTMint.into());
        }

        // Update state to reflect that the NFT has been staked
        stake_account.owner = *nft_owner.key;

        Ok(())
    }

    pub fn unstake_nft(ctx: Context<UnstakeNFT>) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;

        // Ensure the caller is the owner of the staked NFT
        if stake_account.owner != *ctx.accounts.nft_owner.key {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Update state to reflect that the NFT has been unstaked
        stake_account.owner = Pubkey::default();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub nft_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnstakeNFT<'info> {
    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub nft_owner: Signer<'info>,
}

#[account]
pub struct StakeAccount {
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid NFT Mint")]
    InvalidNFTMint,
    #[msg("Unauthorized")]
    Unauthorized,
}
