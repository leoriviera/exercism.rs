use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|items| format!("For want of a {} the {} was lost.", items[0], items[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect::<Vec<_>>()
        .join("\n")
}
