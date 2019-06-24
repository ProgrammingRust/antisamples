// error-pattern: non-exhaustive patterns

enum Suit {
    Books, Bugs, Fromps, Zurfs
}

enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace
}

struct Card {
    rank: Rank,
    suit: Suit
}

fn main() {
    use self::Suit::*;
    use self::Rank::*;
    let card = Card { rank: Three, suit: Fromps };

    let score = match card.rank {
        Jack => 10,
        Queen => 10,
        Ace => 11
    };  // error: nonexhaustive patterns
    println!("{}", score);
}

