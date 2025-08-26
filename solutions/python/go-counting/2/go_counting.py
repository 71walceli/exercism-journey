WHITE = "W"
BLACK = "B"
NEUTRAL = "."
NONE  = NEUTRAL # To be exported outside for the tests

from typing import TypeAlias

Point: TypeAlias = tuple[int, int]

class Board:
    """ Count territories of each player in a Go game

    Args:
        board (list[str]): A two-dimensional Go board
    """

    def __init__(self, board: list[str]):
        if len(set(len(row) for row in board)) > 1:
            raise ValueError("Invalid board shape")
        self.board = tuple(tuple(
            cell.upper()
                .replace(" ", NEUTRAL)
                .replace("B", BLACK)
                .replace("W", WHITE)
            for cell in row
        ) for row in board)
        self.max_x, self.max_y = len(self.board[0])-1, len(self.board)-1

    def _get(self, x: int, y: int) -> str | None:
        if self.max_x >= x >= 0 and self.max_y >= y >= 0:
            return self.board[y][x]

    def _dfs(self, x: int, y: int, visited: set[Point], border: set[str]) \
    -> tuple[set[Point], set[str]]:
        position = x, y
        visited.add(position)
        
        # Can it go down?
        for _x, _y in ( (x, y-1),(x+1, y),(x, y+1),(x-1, y) ):
            adjacent_value = self._get(_x,_y)
            if adjacent_value is None or (_x,_y) in visited:
                continue
            if adjacent_value != NEUTRAL:
                border.add(adjacent_value)
                continue
            self._dfs(_x, _y, visited, border)    # Can it go down? Yes
        return visited, border
    
    def find_region(self, x: int, y: int) -> tuple[set[Point], set[str]]:
        if not (-1 < x < len(self.board[0]) and -1 < y < len(self.board)):
            raise ValueError("Invalid coordinate")
        if self.board[y][x] != NEUTRAL:
            return set(), set()
        visited = set()
        border = set()
        return self._dfs(x, y, visited, border)

    def territory(self, x: int, y: int) -> tuple[str, set[Point]]:
        """ Find the owner and the territories given a coordinate on the board
        
        Args:
            x (int): Column on the board
            y (int): Row on the board
        Returns:
            (str, set(x,y)): A tuple, the first element being the owner of that area.  One of 
                `WHITE`, `BLACK`, `NEUTRAL`.  The second being a set of coordinates, representing
                the owner's territories.
        """
        region, border = self.find_region(x,y)
        owner = tuple(border)[0] if len(border) == 1 else NEUTRAL
        return owner, region

    def territories(self) -> dict[str, set[Point]]:
        """ Find the owners and the territories of the whole board.
        
        Args:
            none
        Returns:
            dict(str, set): A dictionary whose key being the owner (`WHITE`, `BLACK`, `NEUTRAL`). 
                The value being a set of coordinates owned by the owner.
        """
        all_visited = set()
        all_territories = {
            WHITE: set(),
            BLACK: set(),
            NEUTRAL: set(),
        }
        for y, _ in enumerate(self.board):
            for x, _ in enumerate(self.board[y]):
                if (x,y) in all_visited:
                    continue
                owner, region = self.territory(x, y)
                all_visited |= region
                all_territories[owner] |= region
        return all_territories
