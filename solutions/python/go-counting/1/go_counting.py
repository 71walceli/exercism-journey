WHITE = "W"
BLACK = "B"
NONE  = "."
EMPTY = NONE

from typing import TypeAlias, Literal

Point: TypeAlias = tuple[int, int]

class Board:
    """Count territories of each player in a Go game

    Args:
        board (list[str]): A two-dimensional Go board
    """

    def __init__(self, board: list[str]):
        if len(set(len(row) for row in board)) > 1:
            raise ValueError("Invalid board shape")
        self.board = tuple(tuple(
            cell.upper()
                .replace(" ", EMPTY)
                .replace("B", BLACK)
                .replace("W", WHITE)
            for cell in row
        ) for row in board)
        """
        with open("grids.txt", "a") as f:
            for row in self.board:
                print(" ".join(row), file=f)
            print("", file=f)
        """

    def _get(self, x: int, y: int) -> str | None:
        try:
            if x >= 0 and y >= 0:
                return self.board[y][x]
            raise IndexError
        except IndexError:
            return None

    def _dfs(self, x: int, y: int, visited: set[Point]):
        position = x, y
        visited.add(position)

        # Can it process this node? Yes
        value = self._get(*position)
        if value != EMPTY:
            return visited  # Can it process this node? Yes
        
        # Can it go down?
        for x,y in ( (x,   y-1),(x+1, y  ),(x,   y+1),(x-1, y  ),):
            adjacent_value = self._get(x,y)
            if adjacent_value is None or (x,y) in visited:
                continue
            self._dfs(x, y, visited)    # Can it go down? Yes
        return visited
    
    def find_region(self, x: int, y: int) -> set[Point]:
        if not (-1 < x < len(self.board[0]) and -1 < y < len(self.board)):
            raise ValueError("Invalid coordinate")
        if self.board[y][x] != EMPTY:
            return set()
        visited = set()
        return self._dfs(x, y, visited)

    def territory(self, x: int, y: int) -> tuple[str, set[Point]]:
        """Find the owner and the territories given a coordinate on
           the board

        Args:
            x (int): Column on the board
            y (int): Row on the board

        Returns:
            (str, set): A tuple, the first element being the owner
                        of that area.  One of "W", "B", "".  The
                        second being a set of coordinates, representing
                        the owner's territories.
        """
        region = self.find_region(x,y)
        values = tuple(self.board[y][x] for x,y in region)
        empty_region = set((x,y) for x,y in region if self.board[y][x] == EMPTY)
        edge_types = set(value for value in values if value != EMPTY)
        match len(edge_types):
            case 1:
                owner = tuple(edge_types)[0] if tuple(edge_types)[0] in (WHITE, BLACK) else EMPTY
            case _:
                owner = EMPTY
        return (owner, empty_region)

    def territories(self) -> dict[str, set[Point]]:
        """Find the owners and the territories of the whole board

        Args:
            none

        Returns:
            dict(str, set): A dictionary whose key being the owner
                        , i.e. "W", "B", "".  The value being a set
                        of coordinates owned by the owner.
        """
        all_vislted = set()
        all_territories = {
            WHITE: set(),
            BLACK: set(),
            EMPTY: set(),
        }
        for y, row in enumerate(self.board):
            for x, cell in enumerate(self.board[y]):
                if (x,y) in all_vislted:
                    continue
                owner, region = self.territory(x, y)
                all_vislted |= region
                all_territories[owner] |= region
        return all_territories


if __name__ == "__main__":
    boards = (
        (
            [
                " ww ",
                "w  w",
                "w w ",
                " w  ",
            ],
            (
                (0,0),
                (1,0),
                (1,1),
                (0,3),
                (3,0),
                (3,3),
                (2,2),
            ),
        ),
        (
            [
                "..B..",
                ".B.B.",
                "B.W.B",
                ".W.W.",
                "..W..",
            ],
            (
                (0,0),
                (1,0),
                (1,1),
                (0,3),
                (3,0),
                (3,3),
                (2,2),
                (2,1),
                (4,1),
                (2,3),
                (2,2),
            ),
        ),
        (
            [
                ".",
            ],
            (
                (0,0),
                (1,0),
                (0,-1),
            ),
        ),
        (
            [
                ".B."
            ],
            (
                (0,0),
                (1,0),
            ),
        ),
        (
            [
                ".BW.",
                ".BW."
            ],
            (
                (3,0),
                (2,0),
                (3,1),
            ),
        ),
    )
    for _board, points in boards:
        board = Board(_board)
        print(*(" ".join(row) for row in board.board), sep="\n")
        for point in points:
            try:
                region = sorted(board.find_region(*point))
            except ValueError as e:
                print(point, "ERROR", str(e))
                continue
            values = tuple(board.board[y][x] for x,y in region)
            edge_types = set(value for value in values if value != EMPTY)
            total_size = sum(1 for value in values)
            empty_size = sum(1 for value in values if value == EMPTY)
            
            print(point, "=", {
                "total_size": total_size, 
                "empty_size": empty_size, 
                "edge_types": [*edge_types], 
            },
                "=", region
            )
        print()
