use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};

pub fn spl_transfer<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    signer_seeds: Option<&[&[&[u8]]]>, // Use Option to explicitly handle the presence or absence of seeds
) -> Result<()> {
    let transfer_accounts = Transfer {
        from,
        to,
        authority,
    };

    let transfer_ctx = match signer_seeds {
        Some(seeds) => CpiContext::new_with_signer(
            token_program, 
            transfer_accounts, 
            seeds
        ),
        None => CpiContext::new(
            token_program, 
            transfer_accounts
        ),
    };

    transfer(transfer_ctx, amount)
}
