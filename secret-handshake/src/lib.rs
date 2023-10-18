pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = Vec::new();

    for i in 0..5 {
        let d = (n >> i) & 1;

        match d * (i + 1) {
            1 => actions.push("wink"),
            2 => actions.push("double blink"),
            3 => actions.push("close your eyes"),
            4 => actions.push("jump"),
            5 => actions.reverse(),
            _ => continue,
        };
    }

    actions
}
