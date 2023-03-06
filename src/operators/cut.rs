use crate::types::types::{Bobbins, Idx, Idxs, Subtours, Tour};

pub fn cut(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = vec![];
    let mut idxs: Idxs = tour
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| {
            if subset.contains(&node) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Idxs>();
    idxs.sort();
    let last_ix: Idx = tour.len() - 1;
    let mut prev: i32 = -1 as i32;
    for (e, idx) in idxs.iter().enumerate() {
        if e == idxs.len() - 1 && *idx != last_ix {
            for subtour in vec![
                tour[(prev + 1) as usize..*idx].to_vec(),
                tour[*idx..].to_vec(),
            ] {
                if !subtour.is_empty() {
                    if subset.contains(&subtour[0]) {
                        subtours.push(subtour)
                    } else {
                        subtours.push(subtour.into_iter().rev().collect())
                    }
                }
            }
        } else {
            let subtour: Tour = tour[(prev + 1) as usize..=*idx].to_vec();
            if !subtour.is_empty() {
                if subset.contains(&subtour[0]) {
                    subtours.push(subtour)
                } else {
                    subtours.push(subtour.iter().rev().cloned().collect())
                }
            }
            prev = *idx as i32
        }
    }
    subtours
}
