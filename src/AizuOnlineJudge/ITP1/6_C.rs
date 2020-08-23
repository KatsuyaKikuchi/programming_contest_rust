use proconio::input;
use proconio::marker::Usize1;

struct Floor
{
    rooms: Vec<i32>,
}

struct Building
{
    floors: Vec<Floor>,
}

impl Floor {
    fn new() -> Floor {
        Floor {
            rooms: vec![0; 10],
        }
    }

    fn set_value(&mut self, r: usize, value: i32) {
        self.rooms[r] += value;
    }

    fn print(&self) {
        for &r in self.rooms.iter() {
            print!(" {}", r);
        }
        println!("");
    }
}

impl Building {
    fn new() -> Building {
        let mut building = Building {
            floors: Vec::new(),
        };

        for _ in 0..3 {
            let floor = Floor::new();
            building.floors.push(floor);
        }
        building
    }

    fn set_value(&mut self, f: usize, r: usize, value: i32) {
        self.floors[f].set_value(r, value);
    }

    fn print(&self) {
        for f in self.floors.iter() {
            f.print();
        }
        println!("{}", "#".repeat(20));
    }
}


fn main()
{
    let mut buildings: Vec<Building> = Vec::new();
    for _ in 0..4 {
        let building = Building::new();
        buildings.push(building);
    }

    input! {
    n:usize,
    }

    for _ in 0..n {
        input! {
        (b,f,r,v):(Usize1,Usize1,Usize1,i32),
        }
        buildings[b].set_value(f, r, v);
    }

    for b in buildings.iter() {
        b.print();
    }
}