use anchor_lang::prelude::*;

declare_id!("GPDqEdD9ZMH34g3i9YRGpQaa5BoANSbbkj63TuSECWrs");

#[program]
pub mod new_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
