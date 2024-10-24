use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct WithdrawLamports<'info> {
    #[account(mut,seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,

    /// CHECK: Safe
    #[account(mut)]
    pub pda: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(address = admin_info.receiver)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_lamports(ctx: Context<WithdrawLamports>) -> Result<()> {

    let rent = &Rent::get()?;
    let rent_balance = rent.minimum_balance(ctx.accounts.pda.to_account_info().data_len());
    let bal = ctx.accounts.pda.get_lamports();

    require!(bal > rent_balance, ErrorCode::BalNeedGTRentBalance);
    let withdraw_amount = bal - rent_balance;

    **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;
    **ctx.accounts.receiver.try_borrow_mut_lamports()? += withdraw_amount;

    msg!("withdraw_amount is {:?}", withdraw_amount);

    Ok(())
}