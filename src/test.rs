use std::{collections::VecDeque, mem, rc::Rc, cell::RefCell, fmt::Debug, ops::Deref};

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

#[derive(Debug, Clone)]
struct MyRc<T: Debug>(Rc<T>);

impl <T: Debug> MyRc<T> {
    fn new(x: T) -> Self {
        Self(Rc::new(x))
    }
}

impl<T: Debug> Drop for MyRc<T> {
    fn drop(&mut self) {
        println!("I'm dropped: {:?}", &self);
        drop(self)
    }
}

impl<T: Debug> Deref for MyRc<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_drop_rc_in_vec() {
    let mut v: Vec<MyRc<i32>> = Vec::new();
    for i in 0..10 {
        v.push(MyRc::new(i));
    }

    let mut vd: VecDeque<MyRc<i32>> = VecDeque::new();
    println!("shit!!!!!!!!!!!!!!!!!");
    for _ in 0..10 {
        vd.push_back(v[3].clone());
        vd.pop_front();
    }
    println!("fuck!!!!!!!!!!!!!!!!!");
    println!("size of MyRc: {}", size_of(&v[6]));

    let mut vv: Vec<MyRc<i32>> = Vec::new();
    vv.push(v[2].clone());
    println!("lalala");
    for i in 7..10 {
        // vv[0] = v[i].clone();
        // let _ = std::mem::replace(&mut vv[0], v[i].clone());
        println!("before");
        vv[0] = v[i].clone();
        println!("{:?}, after", vv[0]);
    }
    // vv[0] = v[9].clone();
    println!("bdadbdaldkf");

    let mut a = MyRc::new(99999);
    println!("fucking before");
    a = v[5].clone();
    println!("{:?}", a);
    println!("fucking after");
}