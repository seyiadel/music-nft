use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};

declare_id!("CKrPfTowCdd7Jh9oKLZQyUUVkwJv3vNV1skXfPWjHHRm");

#[program]
pub mod music_program_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let music_platform = &mut ctx.accounts.music_platform;
        music_platform.authority = ctx.accounts.authority.key();
        music_platform.total_nfts = 0;
        Ok(())

    }

    pub fn mint_nft(ctx: Context<MintNFT>, title: String, artist: String) -> Result<()> {
        let music_platform = &mut ctx.accounts.music_platform;
        let nft = &mut ctx.accounts.nft;

        nft.title = title;
        nft.artist = artist;
        nft.owner = ctx.accounts.owner.key();
        nft.is_staked = false;
        nft.play_count = 0;

        music_platform.total_nfts += 1;

        // Mint 1 token to represent the NFT
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            1,
        )?;

        Ok(())
    }

    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.is_staked = true;
        Ok(())
    }

    pub fn unstake_nft(ctx: Context<UnstakeNFT>) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.is_staked = false;
        Ok(())
    }

    pub fn play_track(ctx: Context<PlayTrack>) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        require!(nft.is_staked, MusicError::NFTNotStaked);
        nft.play_count += 1;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub music_platform: Account<'info, MusicPlatform>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub music_platform: Account<'info, MusicPlatform>,
    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 1 + 8)]
    pub nft: Account<'info, MusicNFT>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(mut, has_one = owner)]
    pub nft: Account<'info, MusicNFT>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnstakeNFT<'info> {
    #[account(mut, has_one = owner)]
    pub nft: Account<'info, MusicNFT>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PlayTrack<'info> {
    #[account(mut)]
    pub nft: Account<'info, MusicNFT>,
    pub user: Signer<'info>,
}


#[account]
pub struct MusicNFT{
    pub artist:String,
    pub owner:Pubkey,
    pub is_staked:bool,
    pub play_count:u64
}

#[account]
pub struct MusicPlatform {
    pub authority: Pubkey,
    pub total_nfts: u64,
}

#[error_code]
pub enum MusicError {
    #[msg("NFT must be staked to play")]
    NFTNotStaked,
}


