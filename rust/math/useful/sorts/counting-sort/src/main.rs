use std::convert::TryInto;

fn counting_sort(list: &mut Vec<i32>) {
    if list.is_empty() {
        return;
    }

    let max = *list.iter().max().unwrap();
    let min = *list.iter().min().unwrap();

    let mut counts: Vec<usize> = vec![0; <i32 as TryInto<usize>>::try_into(max - min).unwrap() + 1];

    for number in &mut *list {
        let index: usize = (*number - min).try_into().unwrap();
        counts[index] += 1;
    }

    let mut built = vec![];
    for (index, _) in counts.iter().enumerate() {
        for _ in 0..counts[index] {
            built.push(<usize as TryInto<i32>>::try_into(index).unwrap() + min);
        }
    }
    
    *list = built;
}

fn main() {
    let mut list = vec![8, 5, 5, -1];
    counting_sort(&mut list);
    println!("{:?}", list);
}
