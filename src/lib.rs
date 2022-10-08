use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

use anchor_spl::token::mint_to;

use mpl_token_metadata::instruction::create_metadata_accounts_v2;

use anchor_spl::{
    token,
    token::{Burn, Mint, MintTo, Token, TokenAccount, Transfer},
};
declare_id!("7PwsaByb6Uvzuc1uUUbJrAdtL7wtRXKWHsVtT3nbSDvs");

#[program]
mod my_mora_program {
    use super::*;

    pub fn buy_box(ctx: Context<TransferTokens>, amount: u64,orderid:String) -> Result<()> {
        msg!("BuyBox");
        let sender = &ctx.accounts.sender;
        let sender_tokens = &ctx.accounts.sender_tokens;
        let recipient_tokens = &ctx.accounts.recipient_tokens;
        let token_program = &ctx.accounts.token_program;
        let transfer_instruction = Transfer {
            from: sender_tokens.to_account_info(),
            to: recipient_tokens.to_account_info(),
            authority: sender.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        msg!(
            "Roshambo {{method:\"BuyBox\",tokenAddress:\"{}\",from:\"{}\",to:\"{}\",authorityAddress:\"{}\",amount:\"{}\",orderid:\"{}\"}}",
            ctx.accounts.mint.key(),
            ctx.accounts.sender_tokens.owner,
            ctx.accounts.recipient_tokens.owner,
            ctx.accounts.sender.key(),
            amount,
            orderid
        );
        Ok(())
    }

    pub fn buy_box_sol(ctx: Context<TransferSol>, amount: u64,orderid:String) -> Result<()> {
        msg!("BuyBox");
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )?;
        msg!(
            "Roshambo {{method:\"BuyBox\",tokenAddress:\"{}\",from:\"{}\",to:\"{}\",authorityAddress:\"{}\",amount:\"{}\",orderid:\"{}\"}}",
            "SOLANA",
            ctx.accounts.from.key(),
            ctx.accounts.to.key(),
            ctx.accounts.from.key(),
            amount,
            orderid
        );
        Ok(())
    }

    
    pub fn recharge_balance_user(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        msg!("RechargeBalanceUser");
        transfer_tokens(ctx, amount, "RechargeBalanceUser".to_string())?;
        Ok(())
    }

    pub fn recharge_balance_user_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        msg!("RechargeBalanceUserSol");
        transfer_tokens_sol(ctx,amount,"RechargeBalanceUser".to_string())?;
        Ok(())
    }

    pub fn recharge_balance_pool(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        msg!("RechargeBalancePool");
        transfer_tokens(ctx, amount, "RechargeBalancePool".to_string())?;
        Ok(())
    }
    pub fn recharge_balance_pool_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        msg!("RechargeBalancePool");
        transfer_tokens_sol(ctx,amount,"RechargeBalancePool".to_string())?;
        Ok(())
    }

    pub fn pool_stack_test(ctx: Context<TransferTokens>, amount: u64,types :String) -> Result<()> {
        msg!("poolStackTest");
        let sender = &ctx.accounts.sender;
        let sender_tokens = &ctx.accounts.sender_tokens;
        let recipient_tokens = &ctx.accounts.recipient_tokens;
        let token_program = &ctx.accounts.token_program;
        let transfer_instruction = Transfer {
            from: sender_tokens.to_account_info(),
            to: recipient_tokens.to_account_info(),
            authority: sender.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction);
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!(
            "Roshambo {{method:\"PoolStack\",tokenAddress:\"{}\",from:\"{}\",to:\"{}\",authorityAddress:\"{}\",amount:\"{}\",type:\"{}\"}}",
            ctx.accounts.mint.key(),
            ctx.accounts.sender_tokens.owner,
            ctx.accounts.recipient_tokens.owner,
            ctx.accounts.sender.key(),
            amount,
            types,
        );
        Ok(())
    }

    pub fn pool_stack_test_sol(ctx: Context<TransferSol>, amount: u64,types :String) -> Result<()> {
        msg!("poolStackTestSol");
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )?;

        msg!(
            "Roshambo {{method:\"PoolStack\",tokenAddress:\"{}\",from:\"{}\",to:\"{}\",authorityAddress:\"{}\",amount:\"{}\",type:\"{}\"}}",
            "SOLANA",
            ctx.accounts.from.key(),
            ctx.accounts.to.key(),
            ctx.accounts.from.key(),
            amount,
            types,
        );
        Ok(())
    }

    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
        method: String,
    ) -> Result<()> {
        let sender = &ctx.accounts.sender;
        let sender_tokens = &ctx.accounts.sender_tokens;
        let recipient_tokens = &ctx.accounts.recipient_tokens;
        let token_program = &ctx.accounts.token_program;
        let transfer_instruction = Transfer {
            from: sender_tokens.to_account_info(),
            to: recipient_tokens.to_account_info(),
            authority: sender.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        msg!(
            "Roshambo {{method:\"{}\",tokenAddress:\"{}\",from:\"{}\",to:\"{}\",authorityAddress:\"{}\",amount:\"{}\"}}",
            method,
            ctx.accounts.mint.key(),
            ctx.accounts.sender_tokens.owner,
            ctx.accounts.recipient_tokens.owner,
            ctx.accounts.sender.key(),
            amount
        );

        Ok(())
    }

    pub fn synthesis_card(ctx: Context<Synthesis>) -> Result<()> {
        let token_program = &mut ctx.accounts.token_program;

        let mint3 = &mut ctx.accounts.mint3;
        let from3 = &mut ctx.accounts.from3;
        let authority3 = &mut ctx.accounts.authority3;

        let mint2 = &mut ctx.accounts.mint2;
        let from2 = &mut ctx.accounts.from2;
        let authority2 = &mut ctx.accounts.authority2;

        let mint1 = &mut ctx.accounts.mint1;
        let from1 = &mut ctx.accounts.from1;
        let authority1 = &mut ctx.accounts.authority1;



        token::burn(
            into_burn_context(
                token_program.to_account_info(),
                mint3.to_account_info(),
                from3.to_account_info(),
                authority3.to_account_info(),
            ),
            1,
        )?;

        token::burn(
            into_burn_context(
                token_program.to_account_info(),
                mint2.to_account_info(),
                from2.to_account_info(),
                authority2.to_account_info(),
            ),
            1,
        )?;


        token::burn(
            into_burn_context(
                token_program.to_account_info(),
                mint1.to_account_info(),
                from1.to_account_info(),
                authority1.to_account_info(),
            ),
            1,
        )?;



        Ok(())
    }

    pub fn synthesis_card_test(
        ctx: Context<SynthesisTest>,
        creator_key: Pubkey,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {


        msg!("synthesis card test:");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = mint_to(cpi_ctx, 1);
        if let Err(_) = result {
            return Err(error!(ErrorCode::MintFailed));
        }
        msg!("Token minted !!!");

        msg!("Metadata account creating");
        let accounts = vec![
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        let result = invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                name,
                symbol,
                uri,
                Some(creators),
                1,
                true,
                false,
                None,
                None,
            ),
            &accounts,
        );

        if let Err(_) = result {
            return Err(error!(ErrorCode::MetadataCreateFailed));
        }
        msg!("mint !!!{:?}", ctx.accounts.mint.key());

        msg!("token_account !!!{:?}", ctx.accounts.token_account.key());

        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnTokens>,orderid:String) -> Result<()> {
        let burn_ctx3 = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::burn(burn_ctx3, 1)?;

        msg!(
            "Roshambo {{method:\"BurnToken\",tokenAddress:\"{}\",userTokenAccount:\"{}\",userAddress:\"{}\",orderid:\"{}\"}}",
            ctx.accounts.mint.key(),
            ctx.accounts.from.key(),
            ctx.accounts.authority.key(),
            orderid
        );

        Ok(())
    }

    pub fn pool_stack(
        ctx: Context<Pool>,
        creator_key: Pubkey,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Nft token minting:");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = mint_to(cpi_ctx, 1);
        if let Err(_) = result {
            return Err(error!(ErrorCode::MintFailed));
        }
        msg!("Token minted !!!");

        msg!("Metadata account creating");
        let accounts = vec![
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        let result = invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                name,
                symbol,
                uri,
                Some(creators),
                1,
                true,
                false,
                None,
                None,
            ),
            &accounts,
        );

        if let Err(_) = result {
            return Err(error!(ErrorCode::MetadataCreateFailed));
        }
        msg!(
            "Roshambo {{method:\"mint_nft\",mintAddress:\"{}\",userAddress:\"{}\",authorityAddress:\"{}\",metadataAddress:\"{}\"}}",
            ctx.accounts.mint.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.metadata.key(),
        );
        Ok(())
    }


    pub fn transfer_tokens_sol(
        ctx: Context<TransferSol>,
        amount: u64,
        method: String,
    ) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )?;
        msg!(
            "Roshambo {{method:\"{}\",tokenAddress:\"{}\",from:\"{}\",to:\"{}\",authorityAddress:\"{}\",amount:\"{}\"}}",
            method,
            "SOLANA",
            ctx.accounts.from.key(),
            ctx.accounts.to.key(),
            ctx.accounts.from.key(),
            amount
        );

        Ok(())
    }

    
    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        name: String,
        symbol: String,
        uri: String,
        
    ) -> Result<()> {
        msg!("Nft token minting:");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = mint_to(cpi_ctx, 1);
        if let Err(_) = result {
            return Err(error!(ErrorCode::MintFailed));
        }
        msg!("Token minted !!!");

        msg!("Metadata account creating");
        let accounts = vec![
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        let result = invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                name,
                symbol,
                uri,
                Some(creators),
                1,
                true,
                false,
                None,
                None,
            ),
            &accounts,
        );

        if let Err(_) = result {
            return Err(error!(ErrorCode::MetadataCreateFailed));
        }
        msg!(
            "Roshambo {{method:\"mintNFT\",mintAddress:\"{}\",userAddress:\"{}\",authorityAddress:\"{}\",metadataAddress:\"{}\"}}",
            ctx.accounts.mint.key(),
            ctx.accounts.token_account.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            
        );
        Ok(())
    }


    pub fn mint_nft2(
        ctx: Context<MintNFTTest>,
        nft_type: String,
    ) -> Result<()> {
        msg!("Nft token minting:");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = mint_to(cpi_ctx, 1);
        if let Err(_) = result {
            return Err(error!(ErrorCode::MintFailed));
        }
        msg!("Token minted !!!");

        msg!(
            "Roshambo {{method:\"mintNFT2\",mintAddress:\"{}\",userAddress:\"{}\",authorityAddress:\"{}\",nftType:\"{}\"}}",
            ctx.accounts.mint.key(),
            ctx.accounts.token_account.key(),
            ctx.accounts.mint_authority.key(),
            nft_type,
        );
        Ok(())
    }

}

fn into_burn_context<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    from: AccountInfo<'info>,
    authority: AccountInfo<'info>,
) -> CpiContext<'info, 'info, 'info, 'info, Burn<'info>> {
    let cpi_accounts = Burn {
        mint: mint,
        from: from,
        authority: authority,
    };
    CpiContext::new(token_program, cpi_accounts)
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct MintNFTTest<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,


}

#[derive(Accounts)]
pub struct Pool<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,

    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    pub system_program_token: Program<'info, System>,
    pub token_program_token: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Mint failed!")]
    MintFailed,

    #[msg("Metadata account create failed!")]
    MetadataCreateFailed,
}
#[derive(Accounts)]
pub struct Synthesis<'info> {
    #[account(mut)]
    pub mint3: Account<'info, Mint>,
    #[account(mut)]
    pub from3: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority3: Signer<'info>,

    #[account(mut)]
    pub mint1: Account<'info, Mint>,
    #[account(mut)]
    pub from1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority1: Signer<'info>,

    #[account(mut)]
    pub mint2: Account<'info, Mint>,
    #[account(mut)]
    pub from2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority2: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SynthesisTest<'info> {
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct TransferSol<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
// #[derive(Accounts)]
// pub struct TransferWrapper<'info> {
//     pub sender: Signer<'info>,
//     #[account(mut)]
//     pub sender_token: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub receiver_token: Account<'info, TokenAccount>,
//     pub mint: Account<'info, Mint>,
//     pub token_program: Program<'info, Token>,
// }
// impl<'info> TransferWrapper<'info> {
//     fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
//         CpiContext::new(
//             self.token_program.to_account_info(),
//             Transfer {
//                 from: self.sender_token.to_account_info(),
//                 to: self.receiver_token.to_account_info(),
//                 authority: self.sender.to_account_info(),
//             },
//         )
//     }
// }
