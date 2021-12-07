use std::{collections::VecDeque, mem};

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