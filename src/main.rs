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
            for i in 0..size {
                if queens_line[i] * queens_column[i] * queens_column[i] * queens_neighbor != 1 {
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut grid: Grid = build_grid();
    if backtracking(&mut grid, 0) {
        println!("voici la grille solutionnée : \n {:?}", grid)
    }
}

fn parse_line_to_grid(line_str: &String) -> Vec<Case> {
    let mut line_vec: Vec<Case> = Vec::new();
    let bytes = line_str.as_bytes();
    let mut commas_index: Vec<usize> = Vec::new();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            commas_index.push(i)
        }
    }
    line_vec.push(Case {
        color: line_str[0..commas_index[0]].parse().unwrap(),
        queen: false,
    });

    for i in 0..commas_index.len() - 1 {
        line_vec.push(Case {
            color: line_str[commas_index[i] + 1..commas_index[i + 1]]
                .parse()
                .unwrap(),
            queen: false,
        });
    }
    line_vec.push(Case {
        color: line_str[commas_index[commas_index.len() - 1] + 1..]
            .parse()
            .unwrap(),
        queen: false,
    });
    return line_vec;
}

fn build_grid() -> Grid {
    println!("Please input the first line");
    let mut base_grid: Grid = Grid { cases: Vec::new() };
    let mut line_str: String = String::new();
    io::stdin()
        .read_line(&mut line_str)
        .expect("Failed to read line");
    let mut line_parsed: Vec<Case> = parse_line_to_grid(&line_str);
    let size: usize = line_parsed.len();
    base_grid.cases.push(line_parsed);
    println!("There should be {} lines", size);
    for i in 1..size {
        let k = i + 1;
        println!("Please input the {} line", k);
        line_str.clear();
        io::stdin()
            .read_line(&mut line_str)
            .expect("Failed to read line");
        line_parsed = parse_line_to_grid(&line_str);
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
    for y in 0..grid.cases.len() {
        for x in 0..grid.cases[y].len() {
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
    for i in y - 1..y + 2 {
        for j in x - 1..x + 2 {
            if (i, j) != (*y, *x) && grid.cases[i][j].queen {
                return true;
            }
        }
    }
    false
}

fn authorized_queen(grid: &Grid, x: &usize, y: &usize) -> bool {
    for i in 0..grid.cases.len() {
        if !check_queen_in_line(grid, i)
            || !check_queen_in_column(grid, i)
            || !check_queen_in_color(grid, i)
            || !check_queen_in_neighbor(grid, x, y)
        {
            return false;
        }
    }
    true
}

fn backtracking(grid: &mut Grid, y: usize) -> bool {
    let size = grid.size();
    for x in 0..size {
        if grid.finished() {
            return true;
        } else {
            if authorized_queen(&grid, &x, &y) {
                grid.cases[y][x].queen = true;
                if y == size {
                    return grid.finished();
                } else {
                    let k = y + 1;
                    if backtracking(grid, k) {
                        return true;
                    } else {
                        grid.cases[y][x].queen = false;
                        return false;
                    }
                }
            }
        }
    }
    return false;
}
