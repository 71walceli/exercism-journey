"""Functions to automate Conda airlines ticketing system."""
from typing import Literal, Iterator, TypeAlias
from itertools import chain

SeatLetter: TypeAlias = Literal['A','B','C','D']

LETTERS = "ABCD"
NUM_ROWS = len(LETTERS)

def generate_seat_letters(number: int) -> Iterator[SeatLetter]:
    """Generate a series of letters for airline seats.

    :param number: int - total number of seat letters to be generated.
    :return: generator - generator that yields seat letters.

    Seat letters are generated from A to D.
    After D it should start again with A.

    Example: A, B, C, D

    """
    return (LETTERS[i % NUM_ROWS] for i in range(number))

SKIPPED_ROW = 13
def generate_seat_numbers(number: int) -> Iterator[int]:
    for i in range(number // NUM_ROWS + 1):
        for j in range(NUM_ROWS):
            if i*NUM_ROWS+j > number:
                break
            i_ = i+1
            if i_ >= SKIPPED_ROW:
                yield i_+1
                continue
            yield i_

Seat = str    # f'int{SeatLetter}'
def generate_seats(number: int) -> Iterator[str]:
    """Generate a series of identifiers for airline seats.

    :param number: int - total number of seats to be generated.
    :return: generator - generator that yields seat numbers.

    A seat number consists of the row number and the seat letter.

    There is no row 13.
    Each row has 4 seats.

    Seats should be sorted from low to high.

    Example: 3C, 3D, 4A, 4B

    """
    letter_gen = generate_seat_letters(number)
    number_gen = generate_seat_numbers(number)
    return (f"{next(number_gen)}{next(letter_gen)}" for i in range(number))

def assign_seats(passengers: list[str]) -> dict[str, Seat]:
    """Assign seats to passengers.

    :param passengers: list[str] - a list of strings containing names of passengers.
    :return: dict - with the names of the passengers as keys and seat numbers as values.

    Example output: {"Adele": "1A", "Björk": "1B"}

    """
    seat_gen = generate_seats(len(passengers))
    return {passenger: next(seat_gen) for passenger in passengers}

def generate_codes(seat_numbers: list[str], flight_id: str) -> str:
    """Generate codes for a ticket.

    :param seat_numbers: list[str] - list of seat numbers.
    :param flight_id: str - string containing the flight identifier.
    :return: generator - generator that yields 12 character long ticket codes.
    """
    seats = iter(seat_numbers)
    return (f'{f"{next(seats)}{flight_id}":<012}' for seat in seat_numbers)
