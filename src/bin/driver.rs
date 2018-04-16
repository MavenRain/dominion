extern crate dominion;
use dominion::enums::{Action, Effect};
use dominion::structs::State;
use dominion::cards::{copper, duchy, silver, village};
use dominion::effects::gain_cards;

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