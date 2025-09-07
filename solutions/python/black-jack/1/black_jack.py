"""Functions to help play and score a game of blackjack.

How to play blackjack:    https://bicyclecards.com/how-to-play/blackjack/
"Standard" playing cards: https://en.wikipedia.org/wiki/Standard_52-card_deck

Card values:
1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
2.  'A' (ace card) = 1 or 11
3.  '2' - '10' = numerical value.
"""


def value_of_card(card):
    """Determine the scoring value of a card.
    
    :param card: str - given card.
    :return: int - value of a given card.  See below for values.
    """
    match card:
        case 'A':
            return 1
        case 'J' | 'Q' | 'K':
            return 10
        case _:
            return int(card)

def _get_pair_values(card1: str, card2: str) -> (int, int):
    return (
        value_of_card(card1),
        value_of_card(card2),
    )

def higher_card(card1, card2):
    """Determine which card has a higher value in the hand.

    :param card1, card2: str - cards dealt in hand.  See below for values.
    :return: str or tuple - resulting Tuple contains both cards if they are of equal value.
    """
    card1_value, card2_value = _get_pair_values(card1, card2)
    if card1_value == card2_value:
        return (card1, card2)
    return card1 if card1_value > card2_value else card2

#def get_score(card1: str, card2: str, a_)

def value_of_ace(card1, card2):
    """Calculate the most advantageous value for the ace card. If an ace is already present, it's assumed to be worth 11, any other after will be worth 1.

    :param card1, card2: str - card dealt. See below for values.
    :return: int - 11 as long as hand score stays below 21, 1 otherwise.
    """
    score = sum(_get_pair_values(card1, card2))
    if 'A' in (card1, card2):
        score +=10
    return 1 if score + 11 > 21 else 11

def is_blackjack(card1, card2):
    """Determine if the hand is a 'natural' or 'blackjack'.

    :param card1, card2: str - card dealt. See below for values.
    :return: bool - is the hand a blackjack (two cards worth 21).
    """
    return _get_pair_values(card1, card2) in ( (1,10), (10,1) )    # 'A' (value == 1) = 11 (if already in hand)

def can_split_pairs(card1, card2):
    """Determine if a player can split their hand into two hands.

    :param card_one, card_two: str - cards dealt.
    :return: bool - can the hand be split into two pairs? (i.e. cards are of the same value).
    """
    card1_value, card2_value = _get_pair_values(card1, card2)
    return card1_value == card2_value

def can_double_down(card1, card2):
    """Determine if a blackjack player can place a double down bet.

    :param card1, card2: str - first and second cards in hand.
    :return: bool - can the hand be doubled down? (i.e. totals 9, 10 or 11 points).
    """
    return 9 <= sum(_get_pair_values(card1, card2)) <= 11
