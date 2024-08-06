#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey)]
pub struct InitializeUserProfile<'info> {
    //   PDA
    #[account(
        init,
        seeds = [b"FarmingProfile".as_ref(),lp_mint.key().as_ref(),user.key().as_ref()],
        bump,
        payer = user,
        space = size_of::<UserLpStakingProfile>() + 16
    )]
    pub user_farming_profile: Account<'info, UserLpStakingProfile>,
    #[account(address= lp_mint_address)]
    pub lp_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey)]
pub struct CreateLpTokenBag<'info> {
    // PDA
    #[account(
        init,
        payer = payer,
        seeds = [ lp_mint.key().as_ref() ],
        bump,
        token::mint = lp_mint,
        // PDA authority!
        token::authority = program_lp_token_bag,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,

    #[account(
        address = lp_mint_address,
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

    pub fn create_lp_token_bag(
        ctx: Context<CreateLpTokenBag>,
        lp_mint_address: Pubkey,
    ) -> Result<()> {
        Ok(())
    }
