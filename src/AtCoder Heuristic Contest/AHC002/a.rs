use proconio::input;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}

struct Input {
    start: (usize, usize),
    tiles: Vec<Vec<i32>>,
    scores: Vec<Vec<i32>>,
    height: usize,
    width: usize,
}

impl Input
{
    fn new(start: (usize, usize), tiles: Vec<Vec<i32>>, scores: Vec<Vec<i32>>) -> Self {
        Self {
            start: start,
            tiles: tiles,
            scores: scores,
            height: 50,
            width: 50,
        }
    }

    fn tile(&self, pos: (usize, usize)) -> i32 {
        self.tiles[pos.0][pos.1]
    }

    fn score(&self, pos: (usize, usize)) -> i32 {
        self.scores[pos.0][pos.1]
    }
}

struct Output
{
    route: Vec<usize>,
    used: HashSet<i32>,
    pos: (usize, usize),
    score: i32,
    dir: Vec<(isize, isize)>,
}

impl Output {
    fn new(start: (usize, usize), input: &Input) -> Self {
        let mut used = HashSet::new();
        used.insert(input.tile(start));
        Self {
            route: Vec::new(),
            used: used,
            pos: start,
            score: input.score(start),
            dir: vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
        }
    }

    fn trans(&mut self, dir: usize, input: &Input) -> Option<i32> {
        if let Some(nxt) = self.next_pos(dir) {
            let tile = input.tile(nxt);
            if self.used.contains(&tile) {
                return None;
            }
            self.pos = nxt;
            self.route.push(dir);
            self.used.insert(tile);
            self.score += input.score(nxt);
            return Some(self.score);
        }
        None
    }

    fn back(&mut self, size: usize, input: &Input) -> Option<Vec<usize>> {
        if self.route.len() < size {
            return None;
        }

        let mut route = Vec::new();
        for _ in 0..size {
            route.push(self.route.last().unwrap().clone());
            let pos = self.pos;
            let dir = (self.route.last().unwrap().clone() + 2) % 4;
            let tile = input.tile(pos);
            let score = input.score(pos);
            self.used.remove(&tile);
            self.score -= score;
            self.route.pop();
            self.pos = self.next_pos(dir).unwrap();
        }
        route.reverse();
        Some(route)
    }

    fn next_pos(&self, dir: usize) -> Option<(usize, usize)> {
        let pos = self.pos;
        let (nh, nw) = (pos.0 as isize + self.dir[dir].0, pos.1 as isize + self.dir[dir].1);
        if nh < 0 || nw < 0 {
            return None;
        }
        let nxt = (nh as usize, nw as usize);
        if nxt.0 >= 50 || nxt.1 >= 50 {
            return None;
        }
        Some(nxt)
    }

    fn get_route(&self) -> Vec<char> {
        let dir = vec!['R', 'D', 'L', 'U'];
        let mut ret = Vec::new();
        for &idx in self.route.iter() {
            ret.push(dir[idx]);
        }
        ret
    }
}

fn main() {
    get_time();
    const T0: f64 = 2e3;
    const T1: f64 = 6e2;
    const TL: f64 = 1.92;
    let (height, width) = (50, 50);
    input! {
        pos:(usize,usize),
        tiles:[[i32;width];height],
        scores:[[i32;width];height]
    }

    let input = Input::new(pos, tiles, scores);
    let mut output = Output::new(pos, &input);

    let mut count = 0;
    let mut T = T0;

    //let dir = vec!['R', 'D', 'L', 'U'];
    let mut dt =
        vec![
            vec![0, 2, 1], vec![0, 2, 3],
            vec![2, 0, 1], vec![2, 0, 3],
            vec![1, 3, 0], vec![1, 3, 2],
            vec![3, 1, 0], vec![3, 1, 2],
            vec![0, 1, 3], vec![0, 3, 1],
            vec![2, 1, 3], vec![2, 3, 1],
            vec![1, 0, 2], vec![1, 2, 0],
            vec![3, 0, 2], vec![3, 2, 0],
            vec![0, 1, 2, 3], vec![0, 1, 3, 2], vec![0, 2, 1, 3], vec![0, 2, 3, 1], vec![0, 3, 1, 2], vec![0, 3, 2, 1],
            vec![1, 0, 2, 3], vec![1, 0, 3, 2], vec![1, 2, 0, 3], vec![1, 2, 3, 0], vec![1, 3, 0, 2], vec![1, 3, 2, 0],
            vec![2, 0, 1, 3], vec![2, 0, 3, 1], vec![2, 1, 0, 3], vec![2, 1, 3, 0], vec![2, 3, 0, 1], vec![2, 3, 1, 0],
            vec![3, 0, 1, 2], vec![3, 0, 2, 1], vec![3, 1, 0, 2], vec![3, 1, 2, 0], vec![3, 2, 0, 1], vec![3, 2, 1, 0]];

    /*
    loop {
        let old_score = output.score;
        let mut dir = 0;
        let mut max_score = 0;
        for i in 0..dt.len() {
            let mut move_count = 0;
            loop {
                let mut update = false;
                for &idx in dt[i].iter() {
                    if let Some(score) = output.trans(idx, &input) {
                        update = true;
                        break;
                    }
                }

                if !update {
                    break;
                }
                move_count += 1;
            }
            let score = output.score;
            if max_score < score {
                max_score = score;
                dir = i;
            }
            output.back(move_count, &input);
        }
        let mut move_count = 0;
        loop {
            let mut update = false;
            for &idx in dt[dir].iter() {
                if let Some(score) = output.trans(idx, &input) {
                    update = true;
                    break;
                }
            }

            if !update {
                break;
            }
            move_count += 1;
        }
        if move_count <= 10 {
            break;
        }
        output.back(min(move_count, 10), &input);
    }*/
    let mut ans_score = output.score;
    let mut ans = output.get_route();
    loop {
        count += 1;
        if count % 100 == 0 {
            let t = get_time() / TL;
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }

        // n回戻ってランダムな方向に直進
        let back_count = rand::thread_rng().gen_range(0, output.route.len() + 1) as usize;
        let old_score = output.score;
        let old_route = output.back(back_count, &input);
        if old_route == None {
            continue;
        }
        let old_route = old_route.unwrap();
        let mut dir = 0;
        let mut max_score = 0;
        for i in 0..dt.len() {
            let mut move_count = 0;
            loop {
                let mut update = false;
                for &idx in dt[i].iter() {
                    if let Some(score) = output.trans(idx, &input) {
                        update = true;
                        break;
                    }
                }

                if !update {
                    break;
                }
                move_count += 1;
            }
            let score = output.score;
            if max_score < score {
                max_score = score;
                dir = i;
            }
            output.back(move_count, &input);
        }
        let mut move_count = 0;
        loop {
            let mut update = false;
            for &idx in dt[dir].iter() {
                if let Some(score) = output.trans(idx, &input) {
                    update = true;
                    break;
                }
            }

            if !update {
                break;
            }
            move_count += 1;
        }

        let new_score = output.score;
        let t = rand::thread_rng().gen_range(0.0, 1.0);
        if old_score > new_score && f64::exp((new_score - old_score) as f64 / T) < t {
            // 昔に戻す
            output.back(move_count, &input);
            for dir in old_route {
                output.trans(dir, &input);
            }
        }
        if output.score > ans_score {
            ans_score = output.score;
            ans = output.get_route();
        }
    }

    println!("{}", ans_score);
    for c in ans {
        print!("{}", c);
    }
    println!("");
}