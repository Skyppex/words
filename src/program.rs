use crate::args::Args;

pub fn run(input: String, args: Args) -> String {
    let input = input.replace('\r', " ").replace('\n', " ");
    let mut items = get_items(input, args.clone());
    
    if args.contains.is_some() {
        let contains = args.clone().contains.unwrap();

        if args.case_sensitive {
            items = items.into_iter().filter(|s| s.contains(&contains)).collect();
        } else {
            items = items.into_iter().filter(|s| s.to_lowercase().contains(&contains.to_lowercase())).collect();
        }
    }

    if args.first.is_some() {
        items = items.into_iter().take(args.first.unwrap().unwrap_or(1) as usize).collect();
    }

    if args.last.is_some() {
        items = items.into_iter().rev().take(args.last.unwrap().unwrap_or(1) as usize).collect();
    }

    if items.len() == 0 {
        return String::new();
    }

    join_items(items, args)
}

fn get_items<'a>(input: String, args: Args) -> Vec<String> {
    if args.sentence {
        return input.split('.')
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();
    }

    return input.split_whitespace()
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>();
}

fn join_items(items: Vec<String>, args: Args) -> String {
    if args.sentence {
        if args.list {
            return items.join(".\n") + ".";
        }

        return items.join(". ") + ".";
    }

    if args.list {
        return items.join("\n");
    }

    return items.join(" ");
}

#[cfg(test)]
mod tests {
    
}