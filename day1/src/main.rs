use itertools::{FoldWhile, Itertools};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let lines = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let max = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max();
    println!("{max:?}");

    let lines = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok());
    let max = GroupSumIter { inner: lines }.max();
    println!("{max:?}");

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();
    println!("{max:?}");

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .max()
        .flatten()
        .unwrap_or_default();
    println!("{max:?}");

    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .flatten()
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .sum::<u64>();
    println!("{answer:?}");

    let mut group_sums = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            it.fold_while(None, |acc: Option<u64>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        .map(Reverse);

    let mut heap = BinaryHeap::new();

    for init in (&mut group_sums).take(3) {
        heap.push(init);
    }

    for rest in group_sums {
        heap.push(rest);
        heap.pop();
    }

    let answer = heap.into_iter().map(|Reverse(v)| v).sum::<u64>();
    println!("{answer:?}");

    Ok(())
}

/// An iterator that takes `Option<u64>` items and yields sums of groups of
/// `Some(u64)` items separated by `None` items.
struct GroupSumIter<I> {
    inner: I,
}

impl<I> Iterator for GroupSumIter<I>
where
    I: Iterator<Item = Option<u64>>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = loop {
            match self.inner.next() {
                Some(Some(v)) => break v,
                Some(None) => {}
                // we've reached the end of the inner iterator
                None => return None,
            }
        };

        loop {
            match self.inner.next() {
                Some(Some(v)) => sum += v,
                Some(None) | None => {
                    // reached a separator or the end of the iterator
                    break Some(sum);
                }
            }
        }
    }
}
