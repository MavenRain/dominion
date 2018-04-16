use enums::*;

#[derive(Clone, Debug)]
pub struct Card {
  pub cost: i8,
  pub points: i8,
  pub effects: Option<Vec<Effect>>,
  pub value: i8,
}

pub struct State {
  pub hand: Vec<Card>,
  pub deck: Vec<Card>,
  pub discard: Vec<Card>,
  pub actions_remaining: i8,
  pub extra_coins: i8,
  pub purchases_remaining: i8
}

impl State {
  pub fn summarize(& self) {
    println!("hand: {:?}", self.hand);
    println!("deck: {:?}", self.deck);
    println!("discard: {:?}", self.discard);
    println!("actions remaining: {}", self.actions_remaining);
    println!("extra coins: {}", self.extra_coins);
    println!("purchases remaining: {}", self.purchases_remaining);
  }
}

fn effect_vec_compare(va: &[Effect], vb: &[Effect]) -> bool {
    (va.len() == vb.len()) &&
    va.iter().zip(vb).all(|(a,b)| * a == * b)
}

impl PartialEq for Card {
  fn eq(& self, other: & Card) -> bool {
    self.cost == other.cost &&
    self.points == other.points &&
    match (self.effects.clone(), other.effects.clone()) {
      (None, None) => true,
      (Some(selfeffects), Some(othereffects)) => effect_vec_compare(& selfeffects, & othereffects),
      _ => false
    } &&
    self.value == other.value
  }
}

impl Eq for Card {}