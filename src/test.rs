use std::{collections::VecDeque, mem, rc::Rc, cell::RefCell};

use colored::Colorize;

use crate::utils::is_borrowed;

#[test]
fn test_vec_deque() {
    let mut deq = VecDeque::<i32>::new();
    deq.reserve(32);
    print_size(&deq);

    for i in 0..11 {
        deq.push_back(i);
    }
    deq.pop_front();
    print_size(&deq);

    for i in 10..100 {
        deq.push_back(i);
        deq.pop_front();
        println!("\n in i: {}", i);
        print_size(&deq)
    }
    print_size(&deq);
    println!("{:?}", deq);

    let deq2 = deq.clone();
    let x: Vec<i32> = deq2.into_iter().collect();
    println!("{:?}", x)
}

fn print_size<T>(v: &VecDeque<T>) {
    println!("{}", v.len());
    print_addr(v);
}

fn print_addr<T>(v: &VecDeque<T>) {
    let s = v.as_slices();
    println!("p0: {:p}/{}, P1: {:p}/{}", s.0, s.0.len(), s.1, s.1.len())
}

#[test]
fn test_vec() {
    let v: Vec<String> = Vec::with_capacity(10);
    println!("len: {}, cap: {}", v.len(), v.capacity())
}

#[derive(Default)]
struct BigBlock1 {
    a: Vec<String>,
    b: Vec<Vec<Vec<String>>>,
    c: Vec<String>,
}

#[derive(Default)]
struct BigBlock2 {
    a: Vec<BigBlock1>,
    b: Vec<BigBlock1>,
}

#[test]
fn test() {
    let bb2: Rc<RefCell<BigBlock2>> = Rc::default();
    println!("bb2 size: {}", size_of(&bb2));
}

fn size_of<T>(_: &T) -> usize {
    mem::size_of::<T>()
}

#[test]
fn test_replace_all() {
    let re = regex::Regex::new(r"(?P<matched>v\d+)").unwrap();
    let text = "v1, av2, cdvv32bdv33";

    let r = re.replace_all(text, "$matched".blue().to_string());
    println!("{}, is borrowed: {}", &r, is_borrowed(&r));
}