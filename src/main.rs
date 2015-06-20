extern crate image;

use image::GenericImage;

use std::path::Path;
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, Ord, Hash)]
struct ColorEntry {
    data: [u8; 4],
    pub count: u64,
}

impl ColorEntry {
    fn new(data: [u8; 4], count: u64) -> ColorEntry {
        ColorEntry {
            data: data,
            count: count,
        }
    }
}


impl PartialEq for ColorEntry {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for ColorEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.count < other.count {
            Some(Ordering::Less)
        } else if self.count > other.count {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }

}

fn main() {
    let img = image::open(&Path::new("test.jpg")).unwrap();
    let mut color_counts: HashMap<[u8; 4], u64> = HashMap::new();
    for (_, _, rgba) in img.pixels() {
        if color_counts.contains_key(&rgba.data) {
            *(color_counts.get_mut(&rgba.data).unwrap()) += 1;
        } else {
            color_counts.insert(rgba.data, 1);
        }
    }

    let mut heap = BinaryHeap::new();

    for (k, v) in color_counts {
        heap.push(ColorEntry::new(k, v));
    }
    println!("most used colors:");
    let color = heap.pop().unwrap();
    println!("rgb({}, {}, {})", color.data[0], color.data[1], color.data[2]);
    let color = heap.pop().unwrap();
    println!("rgb({}, {}, {})", color.data[0], color.data[1], color.data[2]);
    let color = heap.pop().unwrap();
    println!("rgb({}, {}, {})", color.data[0], color.data[1], color.data[2]);
}
