//! Repairing text spacing from rstml's raw token stream.

const TIGHT_START: &str = ",.;:!?)]}%/'";
const TIGHT_END: &str = "([{/-#";

pub(crate) fn starts_tight(text: &str) -> bool {
    text.chars()
        .next()
        .is_some_and(|ch| TIGHT_START.contains(ch))
}

pub(crate) fn ends_tight(text: &str) -> bool {
    text.chars().last().is_some_and(|ch| TIGHT_END.contains(ch))
}

/// Rebuild readable text from rstml's raw token stream, which loses the
/// original whitespace and inserts spaces around punctuation.
pub(crate) fn clean_text(raw: &str) -> String {
    if raw.trim().is_empty() {
        // A single space between two expressions on the same line is
        // significant; whitespace that spans lines is JSX indentation.
        return if raw.contains('\n') || raw.is_empty() {
            String::new()
        } else {
            " ".to_string()
        };
    }
    let mut text = raw.split_whitespace().collect::<Vec<_>>().join(" ");

    for mark in [",", ";", ":", ".", "!", "?", ")", "]", "}", "%"] {
        text = text.replace(&format!(" {mark}"), mark);
    }
    for mark in ["(", "[", "{"] {
        text = text.replace(&format!("{mark} "), mark);
    }
    text = text
        .replace(" / ", "/")
        .replace(" /", "/")
        .replace("/ ", "/");
    text = text
        .replace(" + ", "+")
        .replace(" +", "+")
        .replace("+ ", "+");
    text = text.replace("# ", "#");
    text = text.replace("/: ", "/:");
    text = tighten_hyphens(&text);
    text = separate_sentences(&text);

    for sign in ["-", "!"] {
        if let Some(rest) = text.strip_prefix(&format!("{sign} ")) {
            text = format!("{sign}{rest}");
            break;
        }
    }

    text.trim().to_string()
}

/// Rust token printing glues `First.` to `Second`; reopen the gap in front of
/// a capitalized word while keeping `open-meteo.com` tight.
fn separate_sentences(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(chars.len() + 4);
    for (index, &ch) in chars.iter().enumerate() {
        out.push(ch);
        if ch == '.' && chars.get(index + 1).is_some_and(|next| next.is_uppercase()) {
            out.push(' ');
        }
    }
    out
}

/// `open - meteo` comes from `open-meteo`; rejoin hyphens that sit between
/// words. Signs (` -5`) and sentence punctuation (` - `) are left alone.
fn tighten_hyphens(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out: Vec<char> = Vec::with_capacity(chars.len());
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch == '-' {
            let left_word =
                out.len() >= 2 && out[out.len() - 1] == ' ' && out[out.len() - 2].is_alphanumeric();
            let right_word = index + 2 < chars.len()
                && chars[index + 1] == ' '
                && chars[index + 2].is_alphanumeric();
            if left_word && right_word {
                out.pop();
                out.push('-');
                index += 2;
                continue;
            }
        }
        out.push(ch);
        index += 1;
    }
    out.into_iter().collect()
}
