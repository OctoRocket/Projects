#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

use anyhow::{anyhow, Result};
use std::cmp::Ordering;

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

struct Comparator {
    count: usize,
}

impl Comparator {
    const fn new() -> Self {
        Self { count: 0 }
    }

    fn lt<T: PartialOrd>(&mut self, a: &T, b: &T) -> bool {
        self.count += 1;

        a < b
    }

    fn cmp<T: PartialOrd + >(&mut self, a: &T, b: &T) -> Ordering {
        self.count += 1;

        a.partial_cmp(b)
            .expect("Partial ord failed in comparator component")
    }
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

fn main() {
    let list = vec![13,88,38,94,17,91,65,96,30,66,36,6,37,39,22,85,48,83,56,97,87,92,2,74,0,5,14,27,95,99,68,86,69,23,42,29,24,40,44,21,78,70,79,20,93,8,98,57,53,34,50,15,1,10,62,11,63,43,64,58,90,80,4,89,12,55,76,35,16,77,75,31,61,28,72,47,18,73,3,59,19,71,32,51,67,9,25,54,26,7,46,49,84,33,81,45,82,41,60,52];
    let mut comparator = Comparator::new();

    println!("{:?}", merge_insertion_sort(list, &mut comparator));
    println!("Comparisons: {}", comparator.count);
}

fn merge_insertion_sort<T: PartialOrd + Clone + std::fmt::Debug>(
    list: Vec<T>,
    comparator: &mut Comparator,
) -> Vec<T> {
    let boxes = list.into_iter().map(|e| Value::Data(e)).collect();
    recurse(boxes, comparator)
        .into_iter()
        .map(|v| {
            match v {
                Value::Data(d) => Ok(d),
                Value::Pair(p) => Err(anyhow!("Expected data, got pair {:?}", p)),
            }
            .unwrap()
        })
        .collect()
}

fn recurse<T: PartialOrd + Clone + std::fmt::Debug>(
    mut list: Vec<Value<T>>,
    comparator: &mut Comparator,
) -> Vec<Value<T>> {
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
        if comparator.lt(&a, &b) {
            merged.push(Value::combine(a, b));
        } else {
            merged.push(Value::combine(b, a));
        }
    }

    let to_insert = recurse(merged, comparator);
    let jacobsthal = jacobsthal_indices(to_insert.len());
    let mut indices = (0..to_insert.len()).collect::<Vec<usize>>();
    let mut a_list = to_insert
        .clone()
        .into_iter()
        .map(|e| *e.get_pair().unwrap().a)
        .collect::<Vec<Value<T>>>();
    let b_list: Vec<Box<Value<T>>> = to_insert
        .into_iter()
        .map(|e| e.get_pair().unwrap().b)
        .collect();

    for index in jacobsthal {
        let value_at_index = *b_list[index].clone();
        let search_result =
            (a_list[indices[index]..]).binary_search_by(|v| comparator.cmp(v, &value_at_index));
        let location = match search_result {
            Ok(r) | Err(r) => r,
        } + indices[index];

        a_list.insert(location, value_at_index);
        indices = indices
            .clone()
            .into_iter()
            .map(|e| if e >= location { e + 1 } else { e })
            .collect();
    }

    if let Some(e) = extra {
        let search_result = &a_list.binary_search_by(|v| comparator.cmp(v, &e));
        a_list.insert(
            match search_result {
                Ok(r) | Err(r) => *r,
            },
            e,
        );
    }

    a_list
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

    result
        .into_iter()
        .filter(|x| x <= &limit)
        .map(|x| x - 1)
        .collect()
}

fn is_prime(number: usize) -> bool {
    if number <= 3 {
        return true;
    }

    (2..=(sqrt(number)))
        .map(|e| number % e)
        .reduce(std::cmp::Ord::min)
        .unwrap()
        != 0
}

const fn sqrt(num: usize) -> usize {
    let mut v = 1;

    while (v + 1) * (v + 1) < num {
        v += 1;
    }

    v
}
