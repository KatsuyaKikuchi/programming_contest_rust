use proconio::input;
use proconio::marker::Chars;

fn count(x: usize, y: usize, board: &Vec<Vec<char>>) -> usize {
    let mut ret = 0;
    let (h, w) = (board.len(), board[0].len());
    let dx = vec![-1, 0, 1];
    let dy = vec![-1, 0, 1];
    for &i in &dx {
        let nx = (x as isize) + i;
        if nx < 0 {
            continue;
        }
        for &j in &dy {
            let ny = (y as isize) + j;
            if ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if nx >= h || ny >= w {
                continue;
            }
            if board[nx][ny] == '#' {
                ret += 1;
            }
        }
    }
    ret
}

fn main()
{
    input! {
    (h,w):(usize,usize),
    board:[Chars;h]
    }

    for i in 0..h {
        for j in 0..w {
            if board[i][j] == '#' {
                print!("#");
            } else {
                print!("{}", count(i, j, &board));
            }
        }
        println!("");
    }
}