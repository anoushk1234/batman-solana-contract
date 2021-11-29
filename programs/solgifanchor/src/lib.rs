use anchor_lang::prelude::*;

declare_id!("8Lc81NeVg8UUfQiM1dRsxRhfAn7exEixmXFUhR4vjnYs");

#[program]
pub mod mysolanagif {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        base_account.votes = 0;
        Ok(())
    }
    pub fn add_gif(ctx: Context<AddGif>, giflink: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;

        let item = ItemStruct {
            giflink: giflink,
            user_address: *base_account.to_account_info().key,
            votes: base_account.votes + 1,
        };
        base_account.gif_list.push(item);

        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn vote_gif(ctx: Context<VoteGif>, giflink: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;

        // let mut found = false;
        // Iterate over the gif_list and increment the votes if the giflink is found. add updated votes to gif_list and base_account
        for item in &mut base_account.gif_list {
            if item.giflink == giflink {
                item.votes += 1;
                // found = true;
            }
        }

        Ok(())
    }
    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
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
        )
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//make a struct to SendSol
#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    to:AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct VoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ItemStruct {
    pub giflink: String,
    pub user_address: Pubkey,
    pub votes: u64,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
    pub votes: u64,
}
