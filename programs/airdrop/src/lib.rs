use anchor_lang::prelude::*;

declare_id!("uGSfh7D9AZmECS3NfCnvMvuktMktCreJf9P7t9Z8qmh");

#[program]
pub mod airdrop {
    use anchor_lang::solana_program::system_instruction;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let wallet = &ctx.accounts.vault_wallet;
        if **owner.try_borrow_lamports()? < amount {
            return err!(MyError::InsufficientSol);
        }
        let transfer_instruction = system_instruction::transfer(owner.key, wallet.key, amount);
        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                owner.to_account_info(),
                wallet.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        ctx.accounts.vault.set_inner(Vault {
            owner: owner.key(),
            vault_wallet: *wallet.key,
            funded_amount: amount,
            airdropped_amount: 0,
        });
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, amount: u64) -> Result<()> {
        let to_wallet = &ctx.accounts.to_wallet;
        let vault_wallet = &ctx.accounts.vault_wallet;
        if **vault_wallet.try_borrow_lamports()? < amount {
            return err!(MyError::InsufficientSol);
        }
        let transfer_instruction =
            system_instruction::transfer(vault_wallet.key, to_wallet.key, amount);
        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                vault_wallet.to_account_info(),
                to_wallet.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[
                b"vault_wallet",
                ctx.accounts.owner.key().as_ref(),
                ctx.accounts.vault.key().as_ref(),
                &[ctx.bumps.vault_wallet],
            ]],
        )?;
        let vault = &mut ctx.accounts.vault;
        vault.airdropped_amount = vault
            .airdropped_amount
            .checked_add(amount)
            .ok_or(MyError::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer=owner, seeds=[b"vault",owner.key().as_ref()], bump, space = 8 + 32+32+ 8+8)]
    pub vault: Account<'info, Vault>,
    #[account(mut, seeds=[b"vault_wallet",owner.key().as_ref(),vault.key().as_ref()], bump)]
    pub vault_wallet: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds=[b"vault",owner.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut, seeds=[b"vault_wallet",owner.key().as_ref(),vault.key().as_ref()], bump)]
    pub vault_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub to_wallet: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub vault_wallet: Pubkey,
    pub funded_amount: u64,
    pub airdropped_amount: u64,
}

#[error_code]
pub enum MyError {
    #[msg("Insufficient sol balance")]
    InsufficientSol,
    #[msg("Integer overflow")]
    Overflow,
}
