#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
)]

use std::cmp::Ordering;
use anyhow::{
    Result,
    anyhow,
};

#[derive(Debug, Clone)]
struct Pair<T: PartialOrd> {
    a: Box<Value<T>>,
    b: Box<Value<T>>,

    data: T,
}

#[derive(Debug, Clone)]
enum Value<T: PartialOrd> {
    Data(T),
    Pair(Box<Pair<T>>),
}

impl<T: PartialOrd + Clone> Value<T> {
    fn combine(a: Self, b: Self) -> Self {
        let data = match a.clone() {
            Self::Data(inner) => inner,
            Self::Pair(inner) => inner.data,
        };

        Self::Pair(Box::new(Pair {
            a: Box::new(a),
            b: Box::new(b),

            data,
        }))
    }

    fn inner(&self) -> T {
        match self.clone() {
            Self::Data(d) => d,
            Self::Pair(p) => p.data,
        }
    }
}

impl<T: PartialOrd + Clone + std::fmt::Debug> Value<T> {
    fn get_pair(&self) -> Result<Pair<T>> {
        match self {
            Self::Data(d) => Err(anyhow!("Wanted pair, got data {:?}", d)),
            Self::Pair(p) => Ok(*p.clone()),
        }
    }
}

impl<T: PartialOrd + Clone> PartialEq for Value<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl<T: PartialOrd + Clone> PartialOrd for Value<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner().partial_cmp(&other.inner())
    }
}

fn jacobsthal_indices(limit: usize) -> Vec<usize> {
    let mut primes = vec![1];
    let mut sequence = vec![1, 1];

    while primes[primes.len() - 1] < limit {
        let length = sequence.len();
        let next = sequence[length - 1] + 2 * sequence[length - 2]; 
        sequence.push(next);

        if is_prime(next) {
            primes.push(next);
        }
    }

    let mut result = vec![1];
    for index in 1..primes.len() {
        let next_decending = ((primes[index - 1] + 1)..primes[index]).rev();

        result.push(primes[index]);
        result.extend(next_decending);
    }

    result.into_iter().filter(|x| x <= &limit).map(|x| x - 1).collect()
}

const fn sqrt(num: usize) -> usize {
    let mut v = 1;

    while (v + 1) * (v + 1) < num {
        v += 1;
    }

    v
}

fn is_prime(number: usize) -> bool {
    if number <= 3 {
        return true;
    }

    (2..=(sqrt(number)))
        .map(|e| number % e)
        .reduce(std::cmp::Ord::min)
        .unwrap() != 0
}

fn insert_sorted<T: PartialOrd>(list: &mut Vec<T>, to_insert: T) {
    let mut bounds = (0, list.len());
    let mut index = (bounds.0 + bounds.1) / 2;

    while bounds.1 - bounds.0 > 1 {
        if list[index] > to_insert {
            bounds.1 = index;
        } else if list[index] < to_insert {
            bounds.0 = index;
        } else {
            break;
        }

        index = (bounds.0 + bounds.1) / 2;
    }

    let offset = if list.is_empty() {
        0
    } else {
        usize::from(to_insert > list[index])
    };

    list.insert(index + offset, to_insert);
}

fn recurse<T: PartialOrd + Clone + std::fmt::Debug>(mut list: Vec<Value<T>>) -> Vec<Value<T>> {
    if list.len() == 1 {
        return list;
    }

    let extra = if list.len() % 2 == 1 {
        list.pop() // always will have a value
    } else {
        None
    };

    let mut merged = vec![];
    while let (Some(a), Some(b)) = (list.pop(), list.pop()) {
        if a > b {
            merged.push(Value::combine(a, b));
        } else {
            merged.push(Value::combine(b, a));
        }
    }

    let sorted = recurse(merged);
    let mut a_list = sorted.clone().into_iter().map(|e| *e.get_pair().unwrap().a).collect();
    let b_list: Vec<Box<Value<T>>> = sorted.clone().into_iter().map(|e| e.get_pair().unwrap().b).collect();
    let jacobsthal = jacobsthal_indices(sorted.len());

    for index in jacobsthal {
        let value_at_index = *b_list[index].clone();

        insert_sorted(&mut a_list, value_at_index);
    }

    if let Some(e) = extra {
        insert_sorted(&mut a_list, e);
    }

    a_list
}

fn merge_insertion_sort<T: PartialOrd + Clone + std::fmt::Debug>(list: Vec<T>) -> Vec<T> {
    let boxes = list.into_iter().map(|e| Value::Data(e)).collect();
    recurse(boxes).into_iter().map(|v| {
        match v {
            Value::Data(d) => Ok(d),
            Value::Pair(p) => Err(anyhow!("Expected data, got pair {:?}", p)),
        }.unwrap()
    }).collect()
}

fn main() {
    let list = vec![3, 5, 1, 2, 7];
    
    println!("{:?}", merge_insertion_sort(list));
}
