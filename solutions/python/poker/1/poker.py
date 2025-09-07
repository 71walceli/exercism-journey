"""
Poker hand evaluation module.

This module provides functionality to evaluate and compare poker hands.
It can determine the type of poker hand (e.g., straight, flush, full house)
and find the best hands from a list of hands.
"""

from enum import Enum
from typing import Literal, Sequence, TypeVar, TypeAlias
from functools import reduce


# Type aliases for better readability and type safety
Rank: TypeAlias = int  # Card ranks: 2-14 (where 11=J, 12=Q, 13=K, 14=A)
Suit: TypeAlias = Literal['C', 'D', 'H', 'S']  # Club, Diamond, Heart, Spade

Card: TypeAlias = tuple[Rank, Suit]
Hand: TypeAlias = tuple[Card, Card, Card, Card, Card]  # Exactly 5 cards

LETTER_RANKS: dict[str, int] = {
    'J': 11,
    'Q': 12,
    'K': 13,
    'A': 14,
}

def _decode_card(card: str) -> Card:
    """
    Convert a string representation of a card to a Card tuple.
    
    Args:
        card: String like "AS", "10H", "JC", etc.
        
    Returns:
        Tuple of (rank, suit) where rank is an integer and suit is a character.
        
    Raises:
        ValueError: If the suit is invalid.
    """
    rank_str = card[:-1]
    rank = LETTER_RANKS[rank_str] if rank_str in LETTER_RANKS else int(rank_str)
    if rank < 2 or rank > 14:
        raise ValueError("Invalid rank value, must be between 2 and 14 inclusive")
    
    suit = card[-1]
    POSSIBLE_SUITS = 'CDHS'
    if suit not in POSSIBLE_SUITS:
        raise ValueError("Invalid suite type")
    return (rank, suit)  # type: ignore

def _decode_hand(hand: str) -> Hand:
    cards = hand.split(" ")
    if len(set(cards)) != 5:
        # There can't be repeated cards.
        raise ValueError("Multiple cards are the same in a single hand.")
    decoded_cards = tuple(_decode_card(card) for card in cards)
    if len(decoded_cards) != 5:
        raise ValueError("Hand must contain exactly 5 cards")
    return decoded_cards  # type: ignore


T = TypeVar('T')
def _compute_freqs(keys: Sequence[T]) -> dict[T, int]:
    return reduce(
        lambda counts, key: {**counts, key: counts.get(key, 0) + 1}, 
        keys,
        {}
    )

def _get_ranks_freq(cards: Hand) -> dict[Rank, int]:
    return _compute_freqs([card[0] for card in cards])

def _get_suits_freq(cards: Hand) -> dict[Suit, int]:
    return _compute_freqs([card[1] for card in cards])

def _sort_freqs(freqs: dict[T, int]) -> tuple[int, ...]:
    return tuple(reversed(sorted(freqs.values())))

def _get_sorted_ranks(ranks_freq: dict[Rank, int]) -> tuple[Rank, ...]: 
    return tuple(sorted(ranks_freq.keys()))


class CardTypes(Enum):
    """Enum representing different poker hand types with their priority values."""
    ROYAL_FLUSH = 10
    STRAIGHT_FLUSH = 9
    FOUR_OF_A_KIND = 8
    FULL_HOUSE = 7
    FLUSH = 6
    STRAIGHT = 5
    THREE_OF_A_KIND = 4
    TWO_PAIR = 3
    ONE_PAIR = 2
    HIGH_CARD = 1


def _get_hand_type(hand: Hand) -> CardTypes:
    ranks_freqs = _get_ranks_freq(hand)
    sorted_rank_freqs = _sort_freqs(ranks_freqs)

    match sorted_rank_freqs:
        case (4, 1):
            return CardTypes.FOUR_OF_A_KIND
        case (3, 2):
            return CardTypes.FULL_HOUSE
        case (3, 1, 1):
            return CardTypes.THREE_OF_A_KIND
        case (2, 2, 1):
            return CardTypes.TWO_PAIR
        case (2, 1, 1, 1):
            return CardTypes.ONE_PAIR

    sorted_ranks = _get_sorted_ranks(ranks_freqs)
    is_in_sequence = (sorted_ranks == (2, 3, 4, 5, LETTER_RANKS['A']) 
        or sorted_ranks == tuple(range(sorted_ranks[0], sorted_ranks[0] + len(sorted_ranks)))
    )
    
    is_all_same_suit = len(_get_suits_freq(hand)) == 1

    if is_in_sequence and is_all_same_suit:
        if LETTER_RANKS['A'] in sorted_ranks and 10 in sorted_ranks:
            return CardTypes.ROYAL_FLUSH
        return CardTypes.STRAIGHT_FLUSH
    elif is_all_same_suit:
        return CardTypes.FLUSH
    elif is_in_sequence:
        return CardTypes.STRAIGHT
    
    return CardTypes.HIGH_CARD

def _gen_sortable_hand(hand: Hand) -> tuple[int, tuple[tuple[int, Rank], ...]]:
    """Generate a sortable representation of a hand for comparison purposes."""
    hand_type = _get_hand_type(hand)
    
    # Handle low ace in straights (A-2-3-4-5)
    modified_hand = hand
    if (hand_type in [CardTypes.STRAIGHT, CardTypes.STRAIGHT_FLUSH] 
        and LETTER_RANKS['A'] in (card[0] for card in hand)
    ):
        modified_hand = tuple((1 if rank == LETTER_RANKS['A'] else rank, suit) 
            for rank, suit in hand
        )  # type: ignore

    ranks_freqs = _get_ranks_freq(modified_hand)
    # Sort by frequency (descending) then by rank (descending)
    sorted_ranks_by_freq = tuple((freq, rank) 
        for rank, freq in sorted(ranks_freqs.items(), key=lambda x: (x[1], x[0]), reverse=True)
    )
    return (hand_type.value, sorted_ranks_by_freq)

def best_hands(hands: list[str]) -> list[str]:
    """Return the list of hands that have the highest poker value."""
    if len(hands) == 0:
        return []
    if len(hands) == 1:
        return hands

    original_hands = hands
    decoded_hands = tuple(_decode_hand(hand) for hand in hands)
    greatest_hand = max(_gen_sortable_hand(hand) for hand in decoded_hands)
    
    result: list[str] = []
    for original, decoded_hand in zip(original_hands, decoded_hands):
        if _gen_sortable_hand(decoded_hand) == greatest_hand:
            result.append(original)
    return result
    
if __name__ == "__main__":
    with open("hands.txt") as f:
        for line in f.readlines():
            hand_str = line.strip()  # Remove newline character
            print(hand_str)
            hand = _decode_hand(hand_str)
            print(f"hand = {hand}")

            ranks_freqs = _get_ranks_freq(hand)
            sorted_ranks = _sort_freqs(ranks_freqs)
            print(f"ranks: {ranks_freqs}, sorted = {sorted_ranks}")
            
            suits_freqs = _get_suits_freq(hand)
            sorted_suits = _sort_freqs(suits_freqs)
            print(f"suits: {suits_freqs}, sorted = {sorted_suits}")

            hand_type = _get_hand_type(hand)
            print(f"type = {hand_type.name}, value = {hand_type.value}")

            sortable_hand = _gen_sortable_hand(hand)
            print(f"Sortable Hand: {sortable_hand}")
            print("-" * 80)
