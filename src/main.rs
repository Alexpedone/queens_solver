use std::env;
use std::io;

#[derive(Debug)]
struct Case {
    color: usize,
    queen: bool,
}

#[derive(Debug)]
struct Grid {
    cases: Vec<Vec<Case>>,
}

impl Grid {
    fn size(&self) -> usize {
        return self.cases.len();
    }
    fn finished(&self) -> bool {
        let size = self.size();
        let mut queens_line: Vec<usize> = vec![0; size];
        let mut queens_column: Vec<usize> = vec![0; size];
        let mut queens_color: Vec<usize> = vec![0; size];
        let mut queens_neighbor: usize = 1;
        for y in 0..size {
            for x in 0..size {
                if self.cases[y][x].queen {
                    queens_line[y] += 1;
                    queens_column[x] += 1;
                    queens_color[self.cases[y][x].color] += 1;
                    if check_queen_in_neighbor(&self, &x, &y) {
                        queens_neighbor += 1
                    }
                }
            }
        }
        for i in 0..size {
            if (queens_line[i] * queens_column[i] * queens_color[i] * queens_neighbor) != 1 {

                return false;
            }
        }
        dbg!(&queens_line);
        dbg!(&queens_column);
        dbg!(&queens_color);
        dbg!(&queens_neighbor);
        return true;
    }

    fn render(self) -> () {
        let size = self.size();
        for i in 0..size {
            print!("\n");
            for j in 0..size {
                if self.cases[i][j].queen {
                    print!(" Q |")
                } else {
                    print!(" . |")
                }
            }
        }
        print!("\n");
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut grid_default: Grid = Grid {
        cases: vec![
            vec![
                Case {
                    color: 0,
                    queen: false,
                },
                Case {
                    color: 0,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
            ],
            vec![
                Case {
                    color: 0,
                    queen: false,
                },
                Case {
                    color: 0,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 2,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
            ],
            vec![
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 3,
                    queen: false,
                },
                Case {
                    color: 3,
                    queen: false,
                },
                Case {
                    color: 3,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
            ],
            vec![
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 3,
                    queen: false,
                },
                Case {
                    color: 4,
                    queen: false,
                },
                Case {
                    color: 4,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
            ],
            vec![
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 3,
                    queen: false,
                },
                Case {
                    color: 4,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
            ],
            vec![
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 5,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 6,
                    queen: false,
                },
            ],
            vec![
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 1,
                    queen: false,
                },
                Case {
                    color: 6,
                    queen: false,
                },
                Case {
                    color: 6,
                    queen: false,
                },
                Case {
                    color: 6,
                    queen: false,
                },
            ],
        ],
    };
    // let mut grid: Grid = _build_grid();
    backtracking(&mut grid_default, 0);
    grid_default.render();
}

fn _parse_line_to_grid(line_str: &String) -> Vec<Case> {
    let mut line_vec: Vec<Case> = Vec::new();
    let clean_str = line_str.trim(); // Retire les espaces de début et de fin
    let parts: Vec<&str> = clean_str.split(',').collect(); // Sépare les valeurs par la virgule
    for part in parts {
        match part.parse::<usize>() {
            Ok(color) => {
                line_vec.push(Case {
                    color,
                    queen: false,
                });
            }
            Err(_) => {
                eprintln!("Erreur lors de l'analyse du nombre: '{}'", part);
                std::process::exit(1);
            }
        }
    }
    line_vec
}

fn _build_grid() -> Grid {
    println!("Please input the first line");
    let mut base_grid: Grid = Grid { cases: Vec::new() };
    let mut line_str: String = String::new();
    io::stdin()
        .read_line(&mut line_str)
        .expect("Failed to read line");
    let mut line_parsed: Vec<Case> = _parse_line_to_grid(&line_str);
    let size: usize = line_parsed.len();
    base_grid.cases.push(line_parsed);
    println!("There should be {} lines", size);
    for i in 1..size {
        let k = i + 1;
        println!("Please input the line {}", k);
        line_str.clear();
        io::stdin()
            .read_line(&mut line_str)
            .expect("Failed to read line");
        line_parsed = _parse_line_to_grid(&line_str);
        base_grid.cases.push(line_parsed);
    }
    base_grid
}

fn check_queen_in_line(grid: &Grid, y: usize) -> bool {
    for x in 0..grid.cases.len() {
        if grid.cases[y][x].queen {
            return true;
        }
    }
    false
}

fn check_queen_in_column(grid: &Grid, x: usize) -> bool {
    for y in 0..grid.cases.len() {
        if grid.cases[y][x].queen {
            return true;
        }
    }
    false
}

fn check_queen_in_color(grid: &Grid, color: usize) -> bool {
    let size = grid.size();
    for y in 0..size {
        for x in 0..size {
            if grid.cases[y][x].color == color {
                if grid.cases[y][x].queen {
                    return true;
                }
            }
        }
    }
    false
}

fn check_queen_in_neighbor(grid: &Grid, x: &usize, y: &usize) -> bool {
    let size = grid.size();
    let y_min = if *y > 0 { y - 1 } else { *y };
    let y_max = if *y < size - 1 { y + 1 } else { *y };
    let x_min = if *x > 0 { x - 1 } else { *x };
    let x_max = if *x < size - 1 { x + 1 } else { *x };
    for i in y_min..=y_max {
        for j in x_min..=x_max {
            if (i, j) != (*y, *x) && grid.cases[i][j].queen {
                return true;
            }
        }
    }
    false
}

fn authorized_queen(grid: &Grid, x: &usize, y: &usize) -> bool {
    !check_queen_in_line(grid, *y)
        || !check_queen_in_column(grid, *x)
        || !check_queen_in_color(grid, grid.cases[*y][*x].color)
        || !check_queen_in_neighbor(grid, x, y)
}

fn backtracking(grid: &mut Grid, y: usize) -> bool {
    let size = grid.size();
    if y == size {
        return grid.finished();
    }
    for x in 0..size {
        if authorized_queen(&grid, &x, &y) {
            grid.cases[y][x].queen = true;
            if backtracking(grid, y + 1) {
                return true;
            }
            grid.cases[y][x].queen = false;
        }
    }
    false
}
