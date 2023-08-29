use pyo3::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use fancy_regex::*;
use blake2::{Blake2b, Digest};
use rand::prelude::SliceRandom;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
#[macro_use]
extern crate lazy_static;

const UWUIFY_EMOJIS: &[&str] = &["<3", ":3", "UwU", "OwO", "^^", ">w<", ">~<", ">.<", ">w<", "^w^", "(◕ᴥ◕)", "ʕ•ᴥ•ʔ", "ʕ￫ᴥ￩ʔ", "(*^ω^)", "(◕‿◕✿)", "(*^.^*)", "(つ✧ω✧)つ", "(/ =ω=)/"];
const UWUIFY_PHRASES: &[&str] = &["*snuzzles*", "*nuzzles*", "*paws*", "*purrs*", "*meows*", "*snugs*", "*pounces on you*", "*smiles*", "*kisses*", "*hugs*"];

// Define the simple rules for uwuification
lazy_static! {
    static ref UWUIFY_REPLACEMENTS: Vec<(&'static str, &'static str)> = vec![
        ("love", "luv"),
        ("nice", "nyaice"),
        ("what", "wut"),
        ("you", "u"),
        ("the ", "da "),
        ("aww", "uwu"),
        ("awesome", "uwusome"),
        ("r", "w"),
        ("l", "w"),
    ];
}

#[pyfunction]
fn uwuify(text: &str) -> PyResult<String> {   
    let mut hasher = Blake2b::new();
    hasher.update(text);
    let seed = hasher.finalize();
    let mut rng = StdRng::from_seed(seed.into());
    
    let mut text = text.to_owned();
    
    // Apply the simple rules first
    for (pattern, replacement) in UWUIFY_REPLACEMENTS.iter() {
        text = text.replace(pattern, replacement);
    }

    let old_text = text.clone().to_lowercase();
    let mut last_char = ' ';
    text.clear();
    for c in old_text.chars() {
        if c == '!' {
            text.push('!');
            text.push('!');
        }
        else if c == '?' {
            if rng.gen_bool(0.5) {
                text.push('!');
                text.push('?');
            }
            else {
                text.push('?');
                text.push('!');
            }
        }
        else if c == ',' {
            let emoji = UWUIFY_EMOJIS.choose(&mut rng).unwrap();
            text.push(' ');
            text.push_str(emoji);
            text.push(c);
        }
        else if last_char == 'n' && (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            text.push('y');
            text.push(c);
        }
        else {
            text.push(c);
        }

        if c != ' ' {
            last_char = c;
        }
    }

    let mut period = 0;
    let mut tracing_period = false;
    let mut offset = 0;
    for (i, c) in text.clone().char_indices() {
        if !tracing_period && c == '.' {
            period = i;
            tracing_period = true;
        }
        if tracing_period && c == ' ' {
            let emoji : &str = UWUIFY_EMOJIS.choose(&mut rng).unwrap();
            text.insert_str(period + offset, format!(" {}", emoji).borrow_mut());
            offset += emoji.char_indices().count() + 1;
            tracing_period = false;
        }
        else if tracing_period && c != ' ' {
            tracing_period = false;
        }
    }
    if tracing_period {
        let emoji : &str = UWUIFY_EMOJIS.choose(&mut rng).unwrap();
        text.pop();
        text.push(' ');
        text.push_str(emoji);
        text.push('.');
    }

    text = format!("{} {}", UWUIFY_PHRASES.choose(&mut rng).unwrap(), text);

    if last_char != '.' {
        text = format!("{} {}", text, UWUIFY_EMOJIS.choose(&mut rng).unwrap());
    }
    
    Ok(text)
}

#[pymodule]
fn uwuifier(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(uwuify, m)?)?;
    Ok(())
}
