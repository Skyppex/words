use crate::args::WordsArgs;

pub fn run(input: String, args: WordsArgs) -> String {
    let input = input.replace(['\r', '\n'], " ");
    let mut items = get_items(input, args.clone());

    if args.contains.is_some() {
        let contains = args.clone().contains.unwrap();

        if args.case_sensitive {
            items.retain(|s| s.contains(&contains));
        } else {
            items.retain(|s| s.to_lowercase().contains(&contains.to_lowercase()));
        }
    }

    if args.first.is_some() {
        items = items
            .into_iter()
            .take(args.first.unwrap().unwrap_or(1) as usize)
            .collect();
    }

    if args.last.is_some() {
        items = items
            .into_iter()
            .rev()
            .take(args.last.unwrap().unwrap_or(1) as usize)
            .collect();
    }

    if args.output.count {
        return items.len().to_string();
    }

    if items.is_empty() {
        return String::new();
    }

    let items = alter_output(items, args.clone());

    join_items(items, args)
}

fn get_items(input: String, args: WordsArgs) -> Vec<String> {
    if args.sentence {
        return input
            .split('.')
            .flat_map(|s| s.split('!'))
            .flat_map(|s| s.split('?'))
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();
    }

    return input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>();
}

fn alter_output(items: Vec<String>, args: WordsArgs) -> Vec<String> {
    let mut items = items;
    let output = args.output;

    if output.no_punctuation {
        items = items
            .into_iter()
            .map(|s| s.chars().filter(|c| !c.is_ascii_punctuation()).collect())
            .collect();
    }

    if output.trim {
        items = items.into_iter().map(|s| s.trim().to_owned()).collect();
    }

    items
}

fn join_items(items: Vec<String>, args: WordsArgs) -> String {
    let output = args.output;

    if args.sentence {
        if output.list {
            return items.join(".\n") + ".";
        }

        if output.json {
            return format!("[\"{}\"]", items.join("\", \""));
        }

        return items.join(". ") + ".";
    }

    if output.list {
        return items.join("\n");
    }

    if output.json {
        return format!("[\"{}\"]", items.join("\", \""));
    }

    items.join(" ")
}

#[cfg(test)]
mod tests {}

