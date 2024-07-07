use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Mint, MintTo, Transfer};

declare_id!("DMZkcTtqUCv3FmDoDhhHdnyDxFuLJzidZPSgxzS8MNG9");

#[program]
pub mod stake_program {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>, decimals: u8) -> Result<()> {
        msg!("Token mint initialized: {}", ctx.accounts.token_mint.key());
        msg!("The decimals enterd: {}", decimals);
        Ok(())
    }

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        msg!("Staking Pool initialized...");
        msg!("Staking Pool Token account: {}", ctx.accounts.pool_token_account.key());
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, amount: u64) -> Result<()> {
        let token_bump = ctx.bumps.token_authority;
        let token_seeds = &["token-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];

        msg!("Airdropping {} tokens...", amount);
        let mint_to_ctx = ctx.accounts.mint_to_ctx().with_signer(signer);
        let _ = token::mint_to(mint_to_ctx, amount);

        msg!("Airdrop Complete!");
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {

        let stake_instruction = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            stake_instruction,
        );

        msg!("Staking {} tokens...", amount);
        ctx.accounts.user_state_account.amount_staked += amount;
        token::transfer(cpi_ctx, amount)?;

        

        msg!("Stake Complete!");
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {

        let stakedamm = ctx.accounts.user_state_account.amount_staked;
        if amount > stakedamm {
            msg!("Cannot Unstake more than Staked!..");
            Ok(())
        }
        else {
            let pool_bump = ctx.bumps.pool_authority;
            let pool_seeds = &["pool-authority".as_bytes(), &[pool_bump]];
            let signer = &[&pool_seeds[..]];

            msg!("Unstaking {} tokens...", amount);
            ctx.accounts.user_state_account.amount_staked -= amount;

            let unstake_tokens = ctx.accounts.unstake_tokens().with_signer(signer);
            let _ = token::transfer(unstake_tokens, amount);
            
            msg!("Unstake Complete!");
            Ok(())
        }
        
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct InitializeMint<'info> {
    #[account(
        init, 
        mint::authority = token_authority,
        mint::decimals = decimals,
        seeds = ["token-mint".as_bytes()],
        bump,
        payer = payer)]
    pub token_mint: Account<'info, Mint>,

    #[account(seeds = ["token-authority".as_bytes()], bump)]
    /// CHECK: This is the mint_authority
    pub token_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitializePool<'info> {

    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(seeds = ["pool-authority".as_bytes()], bump)]
    /// CHECK: This is the pool_authority
    pub pool_authority: AccountInfo<'info>,

    #[account(
        init,
        token:: mint = token_mint,
        token:: authority = pool_authority,
        payer = payer
    )]    
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["token-authority".as_bytes()], bump)]
    /// CHECK: This is the mint authority
    pub token_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        token:: mint = token_mint,
        token:: authority = user,
        payer = user
    )]    
    pub user_token_account: Account<'info, TokenAccount>,


    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>

}

impl <'info> Airdrop <'info> {
    pub fn mint_to_ctx(&self) -> CpiContext<'_,'_,'_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.token_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    /// CHECK: This is the mint
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["pool-authority".as_bytes()], bump)]
    /// CHECK: This is the pool_authority
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token:: mint = token_mint,
        token:: authority = user,
    )]    
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        init, 
        seeds = ["state-account".as_bytes()], 
        bump, 
        payer=user,
        space = 8 + 8
        )]
    pub user_state_account: Account<'info, UserStateAccount>,

    #[account(
        mut,
        token:: mint = token_mint,
        token:: authority = pool_authority
    )]    
    pub pool_token_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    /// CHECK: This is the mint
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["pool-authority".as_bytes()], bump)]
    /// CHECK: This is the pool_authority
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token:: mint = token_mint,
        token:: authority = user,
    )]    
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut, 
        seeds = ["state-account".as_bytes()], 
        bump,
        )]
    pub user_state_account: Account<'info, UserStateAccount>,

    #[account(
        mut,
        token:: mint = token_mint,
        token:: authority = pool_authority
    )]    
    pub pool_token_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
    
}

impl <'info> Unstake <'info> {
    pub fn unstake_tokens(&self) -> CpiContext<'_,'_,'_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.pool_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.pool_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[account]
pub struct UserStateAccount {
    pub amount_staked: u64,
}