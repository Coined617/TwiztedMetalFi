use anchor_lang::prelude::*;

declare_id!("TwiztedMetal11111111111111111111111111111111");

#[program]
pub mod twizted_metal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_tournament(
        ctx: Context<CreateTournament>,
        entry_fee_lamports: u64,
        max_players: u8,
        description: String,
    ) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        tournament.authority = *ctx.accounts.authority.key;
        tournament.entry_fee_lamports = entry_fee_lamports;
        tournament.max_players = max_players;
        tournament.description = description;
        tournament.player_count = 0;
        Ok(())
    }

    pub fn register_player(
        ctx: Context<RegisterPlayer>,
        zk_commitment: [u8; 32],
    ) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        require!(tournament.player_count < tournament.max_players, CustomError::TournamentFull);

        let entry_fee = tournament.entry_fee_lamports;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.player_authority.key,
            ctx.accounts.tournament_vault.key,
            entry_fee,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.player_authority.to_account_info(),
                ctx.accounts.tournament_vault.to_account_info(),
            ],
        )?;

        let player = &mut ctx.accounts.player;
        player.authority = *ctx.accounts.player_authority.key;
        player.zk_commitment = zk_commitment;
        player.tournament = ctx.accounts.tournament.key();
        tournament.player_count += 1;

        Ok(())
    }

    // Placeholder: Add CPI to Fluid, Solend, match result reporting, payout logic, etc.
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTournament<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 1 + 4 + 128 + 1)]
    pub tournament: Account<'info, Tournament>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub tournament_vault: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RegisterPlayer<'info> {
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,
    #[account(init, payer = player_authority, space = 8 + 32 + 32 + 32)]
    pub player: Account<'info, Player>,
    #[account(mut, signer)]
    pub player_authority: AccountInfo<'info>,
    #[account(mut)]
    pub tournament_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Tournament {
    pub authority: Pubkey,
    pub entry_fee_lamports: u64,
    pub max_players: u8,
    pub player_count: u8,
    pub description: String,
}

#[account]
pub struct Player {
    pub authority: Pubkey,
    pub zk_commitment: [u8; 32],
    pub tournament: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("Tournament is already full.")]
    TournamentFull,
}
