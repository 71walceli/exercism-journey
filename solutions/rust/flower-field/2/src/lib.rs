
const EMPTY_CELL_CHR: u8 = 0x20; // ' '
const FLOWER_CELL_CHR: u8 = 0x2a; // '*'

const EMPTY_CELL_VALUE: u8 = 0;
const FLOWER_CELL_VALUE: u8 = 10;

#[derive(Debug)]
struct Garden {
    width: usize,
    height: usize,
    grid: Vec<Vec<u8>>,
}

fn check_garden(garden: &[&str]) -> bool {
    if garden.is_empty() {
        return true;
    }
    
    let width = garden[0].len();
    for row in garden.iter().skip(1) {
        if row.len() != width {
            return false;
        }
    }
    true
}

fn _build(garden: &[&str]) -> Garden {
    let height = garden.len();
    let mut width = 0;
    
    let mut grid = vec![
        vec![0, 0],
        vec![0, 0],
    ];
    if height > 0 {
        width = garden[0].len();
        grid = Vec::new();
        grid.push(vec![0_u8; width+2]);
        for row in garden.iter() {
            let mut new_row = Vec::new();
            new_row.push(0_u8);
            for chr in row.as_bytes().iter() {
                new_row.push(match *chr {
                    EMPTY_CELL_CHR => EMPTY_CELL_VALUE,
                    FLOWER_CELL_CHR => FLOWER_CELL_VALUE,
                    _ => chr - 0x30,
                });
            }
            new_row.push(0_u8);
            
            grid.push(new_row);
        }
        grid.push(vec![0; width+2]);
    }
    Garden { height, width, grid }
}

fn _count(garden: Garden) -> Garden {
    let height = garden.height;
    let width = garden.width;
    let mut grid = garden.grid;
    
    for (i, row) in grid.clone().into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell == FLOWER_CELL_VALUE {
                for _i in i-1..=i+1 {
                    for _j in j-1..=j+1 {
                        if grid[_i][_j] != FLOWER_CELL_VALUE {
                            grid[_i][_j] += 1;
                        }
                    }
                }
            }
        }
    }
    
    Garden { height, width, grid }
}

fn new_garden(garden: &[&str]) -> Garden {
    let garden = _build(garden);
    
    _count(garden)
}

fn to_string(garden: &Garden) -> Vec<String> {
    garden.grid.iter().skip(1).take(garden.height)
        .map(|row| {
            row.iter().skip(1).take(garden.width)
                .map(|cell| match *cell {
                    EMPTY_CELL_VALUE => EMPTY_CELL_CHR,
                    FLOWER_CELL_VALUE => FLOWER_CELL_CHR,
                    _ => cell + 0x30
                } as char)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if check_garden(garden) {
        let _garden = dbg!(new_garden(garden));
        let result = dbg!(to_string(&_garden));
        return result
    }
    panic!("Provided board is invalid, as it has different sized for each row.")
}
