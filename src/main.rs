mod enums;
use enums::{Action, Effect};
mod structs;
use structs::State;
mod cards;
use cards::*;
mod effects;
use effects::*;
 
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
