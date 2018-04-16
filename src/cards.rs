use structs::*;
use enums::*;
use effects::*;

pub fn gold() -> Card {
  Card {
    cost: 6,
    points: 0,
    effects: None,
    value: 3
  }
}  

pub fn estate() -> Card {
  Card {
    cost: 2,
    points: 1,
    effects: None,
    value: 0
  }
}

pub fn province() -> Card {
  Card {
    cost: 8,
    points: 6,
    effects: None,
    value: 0
  }
}

pub fn gardens(cards: i8) -> Card {
  Card {
    cost: 4,
    points: cards / 10,
    effects: None,
    value: 0
  }
}

pub fn duchy() -> Card {
  Card {
    cost: 5,
    points: 3,
    effects: None,
    value: 0
  }
}

pub fn workshop() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCardUpToCost(gain_card_up_to_cost, 4))]),
    value: 0
  }
}

pub fn witch() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 2)),
      Effect::Negative(Attack::Curse(curse_each_player))]),
    value: 0
  }
}

pub fn village() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 1)),
      Effect::Positive(Action::GainActions(gain_actions, 2))]),
    value: 0
  }
}

pub fn curse() -> Card {
  Card {
    cost: 0,
    points: -1,
    effects: None,
    value: 0
  }
}

pub fn vassal() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCoins(gain_coins, 2)),
      Effect::Positive(Action::DiscardTopCardWithOptionToPlayIfAction(discard_top_card_with_option_to_play_if_action))]),
    value: 0
  }
}

pub fn throne_room() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::PlayActionFromHandTwice(play_action_from_hand_twice))]),
    value: 0
  }
}

pub fn smithy() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 3))]),
    value: 0
  }
}

pub fn remodel(card: Card) -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::TrashCardForCardCosting(trash_card_for_card_costing,
      card.cost + 2))]),
    value: 0
  }
}

pub fn poacher(empty_supply_piles: usize) -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 1)),
      Effect::Positive(Action::GainActions(gain_actions, 1)),
      Effect::Positive(Action::GainCoins(gain_coins, 1)),
      Effect::Neutral(Action::DiscardCardsForEmptySupplyPiles(discard_cards_for_empty_supply_piles,
        empty_supply_piles))]),
    value: 0
  }
}

pub fn artisan() -> Card {
  Card {
    cost: 6,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCardToHandCosting(gain_card_to_hand_costing, 5)),
      Effect::Positive(Action::PutCardFromHandOntoDeck(put_card_from_hand_onto_deck))]),
    value: 0
  }
}

pub fn bandit() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCoins(gain_coins, 1)),
      Effect::Negative(Attack::RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest(reveal_top_two_cards_of_deck_trashing_single_non_copper_treasure_then_discarding_rest))]),
    value: 0
  }
}

pub fn bureaucrat() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::PutCardFromHandOntoDeck(put_card_from_hand_onto_deck)),
      Effect::Negative(Attack::RevealVictoryCardFromHandAndPutOntoDeckIfThere(reveal_victory_card_from_hand_and_put_on_deck_if_there))]),
    value: 0
  }
}

pub fn cellar() -> Card {
  Card {
    cost: 2,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainActions(gain_actions, 1)),
      Effect::Positive(Action::DiscardAnyNumberOfCardsAndThenDrawThatMany(discard_any_number_of_cards_and_then_draw_that_many))]),
    value: 0
  }
}

pub fn chapel() -> Card {
  Card {
    cost: 2,
    points: 0,
    effects: Some(vec![Effect::Neutral(Action::TrashUpToFourCardsFromHand(trash_up_to_four_cards_from_hand))]),
    value: 0
  }
}

pub fn council_room() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 4)),
      Effect::Positive(Action::GainBuys(gain_buys, 1)),
      Effect::Neutral(Action::EachOtherPlayerDrawsCard(gain_cards))]),
    value: 0
  }
}

pub fn copper() -> Card {
  Card {
    cost: 0,
    points: 0,
    effects: None,
    value: 1
  }
}

pub fn festival() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainActions(gain_actions, 2)),
      Effect::Positive(Action::GainBuys(gain_buys, 1)),
      Effect::Positive(Action::GainCoins(gain_coins, 2))]),
    value: 0
  }
}

pub fn harbinger() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::PutCardFromDiscardOntoDeck(put_card_from_discard_onto_deck))]),
    value: 0
  }
}

pub fn laboratory() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 2)),
      Effect::Positive(Action::GainActions(gain_actions, 1))]),
    value: 0
  }
}

pub fn library() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::DrawToSevenCardsDiscardingDrawnActionsAtWillAndDiscardingThemAfterward(draw_to_seven_cards_discarding_drawn_actions_at_will_then_discarding_them))]),
    value: 0
  }
}

pub fn market() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 1)),
      Effect::Positive(Action::GainActions(gain_actions, 1)),
      Effect::Positive(Action::GainBuys(gain_buys, 1)),
      Effect::Positive(Action::GainCoins(gain_coins, 1))]),
    value: 0
  }
}

pub fn merchant() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCoinIfSilverPlayed(gain_coin_if_silver_played))]),
    value: 0
  }
}

pub fn militia() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Negative(Attack::EachOtherPlayerDiscardsDownToThreeCardsIfNoMoatInHand(discard_down_to_three_cards_if_no_moat_in_hand))]),
    value: 0
  }
}

pub fn mine(treasure: Card) -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::TrashTreasureForTreasureCosting(trash_treasure_for_treasure_costing, treasure.cost + 3))]),
    value: 0
  }
}

pub fn moat() -> Card {
  Card {
    cost: 2,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 2))]),
    value: 0
  }
}

pub fn moneylender() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Neutral(Action::TrashCopper(trash_copper)),
      Effect::Positive(Action::GainCoins(gain_coins, 3))]),
    value: 0
  }
}

pub fn silver() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: None,
    value: 2
  }
}