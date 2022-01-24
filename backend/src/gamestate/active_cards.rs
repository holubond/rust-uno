use crate::cards::card::{Card, CardSymbol};

static ALLOWED_ACTIVE_CARDS: [CardSymbol; 3] =
    [CardSymbol::Skip, CardSymbol::Draw2, CardSymbol::Draw4];

#[derive(Clone)]
pub(super) struct ActiveCards {
    active_cards: Vec<Card>,
}

impl ActiveCards {
    pub(super) fn new() -> ActiveCards {
        ActiveCards {
            active_cards: vec![],
        }
    }

    pub(super) fn are_cards_active(&self) -> bool {
        !self.active_cards.is_empty()
    }

    pub(super) fn sum_active_draw_cards(&self) -> Option<usize> {
        if self.are_cards_active() {
            match self.active_symbol_unchecked() {
                CardSymbol::Draw2 => Some(2 * self.active_cards.len()),
                CardSymbol::Draw4 => Some(4 * self.active_cards.len()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub(super) fn active_symbol(&self) -> Option<CardSymbol> {
        if self.are_cards_active() {
            Some(self.active_symbol_unchecked())
        } else {
            None
        }
    }

    fn active_symbol_unchecked(&self) -> CardSymbol {
        self.active_cards.get(0).unwrap().symbol.clone()
    }

    /// Ensures that only active cards can be of the same symbol by returning Err otherwise.
    pub(super) fn push(&mut self, card: Card) -> anyhow::Result<()> {
        if self.are_cards_active() && self.active_cards.iter().any(|ac| ac.symbol != card.symbol) {
            anyhow::bail!("Cannot stack active cards of different symbols!")
        }
        if !ALLOWED_ACTIVE_CARDS.contains(&card.symbol) {
            anyhow::bail!("Active card cannot have symbol {}!", &card.symbol)
        }
        // after here, all active cards are expected to have equal symbols

        self.active_cards.push(card);
        Ok(())
    }

    pub(super) fn clear(&mut self) {
        self.active_cards.clear();
    }
}
