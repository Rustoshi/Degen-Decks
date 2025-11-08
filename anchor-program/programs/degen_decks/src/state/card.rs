use anchor_lang::prelude::*;

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Card {
    id: u8,
    card_number: u8,
    sub_number: u8
}