use anchor_lang::prelude::*;

declare_id!("GV1gLaMV2EvxtSj3RtjdtncLYSQwnvaLFJueZMhMQ2TF");

#[program]
pub mod myprogram {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
