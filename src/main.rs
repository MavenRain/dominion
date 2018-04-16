mod enums;
use enums::{Action, Attack, Effect};
mod structs;
use structs::{Card, State};

fn gold() -> Card {
  Card {
    cost: 6,
    points: 0,
    effects: None,
    value: 3
  }
}  

fn estate() -> Card {
  Card {
    cost: 2,
    points: 1,
    effects: None,
    value: 0
  }
}

fn province() -> Card {
  Card {
    cost: 8,
    points: 6,
    effects: None,
    value: 0
  }
}

fn gardens(cards: i8) -> Card {
  Card {
    cost: 4,
    points: cards / 10,
    effects: None,
    value: 0
  }
}

fn duchy() -> Card {
  Card {
    cost: 5,
    points: 3,
    effects: None,
    value: 0
  }
}

fn workshop() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCardUpToCost(gain_card_up_to_cost, 4))]),
    value: 0
  }
}

fn witch() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 2)),
      Effect::Negative(Attack::Curse(curse_each_player))]),
    value: 0
  }
}

fn village() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 1)),
      Effect::Positive(Action::GainActions(gain_actions, 2))]),
    value: 0
  }
}

fn curse() -> Card {
  Card {
    cost: 0,
    points: -1,
    effects: None,
    value: 0
  }
}

fn vassal() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCoins(gain_coins, 2)),
      Effect::Positive(Action::DiscardTopCardWithOptionToPlayIfAction(discard_top_card_with_option_to_play_if_action))]),
    value: 0
  }
}

fn throne_room() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::PlayActionFromHandTwice(play_action_from_hand_twice))]),
    value: 0
  }
}

fn smithy() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 3))]),
    value: 0
  }
}

fn remodel(card: Card) -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::TrashCardForCardCosting(trash_card_for_card_costing,
      card.cost + 2))]),
    value: 0
  }
}

fn poacher(empty_supply_piles: usize) -> Card {
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

fn artisan() -> Card {
  Card {
    cost: 6,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCardToHandCosting(gain_card_to_hand_costing, 5)),
      Effect::Positive(Action::PutCardFromHandOntoDeck(put_card_from_hand_onto_deck))]),
    value: 0
  }
}

fn bandit() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCoins(gain_coins, 1)),
      Effect::Negative(Attack::RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest(reveal_top_two_cards_of_deck_trashing_single_non_copper_treasure_then_discarding_rest))]),
    value: 0
  }
}

fn bureaucrat() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::PutCardFromHandOntoDeck(put_card_from_hand_onto_deck)),
      Effect::Negative(Attack::RevealVictoryCardFromHandAndPutOntoDeckIfThere(reveal_victory_card_from_hand_and_put_on_deck_if_there))]),
    value: 0
  }
}

fn cellar() -> Card {
  Card {
    cost: 2,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainActions(gain_actions, 1)),
      Effect::Positive(Action::DiscardAnyNumberOfCardsAndThenDrawThatMany(discard_any_number_of_cards_and_then_draw_that_many))]),
    value: 0
  }
}

fn chapel() -> Card {
  Card {
    cost: 2,
    points: 0,
    effects: Some(vec![Effect::Neutral(Action::TrashUpToFourCardsFromHand(trash_up_to_four_cards_from_hand))]),
    value: 0
  }
}

fn council_room() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 4)),
      Effect::Positive(Action::GainBuys(gain_buys, 1)),
      Effect::Neutral(Action::EachOtherPlayerDrawsCard(gain_cards))]),
    value: 0
  }
}

fn copper() -> Card {
  Card {
    cost: 0,
    points: 0,
    effects: None,
    value: 1
  }
}

fn festival() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainActions(gain_actions, 2)),
      Effect::Positive(Action::GainBuys(gain_buys, 1)),
      Effect::Positive(Action::GainCoins(gain_coins, 2))]),
    value: 0
  }
}

fn harbinger() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::PutCardFromDiscardOntoDeck(put_card_from_discard_onto_deck))]),
    value: 0
  }
}

fn laboratory() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 2)),
      Effect::Positive(Action::GainActions(gain_actions, 1))]),
    value: 0
  }
}

fn library() -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::DrawToSevenCardsDiscardingDrawnActionsAtWillAndDiscardingThemAfterward(draw_to_seven_cards_discarding_drawn_actions_at_will_then_discarding_them))]),
    value: 0
  }
}

fn market() -> Card {
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

fn merchant() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCoinIfSilverPlayed(gain_coin_if_silver_played))]),
    value: 0
  }
}

fn militia() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Negative(Attack::EachOtherPlayerDiscardsDownToThreeCardsIfNoMoatInHand(discard_down_to_three_cards_if_no_moat_in_hand))]),
    value: 0
  }
}

fn mine(treasure: Card) -> Card {
  Card {
    cost: 5,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::TrashTreasureForTreasureCosting(trash_treasure_for_treasure_costing, treasure.cost + 3))]),
    value: 0
  }
}

fn moat() -> Card {
  Card {
    cost: 2,
    points: 0,
    effects: Some(vec![Effect::Positive(Action::GainCards(gain_cards, 2))]),
    value: 0
  }
}

fn moneylender() -> Card {
  Card {
    cost: 4,
    points: 0,
    effects: Some(vec![Effect::Neutral(Action::TrashCopper(trash_copper)),
      Effect::Positive(Action::GainCoins(gain_coins, 3))]),
    value: 0
  }
}

fn silver() -> Card {
  Card {
    cost: 3,
    points: 0,
    effects: None,
    value: 2
  }
}

fn gain_cards(state: State, cards: usize) -> State {
  State {
    hand: {
      let mut original_hand = state.hand;
      original_hand.extend(state.deck.clone().into_iter().take(cards));
      original_hand
    },
    deck: state.deck.into_iter().skip(cards).collect(),
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn gain_card_up_to_cost(state: State, card: Card, cost: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: if card.cost > cost { state.discard } else {
      let mut new_discard = state.discard;
      new_discard.push(card);
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn lose_action(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining - 1,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn gain_actions(state: State, actions: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining + actions,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn lose_extra_coins(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: 0,
    purchases_remaining: state.purchases_remaining
  }
}

fn gain_coins(state: State, coins: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins + coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn gain_buys(state: State, buys: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining + buys
  }
}

fn complete_purchase(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining - 1
  }
}

fn add_purchases(state: State, purchases: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining + purchases
  }
}

fn gain_silver_onto_deck(state: State) -> State {
  State {
    hand: state.hand,
    deck: {
      let mut new_deck = state.deck;
      new_deck.extend(vec![silver()]);
      new_deck
    },
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn discard_top_card_with_option_to_play_if_action(state: State) -> (State, Option<Card>) {
  (State {
    hand: state.hand,
    deck: state.deck.clone().into_iter().skip(1).collect::<Vec<Card>>(),
    discard: {
      let mut new_discard = state.discard;
      new_discard.extend(state.deck.clone().into_iter().take(1));
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  },
    state.deck.first().map(|x| x.to_owned())
  )
}

fn play_action_from_hand_twice(state: State, card: Card) -> State {
  match state.hand.clone().into_iter().find(|x| * x == card) {
    None => state,
    _ => {
      let new_card = Card {
        cost: card.cost,
        points: card.points,
        effects: {
          if let Some(mut new_effects) = card.effects.clone() {
            let new_effects_clone = new_effects.clone();
            new_effects.extend(new_effects_clone);
            Some(new_effects)
          }
          else { card.effects.clone() }
        },
        value: card.value
      };
      State {
        hand: {
              let mut new_hand = state.hand.clone().into_iter().filter(|x| * x !=
                card).collect::<Vec<Card>>();
              new_hand.push(new_card);
              new_hand
        },
        deck: state.deck,
        discard: state.discard,
        actions_remaining: state.actions_remaining,
        extra_coins: state.extra_coins,
        purchases_remaining: state.purchases_remaining  
      }
    }
  }
}

fn trash_card_for_card_costing(state: State, trash_card: Card, new_card: Card, cost: i8) -> State {
  if new_card.cost > cost { state }
  else {
    State {
      hand: {
        let mut new_hand = state.hand.into_iter().filter(|x| * x != trash_card).collect::<Vec<Card>>();
        new_hand.push(new_card);
        new_hand
      },
      deck: state.deck,
      discard: state.discard,
      actions_remaining: state.actions_remaining,
      extra_coins: state.extra_coins,
      purchases_remaining: state.purchases_remaining
    }
  }
}

fn discard_cards_for_empty_supply_piles(state: State, empty_supply_piles: usize) -> State {
  State {
    hand: state.hand.clone().into_iter().skip(empty_supply_piles).collect::<Vec<Card>>(),
    deck: state.deck,
    discard: {
      let mut new_discard = state.discard;
      new_discard.extend(state.hand.into_iter().take(empty_supply_piles));
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn gain_card_to_hand_costing(state: State, card: Card, cost: i8) -> State {
  State {
    hand: if card.cost > cost { state.hand } else {
      let mut new_hand = state.hand;
      new_hand.push(card);
      new_hand
    },
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn put_card_from_hand_onto_deck(state: State, card: Card) -> State {
  if ! state.hand.contains(& card) { state } else {
    State {
      hand: state.hand.into_iter().filter(|x| * x != card).collect::<Vec<Card>>(),
      deck: {
        let mut new_deck = state.deck;
        new_deck.push(card);
        new_deck
      },
      discard: state.discard,
      actions_remaining: state.actions_remaining,
      extra_coins: state.extra_coins,
      purchases_remaining: state.purchases_remaining
    }
  }
}

fn discard_any_number_of_cards_and_then_draw_that_many(state: State, cards: usize) -> State {
  State {
    hand: {
      let mut new_hand = state.hand.clone();
      new_hand.extend(state.deck.clone().into_iter().take(cards));
      new_hand
    },
    deck: state.deck.into_iter().skip(cards).collect::<Vec<Card>>(),
    discard: {
      let mut new_discard = state.discard;
      new_discard.extend(state.hand.into_iter().take(cards));
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn trash_up_to_four_cards_from_hand(state: State, cards: Vec<Card>) -> State {
  State {
    hand: if cards.len() > 4 { state.hand.clone() } else {
      state.hand.into_iter().filter(|x| ! cards.contains(& x)).collect::<Vec<Card>>()
    },
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn each_player_draws_card(states: Vec<State>) -> Vec<State> {
  states.into_iter().map(|x| State {
    hand: {
      let mut new_hand = x.hand;
      new_hand.extend(x.deck.clone().into_iter().take(1));
      new_hand
    },
    deck: x.deck.into_iter().skip(1).collect::<Vec<Card>>(),
    discard: x.discard,
    actions_remaining: x.actions_remaining,
    extra_coins: x.extra_coins,
    purchases_remaining: x.purchases_remaining
  }).collect::<Vec<State>>()
}

fn put_card_from_discard_onto_deck(state: State, card: Card) -> State {
  State {
    hand: state.hand,
    deck: {
      let mut new_deck = state.deck;
      new_deck.extend(vec![card.clone()]);
      new_deck
    },
    discard: state.discard.into_iter().filter(|x| * x != card).collect::<Vec<Card>>(),
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn gain_coin_if_silver_played(state: State) -> State {
  State {
    hand: state.hand.clone(),
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: if ! state.hand.contains(& silver()) { state.extra_coins + 1} else {
      state.extra_coins
    },
    purchases_remaining: state.purchases_remaining
  }
}

fn trash_treasure_for_treasure_costing(state: State, trashed_treasure: Card,
  new_treasure: Card, max_cost: i8) -> State {
  State {
    hand: if max_cost < new_treasure.cost || ! vec![copper(), silver(), gold()].contains(& trashed_treasure) ||
      ! vec![copper(), silver(), gold()].contains(& new_treasure) { state.hand.clone() } else {
      let mut new_hand = state.hand;
      new_hand.push(new_treasure);
      new_hand
    },
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn trash_copper(state: State) -> State {
  State {
    hand: {
      let remaining_coppers = state.hand.clone().into_iter().filter(|x| * x == copper())
        .skip(1).collect::<Vec<Card>>();
      let mut new_hand = state.hand.clone().into_iter().filter(|x| * x != copper())
        .collect::<Vec<Card>>();
      new_hand.extend(remaining_coppers);
      new_hand
    },
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn draw_to_seven_cards_discarding_drawn_actions_at_will_then_discarding_them(
  state: State, action_cards_to_discard: Vec<Card>) -> State {
  State {
    hand: state.hand.clone(),
    deck: state.deck.clone().into_iter().filter(|x| {
      let mut new_hand = state.hand.clone();
      new_hand.extend(state.deck.clone().into_iter().filter(|y|
        ! action_cards_to_discard.clone().into_iter().filter(|z|
          z.effects.is_some()).collect::<Vec<Card>>().contains(y)));
      ! new_hand.into_iter().take(7).filter(|y| ! state.hand.clone().contains(& y)).collect::<Vec<Card>>().contains(& x)
    }).collect::<Vec<Card>>(),
    discard: {
      let mut new_discard = state.discard.clone();
      new_discard.extend({
        let mut new_hand = state.hand.clone();
        new_hand.extend(state.deck.clone().into_iter().filter(|y|
          ! action_cards_to_discard.clone().into_iter().filter(|z|
            z.effects.is_some()).collect::<Vec<Card>>().contains(y)));
        new_hand.into_iter().take(7).filter(|y| ! state.hand.clone().contains(& y))
      });
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn curse_each_player(states: Vec<State>) -> Vec<State> {
  states.into_iter().map(|x| State {
    hand: {
      let mut new_hand = x.hand;
      new_hand.push(curse());
      new_hand
    },
    deck: x.deck,
    discard: x.discard,
    actions_remaining: x.actions_remaining,
    extra_coins: x.extra_coins,
    purchases_remaining: x.purchases_remaining
  }).collect::<Vec<State>>()
}

fn reveal_top_two_cards_of_deck_trashing_single_non_copper_treasure_then_discarding_rest(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck.clone().into_iter().skip(2).collect::<Vec<Card>>(),
    discard: {
      let mut new_discard = state.discard;
      let first_draw = state.deck.clone().into_iter().nth(0);
      let second_draw = state.deck.clone().into_iter().nth(1);
      match (first_draw, second_draw) {
        (Some(ref first), Some(ref second)) if (* first == silver() || * first == gold()) &&
          * second != silver() && * second != gold() => new_discard.push(second.to_owned()),
        (Some(ref first), Some(ref second)) if * first != silver() && * first != gold() &&
          * second != silver() && * second != gold() => new_discard.extend(vec![first.to_owned(), second.to_owned()]),
        (Some(ref first), Some(ref second)) if * first != silver() && * first != gold() &&
          (* second == silver() || * second == gold()) => new_discard.push(first.to_owned()),
        (Some(ref first), _) if * first != silver() && * first != gold() => new_discard.push(first.to_owned()),
        _ => () 
      }
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn reveal_victory_card_from_hand_and_put_on_deck_if_there(state: State, card: Card) -> State {
  State {
    hand: match & card.clone() {
      ref victory_card if (* victory_card).points > 0 => state.hand.clone().into_iter()
        .filter(|x| x != * victory_card).collect::<Vec<Card>>(),
      _ => state.hand
    },
    deck: match & card.clone() {
      ref victory_card if (* victory_card).points > 0 => {
        let mut new_deck = state.deck.clone();
        new_deck.push((* victory_card).to_owned());
        new_deck
      },
      _ => state.deck
    },
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

fn discard_down_to_three_cards_if_no_moat_in_hand(state: State, cards_to_discard: Vec<Card>) -> State {
  State {
    hand: if state.hand.clone().contains(& moat()) { state.hand.clone() } else {
      let proposed_hand = state.hand.clone().into_iter().filter(|x|
        ! cards_to_discard.clone().contains(& x)).collect::<Vec<Card>>();
      let number_of_cards_to_replenish = 3 - proposed_hand.clone().iter().len();
      let mut new_hand = proposed_hand;
      new_hand.extend(cards_to_discard.clone().into_iter().take(number_of_cards_to_replenish));
      new_hand
    },
    deck: state.deck,
    discard: {
      let mut new_discard = state.discard;
      let proposed_hand = state.hand.into_iter().filter(|x|
        ! cards_to_discard.clone().contains(& x)).collect::<Vec<Card>>();
      let number_of_cards_to_replenish = 3 - proposed_hand.clone().iter().len();
      new_discard.extend(cards_to_discard.into_iter().skip(number_of_cards_to_replenish));
      new_discard
    },
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}
 
fn main() {
  let state = State {
    hand: vec![copper(), duchy(), silver(), village()],
    deck: vec![],
    discard: vec![],
    actions_remaining: 0,
    extra_coins: 0,
    purchases_remaining: 0
  };
  state.summarize();
  let _effects = vec![Effect::Positive(Action::GainCards(gain_cards, 1))];
}
