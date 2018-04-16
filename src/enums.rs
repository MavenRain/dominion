use structs::{Card, State};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
  GainCards(fn(State, usize) -> State, usize),
  GainCardUpToCost(fn(State, Card, i8) -> State, i8),
  GainActions(fn(State, i8) -> State, i8),
  GainBuys(fn(State, i8) -> State, i8),
  GainCoins(fn(State, i8) -> State, i8),
  GainSilverOntoDeck(fn(State) -> State),
  DiscardTopCardWithOptionToPlayIfAction(fn(State) -> (State, Option<Card>)),
  PlayActionFromHandTwice(fn(State, Card) -> State),
  TrashCardForCardCosting(fn(State, Card, Card, i8) -> State, i8),
  DiscardCardsForEmptySupplyPiles(fn(State, usize) -> State, usize),
  GainCardToHandCosting(fn(State, Card, i8) -> State, i8),
  PutCardFromHandOntoDeck(fn(State, Card) -> State),
  DiscardAnyNumberOfCardsAndThenDrawThatMany(fn(State, usize) -> State),
  TrashUpToFourCardsFromHand(fn(State, Vec<Card>) -> State),
  EachOtherPlayerDrawsCard(fn(State, usize) -> State),
  PutCardFromDiscardOntoDeck(fn(State, Card) -> State),
  DrawToSevenCardsDiscardingDrawnActionsAtWillAndDiscardingThemAfterward(fn(State, Vec<Card>) -> State),
  GainCoinIfSilverPlayed(fn(State) -> State),
  TrashTreasureForTreasureCosting(fn(State, Card, Card, i8) -> State, i8),
  TrashCopper(fn(State) -> State),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attack {
  Curse(fn(Vec<State>) -> Vec<State>),
  RevealTopTwoOfDeckAndTrashRevealedNonCopperTreasureThenDiscardRest(fn(State) -> State),
  RevealVictoryCardFromHandAndPutOntoDeckIfThere(fn(State, Card) -> State),
  EachOtherPlayerDiscardsDownToThreeCardsIfNoMoatInHand(fn(State, Vec<Card>) -> State)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
  Positive(Action),
  Neutral(Action),
  Negative(Attack)
}