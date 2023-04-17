use std::collections::HashSet;

use item::Item;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut total_score = 0;

    for line in include_str!("input.txt").lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()?;

        let dup_score = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items
                        .iter()
                        .copied()
                        .find(|&first_item| first_item == item)
                })
            })
            .expect("there should be exactly one duplicate")
            .priority();
        dbg!(dup_score);
        total_score += dup_score;
    }
    dbg!(total_score);

    let sum = include_str!("input.txt")
        .lines()
        .map(|line| -> color_eyre::Result<_> {
            let (first, second) = line.split_at(line.len() / 2);
            let first_items = first
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;
            itertools::process_results(second.bytes().map(Item::try_from), |mut it| {
                it.find(|&item| first_items.contains(&item))
                    .map(|item| dbg!(item.priority()))
                    .ok_or_else(|| color_eyre::eyre::eyre!("compartments have no items in common"))
            })?
        })
        .sum::<color_eyre::Result<usize>>()?;
    dbg!(sum);

    let rucksacks = include_str!("input.txt").lines().map(|line| {
        line.bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    let sum = itertools::process_results(rucksacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .copied()
                    .find(|i| b.contains(i) && c.contains(i))
                    .map(|i| dbg!(i.priority()))
                    .unwrap_or_default()
            })
            .sum::<usize>()
    })?;
    dbg!(sum);

    Ok(())
}

mod item {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                    "{} is not a valid item",
                    value as char
                )),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl Item {
        pub(crate) fn priority(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
}
