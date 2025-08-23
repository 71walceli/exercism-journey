from typing import TypeAlias

Board: TypeAlias = list[str]

MINE = "*"
SAFE = " "

def _get(board: Board, x: int, y: int):
    try:
        if x < 0 or y < 0:
            raise IndexError
        return board[y][x]
    except IndexError:
        return None

def _indices(x: int):
    return range(-1+x, x+1+1)

def _count(board, x, y):
    return sum(1 if _get(board, _x, _y) == MINE else 0
        for _y in _indices(y)
        for _x in _indices(x)
    )

def annotate(board):
    if (
        len(set(len(row) for row in board)) > 1 
        or any(row.replace(SAFE, "").replace(MINE, "") for row in board)
    ):
        raise ValueError("The board is invalid with current input.")
    return [
        "".join(
            str(_count(board, _x, _y)) if cell == SAFE and _count(board, _x, _y) > 0
                else cell
            for _x, cell in enumerate(row)
        )
        for _y, row in enumerate(board)
    ]
