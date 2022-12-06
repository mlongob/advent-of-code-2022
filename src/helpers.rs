/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::collections::VecDeque;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    crates: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.crates.pop_front()
    }

    pub fn push(&mut self, item: T) {
        self.crates.push_front(item);
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<T> {
        self.crates.drain(0..n).collect()
    }

    pub fn push_n(&mut self, items: Vec<T>) {
        items.into_iter().rev().for_each(|i| self.push(i));
    }

    pub fn top_item(&self) -> Option<&T> {
        self.crates.front()
    }

    pub fn new() -> Stack<T> {
        let crates = VecDeque::new();
        Stack { crates }
    }
}
