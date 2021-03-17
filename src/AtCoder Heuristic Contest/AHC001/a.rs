use proconio::input;
use std::cmp::{max, min};
use rand::Rng;

/*
 memo
 コードのリファクタリングする
 拡張はすぐに呼び出せるよう関数化しておく
 x座標を縮めてy座標方向に引き伸ばすなどは試してみたい
 */

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
    x: i64,
    y: i64,
    area: i64,
}

#[derive(Clone)]
struct Aabb {
    min_pos: (i64, i64),
    max_pos: (i64, i64),
    contact_bit: i32,
}

impl Aabb {
    fn new(param: (i64, i64, i64, i64)) -> Self {
        assert!(param.0 <= param.2);
        assert!(param.1 <= param.3);
        Aabb {
            min_pos: (param.0, param.1),
            max_pos: (param.2, param.3),
            contact_bit: 0,
        }
    }

    fn contain(&self, pos: (i64, i64)) -> bool {
        // x+0.5, y+0.5 で判定
        let (x, y) = (pos.0, pos.1);
        if x < self.min_pos.0 || x >= self.max_pos.0 {
            return false;
        }
        if y < self.min_pos.1 || y >= self.max_pos.1 {
            return false;
        }
        true
    }

    fn contact(&self, aabb: &Aabb) -> bool {
        self.max_pos.0 > aabb.min_pos.0 &&
            self.min_pos.0 < aabb.max_pos.0 &&
            self.max_pos.1 > aabb.min_pos.1 &&
            self.min_pos.1 < aabb.max_pos.1
    }

    fn calc_area(&self) -> i64 {
        let (x, y) = (self.max_pos.0 - self.min_pos.0, self.max_pos.1 - self.min_pos.1);
        x * y
    }

    fn get_dt(&self, prev_min: (i64, i64), prev_max: (i64, i64)) -> (i64, i64) {
        let dx = if prev_min.0 == self.min_pos.0 {
            self.max_pos.0 - prev_max.0
        } else {
            self.min_pos.0 - prev_min.0
        };
        let dy = if prev_min.1 == self.min_pos.1 {
            self.max_pos.1 - prev_max.1
        } else {
            self.min_pos.1 - prev_min.1
        };
        (dx, dy)
    }

    fn inflate(&mut self, dir: (i64, i64)) -> Option<(i64, i64)> {
        let prev_min = self.min_pos;
        let prev_max = self.max_pos;
        self.max_pos.0 = max(self.max_pos.0, self.max_pos.0 + dir.0);
        self.max_pos.1 = max(self.max_pos.1, self.max_pos.1 + dir.1);
        self.min_pos.0 = min(self.min_pos.0, self.min_pos.0 + dir.0);
        self.min_pos.1 = min(self.min_pos.1, self.min_pos.1 + dir.1);

        self.min_pos.0 = max(self.min_pos.0, 0);
        self.min_pos.1 = max(self.min_pos.1, 0);
        self.max_pos.0 = min(self.max_pos.0, 10000);
        self.max_pos.1 = min(self.max_pos.1, 10000);

        let dt = self.get_dt(prev_min, prev_max);
        if dt == (0, 0) {
            None
        } else {
            Some(dt)
        }
    }

    fn deflate(&mut self, dir: (i64, i64)) -> Option<(i64, i64)> {
        let prev_min = self.min_pos;
        let prev_max = self.max_pos;
        self.max_pos.0 = min(self.max_pos.0, self.max_pos.0 + dir.0);
        self.max_pos.1 = min(self.max_pos.1, self.max_pos.1 + dir.1);
        self.min_pos.0 = max(self.min_pos.0, self.min_pos.0 + dir.0);
        self.min_pos.1 = max(self.min_pos.1, self.min_pos.1 + dir.1);

        self.min_pos.0 = min(self.min_pos.0, self.max_pos.0);
        self.min_pos.1 = min(self.min_pos.1, self.max_pos.1);
        self.max_pos.0 = max(self.max_pos.0, self.min_pos.0);
        self.max_pos.1 = max(self.max_pos.1, self.min_pos.1);

        let dt = self.get_dt(prev_min, prev_max);
        if dt == (0, 0) {
            None
        } else {
            Some(dt)
        }
    }

    fn translation(&mut self, dir: (i64, i64)) -> Option<(i64, i64)> {
        if let Some(dt) = self.inflate(dir) {
            self.deflate(dt)
        } else {
            None
        }
    }

    fn get_pos(&self) -> (i64, i64, i64, i64) {
        (self.min_pos.0, self.min_pos.1, self.max_pos.0, self.max_pos.1)
    }

    // input条件を保ったままdeflate
    fn area_deflate(&mut self, input: &Input, dt: (i64, i64)) {
        let (dx, dy) = dt;
        self.area_deflate_x(input, dx);
        self.area_deflate_y(input, dy);
    }

    fn area_deflate_x(&mut self, input: &Input, dx: i64) {
        if dx == 0 {
            return;
        }
        let dx = if dx < 0 {
            -min(dx.abs(), self.max_pos.0 - input.x - 1)
        } else {
            min(dx, input.x - self.min_pos.0)
        };
        let len_y = self.max_pos.1 - self.min_pos.1;
        let limit = max(0, (self.calc_area() - input.area + len_y - 1) / len_y);
        let dx = if dx.abs() > limit {
            if dx > 0 {
                limit
            } else {
                -limit
            }
        } else {
            dx
        };
        self.deflate((dx, 0));
    }
    fn area_deflate_y(&mut self, input: &Input, dy: i64) {
        if dy == 0 {
            return;
        }
        let dy = if dy < 0 {
            -min(dy.abs(), self.max_pos.1 - input.y - 1)
        } else {
            min(dy, input.y - self.min_pos.1)
        };
        let len_x = self.max_pos.0 - self.min_pos.0;
        let limit = max(0, (self.calc_area() - input.area + len_x - 1) / len_x);
        let dy = if dy.abs() > limit {
            if dy > 0 {
                limit
            } else {
                -limit
            }
        } else {
            dy
        };
        self.deflate((0, dy));
    }
}

fn score(area: (i64, i64, i64, i64), input: &Input) -> f32 {
    let aabb = Aabb::new(area);
    let pos = (input.x, input.y);
    if !aabb.contain(pos) {
        return 0.0;
    }

    let r = input.area as f32;
    let s = aabb.calc_area() as f32;
    1.0 - (1.0 - (r.min(s) / r.max(s))).powf(2.0)
}

fn calc_score(aabbs: &[Aabb], inputs: &[Input]) -> i64 {
    let n = aabbs.len();
    let mut sum = 0.0;
    for i in 0..n {
        sum += score(aabbs[i].get_pos(), &inputs[i]) as f64;
    }

    sum *= 1_000_000_000.0;
    sum as i64
}

// AABBのxが大きい順index
fn max_x(aabbs: &[Aabb]) -> Vec<usize> {
    let mut indices = aabbs.iter()
        .enumerate()
        .map(|(i, aabb)| (i, aabb.max_pos.0))
        .collect::<Vec<_>>();
    indices.sort_by(|(_, a), (_, b)| b.cmp(a));
    indices.into_iter().map(|(i, _)| i).collect::<Vec<_>>()
}

fn min_x(aabbs: &[Aabb]) -> Vec<usize> {
    let mut indices = aabbs.iter()
        .enumerate()
        .map(|(i, aabb)| (i, aabb.min_pos.0))
        .collect::<Vec<_>>();
    indices.sort_by(|(_, a), (_, b)| a.cmp(b));
    indices.into_iter().map(|(i, _)| i).collect::<Vec<_>>()
}

// AABBのyが大きい順index
fn max_y(aabbs: &[Aabb]) -> Vec<usize> {
    let mut indices = aabbs.iter()
        .enumerate()
        .map(|(i, aabb)| (i, aabb.max_pos.1))
        .collect::<Vec<_>>();
    indices.sort_by(|(_, a), (_, b)| b.cmp(a));
    indices.into_iter().map(|(i, _)| i).collect::<Vec<_>>()
}

fn min_y(aabbs: &[Aabb]) -> Vec<usize> {
    let mut indices = aabbs.iter()
        .enumerate()
        .map(|(i, aabb)| (i, aabb.min_pos.1))
        .collect::<Vec<_>>();
    indices.sort_by(|(_, a), (_, b)| a.cmp(b));
    indices.into_iter().map(|(i, _)| i).collect::<Vec<_>>()
}

fn expand(aabbs: &mut Vec<Aabb>, index: usize, input: &[Input], dt: (i64, i64), check: bool) -> bool {
    let n = aabbs.len();
    let mut update = false;
    if check {
        if input[index].area <= aabbs[index].calc_area() {
            return false;
        }
    }
    let mut contact = false;
    let prev_score = score(aabbs[index].get_pos(), &input[index]);
    let dt = aabbs[index].inflate(dt);
    if dt == None {
        return false;
    }
    let (x, y) = dt.unwrap();
    for j in 0..n {
        if index == j {
            continue;
        }
        if aabbs[index].contact(&aabbs[j]) {
            contact = true;
            break;
        }
    }
    let new_score = score(aabbs[index].get_pos(), &input[index]);
    if check {
        if new_score < prev_score {
            contact = true;
        }
    }
    if contact {
        aabbs[index].deflate((-x, -y));
    } else {
        update = true;
    }
    update
}
/*
fn move_aabb(aabbs: &mut Vec<Aabb>, index: usize, input: &[Input], dt: (i64, i64)) -> bool {
    let n = aabbs.len();
    let dt = aabbs[index].translation(dt);
    if dt == None {
        return false;
    }
    let (x, y) = dt.unwrap();
    let mut contact = false;
    for j in 0..n {
        if index == j {
            continue;
        }
        if aabbs[index].contact(&aabbs[j]) {
            contact = true;
            break;
        }
    }

    if !aabbs[index].contain((input[index].x, input[index].y)) {
        contact = true;
    }

    if contact {
        aabbs[index].translation((-x, -y));
    }
    !contact
}
 */

fn expand2(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], dt: (i64, i64)) {
    let (dx, dy) = dt;
    expand_x(aabbs, index, indices, dx);
    expand_y(aabbs, index, indices, dy);
}

// x方向に拡張できるだけ拡張する、他のAABBにぶつかったら押し戻す
fn expand_x(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], dx: i64) {
    let dx = contact_dx(aabbs, index, indices, dx);
    aabbs[index].inflate((dx, 0));
}

// y方向に拡張できるだけ拡張する、他のAABBにぶつかったら押し戻す
fn expand_y(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], dy: i64) {
    let dy = contact_dy(aabbs, index, indices, dy);
    aabbs[index].inflate((0, dy));
}

fn move_aabb(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], input: &Input, dt: (i64, i64)) {
    let (dx, dy) = dt;
    move_x(aabbs, index, indices, input, dx);
    move_y(aabbs, index, indices, input, dy);
}

// x方向にdxだけ動かす、他のAABBにぶつかったら押し戻す
fn move_x(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], input: &Input, dx: i64) {
    if dx == 0 {
        return;
    }
    let mut dx = if dx > 0 {
        min(dx, input.x - aabbs[index].min_pos.0)
    } else {
        max(dx, input.x + 1 - aabbs[index].max_pos.0)
    };

    let dx = contact_dx(aabbs, index, indices, dx);
    aabbs[index].translation((dx, 0));
}

// 他のAABBにぶつからない最大のdx
fn contact_dx(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], dx: i64) -> i64 {
    let mut dx = dx;
    let mut aabb = aabbs[index].clone();
    let base_x = if dx > 0 {
        aabb.max_pos.0
    } else {
        aabb.min_pos.0
    };
    aabb.inflate((dx, 0));
    for &i in indices.into_iter() {
        if i == index {
            continue;
        }
        if !aabb.contact(&aabbs[i]) {
            continue;
        }
        if dx > 0 {
            let diff = aabbs[i].min_pos.0 - base_x;
            dx = min(dx, diff);
        } else if dx < 0 {
            let diff = base_x - aabbs[i].max_pos.0;
            dx = max(dx, -diff);
        }
    }
    dx
}

// y方向にdyだけ動かす、他のAABBにぶつかったら押し戻す
fn move_y(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], input: &Input, dy: i64) {
    if dy == 0 {
        return;
    }
    let mut dy = if dy > 0 {
        min(dy, input.y - aabbs[index].min_pos.1)
    } else {
        max(dy, input.y + 1 - aabbs[index].max_pos.1)
    };

    let dy = contact_dy(aabbs, index, indices, dy);

    aabbs[index].translation((0, dy));
}

// 他のAABBにぶつからない最大のdy
fn contact_dy(aabbs: &mut Vec<Aabb>, index: usize, indices: &[usize], dy: i64) -> i64 {
    let mut dy = dy;
    let mut aabb = aabbs[index].clone();
    let base_y = if dy > 0 {
        aabb.max_pos.1
    } else {
        aabb.min_pos.1
    };
    aabb.inflate((0, dy));
    for &i in indices.into_iter() {
        if i == index {
            continue;
        }
        if !aabb.contact(&aabbs[i]) {
            continue;
        }
        if dy > 0 {
            let diff = aabbs[i].min_pos.1 - base_y;
            dy = min(dy, diff);
        } else if dy < 0 {
            let diff = base_y - aabbs[i].max_pos.1;
            dy = max(dy, -diff);
        }
    }
    dy
}

fn sorted_indices(aabbs: &[Aabb], dir: usize) -> Vec<usize> {
    match dir {
        0 => max_y(aabbs),
        1 => max_x(aabbs),
        2 => min_y(aabbs),
        3 => min_x(aabbs),
        _ => (0..aabbs.len()).collect::<Vec<usize>>()
    }
}

// 各AABBの近辺にあるAABBのインデックスリストを取得する
fn calc_neighborhood(aabbs: &[Aabb], range: i64) -> Vec<Vec<usize>> {
    let n = aabbs.len();
    let mut neighbor = vec![vec![]; n];
    for i in 0..n {
        let mut aabb = aabbs[i].clone();
        // 適当に拡張
        aabb.inflate((range, range));
        aabb.inflate((-range, -range));
        for j in (i + 1)..n {
            if aabb.contact(&aabbs[j]) {
                neighbor[i].push(j);
                neighbor[j].push(i);
            }
        }
    }
    neighbor
}

fn clone_aabbs(aabbs: &[Aabb]) -> Vec<Aabb> {
    aabbs.iter()
        .map(|aabb| aabb.clone())
        .collect::<Vec<_>>()
}

fn main()
{
    get_time();
    input! {
    n:usize,
    input:[(i64,i64,i64);n],
    }

    let input = input.into_iter()
        .map(|(x, y, r)| Input { x: x, y: y, area: r })
        .collect::<Vec<_>>();

    let mut aabbs = vec![];
    for i in 0..n {
        let (x, y) = (input[i].x, input[i].y);
        let mut aabb = Aabb::new((x, y, x + 1, y + 1));
        aabbs.push(aabb);
    }
    const TL: f64 = 4.6;

    let range = 40;// 一回の拡張量
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let base_indices = (0..n).collect::<Vec<_>>();

    let mut counts = vec![0; n];
    for i in 0..n {
        let r = (input[i].area as f64 / 3.0).sqrt() as i64;
        for j in 0..n {
            let (x, y) = (input[i].x - input[j].x, input[i].y - input[j].y);
            if x * x + y * y <= r * r {
                counts[i] += 1;
            }
        }
    }

    let mut indices = counts
        .iter()
        .enumerate()
        .map(|(i, v)| (v.clone(), i))
        .collect::<Vec<_>>();
    indices.sort_by(|(a, _), (b, _)| b.cmp(a));
    for _ in 0..10 {
        for &(_, i) in indices.iter() {
            for _ in 0..range {
                let mut update = false;
                for j in 0..4 {
                    let dt = (dx[j], dy[j]);
                    let inner_update = expand(&mut aabbs, i, &input, dt, true);
                    update = update || inner_update;
                }
                if !update {
                    break;
                }
            }
        }
    }
    // ビームサーチ用バッファ
    let mut buffer = Vec::new();
    buffer.push(aabbs);
    let length = 33; // ビーム幅
    // while get_time() < TL {
    for cnt in 0..30 {
        println!("loop : {}", cnt);
        // このループの結果一時保管場所
        let mut tmp_buffer = Vec::new();
        // 前回一番大きかったのはとりあえず取っておく
        let prev_max = clone_aabbs(&buffer[0]);
        tmp_buffer.push((calc_score(&prev_max, &input), prev_max));

        for base_aabbs in buffer { // 10
            for dir in 0..4 { // 10*4
                let mut aabbs = clone_aabbs(&base_aabbs);
                let indices = sorted_indices(&aabbs, dir);
                for i in indices { // 10*4*200
                    // dir方向に動かせるだけ動かす
                    // 10*4*200*200
                    //move_aabb(&mut aabbs, i, &neighbor[i], &input[i], (dx * 10000, dy * 10000));
                    move_aabb(&mut aabbs, i, &base_indices, &input[i], (dx[dir] * 10000, dy[dir] * 10000));
                }
                let neighbor = calc_neighborhood(&base_aabbs, range * 1000);

                let (front, right, left) = ((dir + 2) % 4, (dir + 1) % 4, (dir + 3) % 4);
                // dirと逆方向に拡張
                let indices = sorted_indices(&aabbs, front);
                for i in indices {
                    // 適当に拡張
                    for _ in 0..range {
                        let mut update = false;
                        for j in 0..4 {
                            let dt = (dx[j], dy[j]);
                            update = expand(&mut aabbs, i, &input, dt, true) || update;
                        }
                        if !update {
                            break;
                        }
                    }
                    // 3方向に拡張
                    expand2(&mut aabbs, i, &neighbor[i], (dx[front] * range, dy[front] * range));
                    expand2(&mut aabbs, i, &neighbor[i], (dx[right] * range, dy[right] * range));
                    expand2(&mut aabbs, i, &neighbor[i], (dx[left] * range, dy[left] * range));

                    //　面積が目標より小さくなるか、指定点を含まなくなるまでdir方向に縮小
                    aabbs[i].area_deflate(&input[i], (dx[front] * 10000, dy[front] * 10000));
                    while (aabbs[i].calc_area() > input[i].area) {
                        aabbs[i].area_deflate(&input[i], (dx[dir], dy[dir]));
                        aabbs[i].area_deflate(&input[i], (dx[left], dy[left]));
                        aabbs[i].area_deflate(&input[i], (dx[right], dy[right]));
                    }
                }

                tmp_buffer.push((calc_score(&aabbs, &input), aabbs));
            }
        }

        tmp_buffer.sort_by(|(a, _), (b, _)| b.cmp(a));
        buffer = tmp_buffer.into_iter()
            .take(length)
            .map(|(_, v)| v)
            .collect::<Vec<_>>();
    }

    loop {
        // for _ in 0..250 {
        let mut update = false;
        for i in 0..n {
            let mut inner_update = false;
            for j in 0..4 {
                let dt = (dx[j], dy[j]);
                inner_update = expand(&mut buffer[0], i, &input, dt, true) || update;
            }
            update = (update || inner_update);
        }
        if !update {
            break;
        }
    }

    for i in 0..n {
        let (a, b, c, d) = buffer[0][i].get_pos();
        println!("{} {} {} {}", a, b, c, d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_test() {
        let mut aabb = Aabb::new((0, 0, 3, 2));
        assert_eq!(aabb.calc_area(), 6);
        assert_eq!(aabb.contain((0, 0)), true);
        assert_eq!(aabb.contain((1, 1)), true);
        assert_eq!(aabb.contain((0, 1)), true);
        assert_eq!(aabb.contain((2, 0)), true);
        assert_eq!(aabb.contain((0, 2)), false);
        assert_eq!(aabb.contain((1, 2)), false);
        assert_eq!(aabb.contain((3, 0)), false);
        assert_eq!(aabb.contain((3, 2)), false);

        let mut aabb = Aabb::new((1, 2, 5, 7));
        assert_eq!(aabb.get_pos(), (1, 2, 5, 7));

        aabb.inflate((2, 1));
        assert_eq!(aabb.min_pos, (1, 2));
        assert_eq!(aabb.max_pos, (7, 8));
        aabb.inflate((0, -1));
        assert_eq!(aabb.min_pos, (1, 1));
        assert_eq!(aabb.max_pos, (7, 8));
        aabb.inflate((0, 0));
        assert_eq!(aabb.min_pos, (1, 1));
        assert_eq!(aabb.max_pos, (7, 8));

        aabb.deflate((-2, -1));
        assert_eq!(aabb.min_pos, (1, 1));
        assert_eq!(aabb.max_pos, (5, 7));
        aabb.inflate((0, 1));
        assert_eq!(aabb.min_pos, (1, 1));
        assert_eq!(aabb.max_pos, (5, 8));
        aabb.deflate((100, 100));
        assert_eq!(aabb.min_pos, (5, 8));
        assert_eq!(aabb.max_pos, (5, 8));

        aabb.inflate((-100, -100));
        aabb.inflate((10000, 10000));
        assert_eq!(aabb.min_pos, (0, 0));
        assert_eq!(aabb.max_pos, (10000, 10000));

        let aabb0 = Aabb::new((1, 1, 3, 3));
        let aabb1 = Aabb::new((0, 0, 1, 1));
        assert_eq!(aabb0.contact(&aabb1), false);
        let aabb1 = Aabb::new((0, 0, 1, 3));
        assert_eq!(aabb0.contact(&aabb1), false);
        let aabb1 = Aabb::new((0, 0, 2, 3));
        assert_eq!(aabb0.contact(&aabb1), true);

        let mut aabb = Aabb::new((1, 1, 3, 3));
        assert_eq!(aabb.translation((2, 4)), Some((2, 4)));
        assert_eq!(aabb.get_pos(), (3, 5, 5, 7));
        assert_eq!(aabb.translation((-2, -4)), Some((-2, -4)));
        assert_eq!(aabb.get_pos(), (1, 1, 3, 3));
        assert_eq!(aabb.translation((-2, 0)), Some((-1, 0)));
        assert_eq!(aabb.get_pos(), (0, 1, 2, 3));
        assert_eq!(aabb.translation((-1, 0)), None);
        assert_eq!(aabb.get_pos(), (0, 1, 2, 3));
        assert_eq!(aabb.translation((-1, -2)), Some((0, -1)));
        assert_eq!(aabb.get_pos(), (0, 0, 2, 2));
        assert_eq!(aabb.translation((10005, 10005)), Some((9998, 9998)));
        assert_eq!(aabb.get_pos(), (9998, 9998, 10000, 10000));
        assert_eq!(aabb.translation((1, 1)), None);
    }

    #[test]
    fn score_test() {
        let area = (0, 0, 2, 2);
        let data = Input { x: 0, y: 0, area: 1 };
        assert_eq!(score(area, &data), 7.0 / 16.0);
        let data = Input { x: 0, y: 0, area: 2 };
        assert_eq!(score(area, &data), 3.0 / 4.0);
        let data = Input { x: 0, y: 0, area: 3 };
        assert_eq!(score(area, &data), 15.0 / 16.0);
        let data = Input { x: 0, y: 0, area: 4 };
        assert_eq!(score(area, &data), 1.0);
        let data = Input { x: 0, y: 0, area: 5 };
        assert_eq!(score(area, &data), 24.0 / 25.0);
        let data = Input { x: 0, y: 0, area: 6 };
        assert_eq!(score(area, &data), 32.0 / 36.0);
        let data = Input { x: 2, y: 2, area: 4 };
        assert_eq!(score(area, &data), 0.0);
    }
}