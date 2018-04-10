#[derive(Clone)]
enum Action {
  GainCards(i8),
  GainCardUpToCost(i8),
  GainActions(i8),
  GainCoins(i8),
  GainBuys(i8),
  GainSilverOntoDeck,
  DiscardTopCardWithOptionToPlayIfAction,
  PlayActionFromHandTwice,
  TrashCardForCardCosting(i8),
  DiscardCardsForEmptySupplyPiles(i8),
  GainCardToHandCosting(i8),
  PutCardFromHandOntoDeck,
  DiscardAnyNumberOfCardsAndThenDrawThatMany,
  TrashUpToFourCardsFromHand,
  EachOtherPlayerDrawsCard,
  PutCardFromDiscardOntoDeck,
  DrawToSevenCardsDiscardingDrawnActionsAtWillAndDiscardingThemAfterward,
  GainCoinIfSilverPlayed,
  TrashTreasureForTreasureCosting(i8),
  TrashCopper
}

#[derive(Clone)]
enum Attack {
  Curse,
  RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest,
  RevealVictoryCardFromHandAndPutOntoDeckIfThere,
  DiscardDownToThreeCards
}

#[derive(Clone)]
struct Card {
  cost: i8,
  points: i8,
  actions: Option<Vec<Action>>,
  value: i8,
  attack: Option<Attack>
}

fn gold() -> Card {
  Card {
    cost: 6,
    points: 0,
    actions: None,
    value: 3,
    attack: None
  }
}  

fn estate() -> Card {
  Card {
    cost: 2,
    points: 1,
    actions: None,
    value: 0,
    attack: None
  }
}

fn province() -> Card {
  Card {
    cost: 8,
    points: 6,
    actions: None,
    value: 0,
    attack: None
  }
}

fn gardens(cards: i8) -> Card {
  Card {
    cost: 4,
    points: cards / 10,
    actions: None,
    value: 0,
    attack: None
  }
}

fn duchy() -> Card {
  Card {
    cost: 5,
    points: 3,
    actions: None,
    value: 0,
    attack: None
  }
}

fn workshop() -> Card {
  Card {
    cost: 3,
    points: 0,
    actions: Some(vec![Action::GainCardUpToCost(4)]),
    value: 0,
    attack: None
  }
}

fn witch() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::GainCards(2)]),
    value: 0,
    attack: Some(Attack::Curse)
  }
}

fn village() -> Card {
  Card {
    cost: 3,
    points: 0,
    actions: Some(vec![Action::GainCards(1), Action::GainActions(2)]),
    value: 0,
    attack: None
  }
}

fn curse() -> Card {
  Card {
    cost: 0,
    points: -1,
    actions: None,
    value: 0,
    attack: None
  }
}

fn vassal() -> Card {
  Card {
    cost: 3,
    points: 0,
    actions: Some(vec![Action::GainCoins(2), Action::DiscardTopCardWithOptionToPlayIfAction]),
    value: 0,
    attack: None
  }
}

fn throne_room() -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: Some(vec![Action::PlayActionFromHandTwice]),
    value: 0,
    attack: None
  }
}

fn smithy() -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: Some(vec![Action::GainCards(3)]),
    value: 0,
    attack: None
  }
}

fn remodel(card: Card) -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: Some(vec![Action::TrashCardForCardCosting(card.cost + 2)]),
    value: 0,
    attack: None
  }
}

fn poacher(empty_supply_piles: i8) -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: Some(vec![Action::GainCards(1), Action::GainActions(1), Action::GainCoins(1), Action::DiscardCardsForEmptySupplyPiles(empty_supply_piles)]),
    value: 0,
    attack: None
  }
}

fn artisan() -> Card {
  Card {
    cost: 6,
    points: 0,
    actions: Some(vec![Action::GainCardToHandCosting(5), Action::PutCardFromHandOntoDeck]),
    value: 0,
    attack: None
  }
}

fn bandit() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::GainCoins(1)]),
    value: 0,
    attack: Some(Attack::RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest)
  }
}

fn bureaucrat() -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: Some(vec![Action::PutCardFromHandOntoDeck]),
    value: 0,
    attack: Some(Attack::RevealVictoryCardFromHandAndPutOntoDeckIfThere)
  }
}

fn cellar() -> Card {
  Card {
    cost: 2,
    points: 0,
    actions: Some(vec![Action::GainActions(1), Action::DiscardAnyNumberOfCardsAndThenDrawThatMany]),
    value: 0,
    attack: None
  }
}

fn chapel() -> Card {
  Card {
    cost: 2,
    points: 0,
    actions: Some(vec![Action::TrashUpToFourCardsFromHand]),
    value: 0,
    attack: None
  }
}

fn council_room() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::GainCards(4), Action::GainBuys(1), Action::EachOtherPlayerDrawsCard]),
    value: 0,
    attack: None
  }
}

fn copper() -> Card {
  Card {
    cost: 0,
    points: 0,
    actions: None,
    value: 1,
    attack: None
  }
}

fn festival() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::GainActions(2), Action::GainBuys(1), Action::GainCoins(2)]),
    value: 0,
    attack: None
  }
}

fn harbinger() -> Card {
  Card {
    cost: 3,
    points: 0,
    actions: Some(vec![Action::PutCardFromDiscardOntoDeck]),
    value: 0,
    attack: None
  }
}

fn laboratory() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::GainCards(2), Action::GainActions(1)]),
    value: 0,
    attack: None
  }
}

fn library() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::DrawToSevenCardsDiscardingDrawnActionsAtWillAndDiscardingThemAfterward]),
    value: 0,
    attack: None
  }
}

fn market() -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::GainCards(1), Action::GainActions(1), Action::GainBuys(1), Action::GainCoins(1)]),
    value: 0, 
    attack: None
  }
}

fn merchant() -> Card {
  Card {
    cost: 3,
    points: 0,
    actions: Some(vec![Action::GainCoinIfSilverPlayed]),
    value: 0,
    attack: None
  }
}

fn militia() -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: None,
    value: 0,
    attack: Some(Attack::DiscardDownToThreeCards)
  }
}

fn mine(treasure: Card) -> Card {
  Card {
    cost: 5,
    points: 0,
    actions: Some(vec![Action::TrashTreasureForTreasureCosting(treasure.cost + 3)]),
    value: 0,
    attack: None
  }
}

fn moat() -> Card {
  Card {
    cost: 2,
    points: 0,
    actions: Some(vec![Action::GainCards(2)]),
    value: 0,
    attack: None
  }
}

fn moneylender() -> Card {
  Card {
    cost: 4,
    points: 0,
    actions: Some(vec![Action::TrashCopper, Action::GainCoins(3)]),
    value: 0,
    attack: None
  }
}

fn silver() -> Card {
  Card {
    cost: 3,
    points: 0,
    actions: None,
    value: 2,
    attack: None
  }
}

struct State {
  points: i8,
  hand: Vec<Card>,
  deck: Vec<Card>,
  discard: Vec<Card>
}

fn gain_cards(state: State, cards: usize) -> State {
  State {
    points: state.points,
    hand: {
      let mut originalHand = state.hand;
      originalHand.extend(state.deck[..].to_vec().into_iter().take(cards).collect::<Vec<Card>>());
      originalHand
    },
    deck: state.deck.into_iter().skip(cards).collect(),
    discard: state.discard
  }
}

fn gain_card_up_to_cost(state: State, card: Card, cost: i8) -> State {
  State {
    points: state.points,
    hand: state.hand,
    deck: state.deck,
    discard: if card.cost > cost { state.discard } else {
      let mut new_discard = state.discard;
      new_discard.extend(vec![card]);
      new_discard
    }
  }
}

fn main() {
  let cards = vec![copper(), silver(), village()];  
  println!("Hello, world!");
}
