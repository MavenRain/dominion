#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Attack {
  Curse,
  RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest,
  RevealVictoryCardFromHandAndPutOntoDeckIfThere,
  DiscardDownToThreeCards
}

#[derive(Clone, Debug)]
struct Card {
  cost: i8,
  points: i8,
  actions: Option<Vec<Action>>,
  value: i8,
  attack: Option<Attack>
}

fn action_vec_compare(va: &[Action], vb: &[Action]) -> bool {
    (va.len() == vb.len()) &&
    va.iter().zip(vb).all(|(a,b)| * a == * b)
}

impl PartialEq for Card {
  fn eq(& self, other: & Card) -> bool {
    self.cost == other.cost &&
    self.points == other.points &&
    match (self.actions.clone(), other.actions.clone()) {
      (None, None) => true,
      (Some(selfactions), Some(otheractions)) => action_vec_compare(& selfactions, & otheractions),
      _ => false
    } &&
    self.value == other.value &&
    match (self.attack.clone(), other.attack.clone()) {
      (None, None) => true,
      (Some(selfattack), Some(otherattack)) => selfattack == otherattack,
      _ => false
    }
  }
}

impl Eq for Card {}

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
    actions: Some(vec![Action::GainCards(1), Action::GainActions(1), Action::GainCoins(1),
      Action::DiscardCardsForEmptySupplyPiles(empty_supply_piles)]),
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
  hand: Vec<Card>,
  deck: Vec<Card>,
  discard: Vec<Card>,
  actions_remaining: i8,
  extra_coins: i8,
  purchases_remaining: i8
}

impl State {
  fn summarize(& self) {
    println!("hand: {:?}", self.hand);
    println!("deck: {:?}", self.deck);
    println!("discard: {:?}", self.discard);
    println!("actions remaining: {}", self.actions_remaining);
    println!("extra coins: {}", self.extra_coins);
    println!("purchases remaining: {}", self.purchases_remaining);
  }
}

enum ActionNext {
  GainCards(fn(State, usize) -> State),
  GainCardsUpToCost(fn(State, Card, i8) -> State),
  LoseAction(fn(State) -> State),
  GainActions(fn(State, i8) -> State),
  LoseExtraCoins(fn(State) -> State),
  GainExtraCoins(fn(State, i8) -> State),
  CompletePurchase(fn(State) -> State),
  AddPurchases(fn(State, i8) -> State),
  GainSilverOntoDeck(fn(State) -> State),
  DiscardTopCardWithOptionToPlayIfAction(fn(State) -> (State, Option<Card>)),
  PlayActionFromHandTwice(fn(State, Card) -> State),
  TrashCardForCardCosting(fn(State, Card, Card, i8) -> State),
  DiscardCardsForEmptySupplyPiles(fn(State, usize) -> State),
  GainCardToHandCosting(fn(State, Card, i8) -> State),
  PutCardFromHandOntoDeck(fn(State, Card) -> State),
  DiscardAnyNumberOfCardsAndThenDrawThatMany(fn(State, usize) -> State),
  TrashUpToFourCardsFromHand(fn(State, Vec<Card>) -> State)
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

fn gain_extra_coins(state: State, coins: i8) -> State {
  State {
    hand: state.hand,
    deck: state.deck,
    discard: state.discard,
    actions_remaining: state.actions_remaining,
    extra_coins: state.extra_coins + coins,
    purchases_remaining: state.purchases_remaining
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
  match state.hand.clone().into_iter().find(|x| x.to_owned() == card) {
    None => state,
    _ => {
      let new_card = Card {
        cost: card.cost,
        points: card.points,
        actions: {
          if let Some(mut new_actions) = card.actions.clone() {
            let new_actions_clone = new_actions.clone();
            new_actions.extend(new_actions_clone);
            Some(new_actions)
          }
          else { card.actions.clone() }
        },
        value: card.value,
        attack: card.attack.clone()
      };
      State {
        hand: {
              let mut new_hand = state.hand.clone().into_iter().filter(|x| x.to_owned() !=
                card).collect::<Vec<Card>>();
              new_hand.extend(vec![new_card]);
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
        let mut new_hand = state.hand.into_iter().filter(|x| x.to_owned() != trash_card).collect::<Vec<Card>>();
        new_hand.extend(vec![new_card]);
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
      new_hand.extend(vec![card]);
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
      hand: state.hand.into_iter().filter(|x| x.to_owned() != card).collect::<Vec<Card>>(),
      deck: {
        let mut new_deck = state.deck;
        new_deck.extend(vec![card]);
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
    discard: state.discard.into_iter().filter(|x| x.to_owned() != card).collect::<Vec<Card>>(),
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
      let remaining_coppers = state.hand.clone().into_iter().filter(|x| x.to_owned() == copper())
        .skip(1).collect::<Vec<Card>>();
      let mut new_hand = state.hand.clone().into_iter().filter(|x| x.to_owned() != copper())
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
          z.actions.is_some()).collect::<Vec<Card>>().contains(y)));
      ! new_hand.into_iter().take(7).filter(|y| ! state.hand.clone().contains(& y)).collect::<Vec<Card>>().contains(& x)
    }).collect::<Vec<Card>>(),
    discard: {
      let mut new_discard = state.discard.clone();
      new_discard.extend({
        let mut new_hand = state.hand.clone();
        new_hand.extend(state.deck.clone().into_iter().filter(|y|
          ! action_cards_to_discard.clone().into_iter().filter(|z|
            z.actions.is_some()).collect::<Vec<Card>>().contains(y)));
        new_hand.into_iter().take(7).filter(|y| ! state.hand.clone().contains(& y))
      });
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
  let actions = vec![ActionNext::GainCards(gain_cards)];
}
