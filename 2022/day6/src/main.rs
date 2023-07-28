use std::{collections::VecDeque, fs};

const MESSAGE_HEADER_SIZE: usize = 14;
const PACKET_HEADER_SIZE: usize = 4;

fn main() {
    println!("Welcome to Marker-Finder");

    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    println!(
        "Marker found at: {}",
        find_first_message(&input).expect("Input stream did not contain any marker")
    );
}

fn find_first_packet(stream: &String) -> Option<usize> {
    find_thing(&stream, PACKET_HEADER_SIZE)
}

fn find_first_message(stream: &String) -> Option<usize> {
    find_thing(&stream, MESSAGE_HEADER_SIZE)
}

fn find_thing(stream: &String, chars_before_repeating: usize) -> Option<usize> {
    let mut context = Context::new(chars_before_repeating);

    for (index, ch) in stream.chars().enumerate() {
        context.push(ch);

        if context.is_full() && !context.has_duplicates() {
            return Some(index + 1);
        }
    }

    None
}

struct Context {
    data: VecDeque<char>,
    max_size: usize,
}

impl Context {
    fn new(size: usize) -> Context {
        Context {
            data: VecDeque::new(),
            max_size: size,
        }
    }

    fn push(&mut self, to_add: char) {
        self.data.push_front(to_add);
        if self.data.len() > self.max_size {
            self.data.pop_back();
        }
    }

    fn has_duplicates(&self) -> bool {
        for (index, item) in self.data.iter().enumerate() {
            for (index2, item2) in self.data.iter().enumerate() {
                if item == item2 && index != index2 {
                    return true;
                }
            }
        }

        false
    }

    fn is_full(&self) -> bool {
        self.data.len() >= self.max_size
    }
}
