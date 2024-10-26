use anchor_lang::prelude::*;

declare_id!("BMx3HydRi7N7MNuXCKd1S5oC719HEJyszSJePg5MMGVC");

#[program]
pub mod idea_with_rust_issue_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
