use anchor_lang::prelude::*;

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub id: u8,
    pub card_number: u8
}

impl Default for Card {
    fn default() -> Self {
        Card {id: 0, card_number: 0}
    }
}