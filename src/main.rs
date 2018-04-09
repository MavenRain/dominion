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

enum Attack {
  Curse,
  RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest,
  RevealVictoryCardFromHandAndPutOntoDeckIfThere,
  DiscardDownToThreeCards
}

trait Card {
  fn cost(& self) -> i8;
  fn points(& self) -> i8;
  fn action(& self) -> Option<Vec<Action>>;
  fn value(& self) -> i8;
  fn attack(& self) -> Option<Attack>;
}

struct Gold;
impl Card for Gold {
  fn cost(& self) -> i8 { 6 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 3 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Estate;
impl Card for Estate {
  fn cost(& self) -> i8 { 2 }
  fn points(& self) -> i8 { 1 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Province;
impl Card for Province {
  fn cost(& self) -> i8 { 8 }
  fn points(& self) -> i8 { 6 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Gardens {
  cards: i8
}
impl Card for Gardens {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { self.cards / 10 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Duchy;
impl Card for Duchy {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 3 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Workshop;
impl Card for Workshop {
  fn cost(& self) -> i8 { 3 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCardUpToCost(4)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Witch;
impl Card for Witch {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(2)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { Some(Attack::Curse) }
}

struct Village;
impl Card for Village {
  fn cost(& self) -> i8 { 3 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(1), Action::GainActions(2)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Curse;
impl Card for Curse {
  fn cost(& self) -> i8 { 0 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { -1 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Vassal;
impl Card for Vassal {
  fn cost(& self) -> i8 { 3 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCoins(2), Action::DiscardTopCardWithOptionToPlayIfAction]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct ThroneRoom;
impl Card for ThroneRoom {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::PlayActionFromHandTwice]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Smithy;
impl Card for Smithy {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(3)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Remodel {
  card: Card
}
impl Card for Remodel {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::TrashCardForCardCosting(self.card.cost() + 2)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Poacher {
  empty_supply_piles: i8
}
impl Card for Poacher {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(1), Action::GainActions(1), Action::GainCoins(1), Action::DiscardCardsForEmptySupplyPiles(self.empty_supply_piles)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Artisan;
impl Card for Artisan {
  fn cost(& self) -> i8 { 6 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCardToHandCosting(5), Action::PutCardFromHandOntoDeck]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Bandit;
impl Card for Bandit {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCoins(1)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { Some(Attack::RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest) }
}

struct Bureaucrat;
impl Card for Bureaucrat {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::PutCardFromHandOntoDeck]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { Some(Attack::RevealVictoryCardFromHandAndPutOntoDeckIfThere) }
}

struct Cellar;
impl Card for Cellar {
  fn cost(& self) -> i8 { 2 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainActions(1), Action::DiscardAnyNumberOfCardsAndThenDrawThatMany]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Chapel;
impl Card for Chapel {
  fn cost(& self) -> i8 { 2 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::TrashUpToFourCardsFromHand]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct CouncilRoom;
impl Card for CouncilRoom {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(4), Action::GainBuys(1), Action::EachOtherPlayerDrawsCard]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Copper;
impl Card for Copper {
  fn cost(& self) -> i8 { 0 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 1 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Festival;
impl Card for Festival {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainActions(2), Action::GainBuys(1), Action::GainCoins(2)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Harbinger;
impl Card for Harbinger {
  fn cost(& self) -> i8 { 3 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::PutCardFromDiscardOntoDeck]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Laboratory;
impl Card for Laboratory {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(2), Action::GainActions(1)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Library;
impl Card for Library {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::DrawToSevenCardsDiscardingDrawnActionsAtWillAndDiscardingThemAfterward]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Market;
impl Card for Market {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(1), Action::GainActions(1), Action::GainBuys(1), Action::GainCoins(1)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Merchant;
impl Card for Merchant {
  fn cost(& self) -> i8 { 3 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCoinIfSilverPlayed]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Militia;
impl Card for Militia {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { Some(Attack::DiscardDownToThreeCards) }
}

struct Mine {
  treasure: Card
}
impl Card for Mine {
  fn cost(& self) -> i8 { 5 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::TrashTreasureForTreasureCosting(self.treasure.cost() + 3)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Moat;
impl Card for Moat {
  fn cost(& self) -> i8 { 2 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::GainCards(2)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Moneylender;
impl Card for Moneylender {
  fn cost(& self) -> i8 { 4 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { Some(vec![Action::TrashCopper, Action::GainCoins(3)]) }
  fn value(& self) -> i8 { 0 }
  fn attack(& self) -> Option<Attack> { None }
}

struct Silver;
impl Card for Silver {
  fn cost(& self) -> i8 { 3 }
  fn points(& self) -> i8 { 0 }
  fn action(& self) -> Option<Vec<Action>> { None }
  fn value(& self) -> i8 { 2 }
  fn attack(& self) -> Option<Attack> { None }
}

fn main() {
    println!("Hello, world!");
}
