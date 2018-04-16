use structs::*;
use cards::*;

pub fn gain_cards(state: State, cards: usize) -> State {
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

pub fn gain_card_up_to_cost(state: State, card: Card, cost: i8) -> State {
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

pub fn lose_action(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining - 1,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

pub fn gain_actions(state: State, actions: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining + actions,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining
  }
}

pub fn lose_extra_coins(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: 0,
    purchases_remaining: state.purchases_remaining
  }
}

pub fn gain_coins(state: State, coins: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins + coins,
    purchases_remaining: state.purchases_remaining
  }
}

pub fn gain_buys(state: State, buys: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining + buys
  }
}

pub fn complete_purchase(state: State) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining - 1
  }
}

pub fn add_purchases(state: State, purchases: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins,
    purchases_remaining: state.purchases_remaining + purchases
  }
}

pub fn gain_silver_onto_deck(state: State) -> State {
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

pub fn discard_top_card_with_option_to_play_if_action(state: State) -> (State, Option<Card>) {
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

pub fn play_action_from_hand_twice(state: State, card: Card) -> State {
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

pub fn trash_card_for_card_costing(state: State, trash_card: Card, new_card: Card, cost: i8) -> State {
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

pub fn discard_cards_for_empty_supply_piles(state: State, empty_supply_piles: usize) -> State {
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

pub fn gain_card_to_hand_costing(state: State, card: Card, cost: i8) -> State {
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

pub fn put_card_from_hand_onto_deck(state: State, card: Card) -> State {
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

pub fn discard_any_number_of_cards_and_then_draw_that_many(state: State, cards: usize) -> State {
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

pub fn trash_up_to_four_cards_from_hand(state: State, cards: Vec<Card>) -> State {
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

pub fn each_player_draws_card(states: Vec<State>) -> Vec<State> {
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

pub fn put_card_from_discard_onto_deck(state: State, card: Card) -> State {
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

pub fn gain_coin_if_silver_played(state: State) -> State {
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

pub fn trash_treasure_for_treasure_costing(state: State, trashed_treasure: Card,
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

pub fn trash_copper(state: State) -> State {
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

pub fn draw_to_seven_cards_discarding_drawn_actions_at_will_then_discarding_them(
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

pub fn curse_each_player(states: Vec<State>) -> Vec<State> {
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

pub fn reveal_top_two_cards_of_deck_trashing_single_non_copper_treasure_then_discarding_rest(state: State) -> State {
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

pub fn reveal_victory_card_from_hand_and_put_on_deck_if_there(state: State, card: Card) -> State {
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

pub fn discard_down_to_three_cards_if_no_moat_in_hand(state: State, cards_to_discard: Vec<Card>) -> State {
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