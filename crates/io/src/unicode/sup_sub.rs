//
// DO NOT EDIT THIS FILE
// It was generated automatically via `cargo tools build-ucd`

#![allow(dead_code)]
#![allow(clippy::all)]

pub const SUPERSCRIPT_MEMBERSHIP: &'static [(char, char)] = &[
('ª', 'ª'), ('²', '³'), ('¹', 'º'), ('ʰ', 'ʸ'), ('ˠ', 'ˤ'), ('ჼ', 'ჼ'), ('ᴬ', 'ᴮ'), ('ᴰ', 'ᴺ'), ('ᴼ', 'ᵍ'), ('ᵏ', 'ᵡ'), ('ᵸ', 'ᵸ'), ('ᶛ', 'ᶿ'), ('⁰', 'ⁱ'), ('⁴', 'ⁿ'), ('℠', '℠'), ('™', '™'), ('ⱽ', 'ⱽ'), ('ⵯ', 'ⵯ'), ('㆒', '㆟'), ('ꚜ', 'ꚝ'), ('ꝰ', 'ꝰ'), ('ꟸ', 'ꟹ'), ('ꭜ', 'ꭟ'), ('🅪', '🅬'), ];
pub const SUBSCRIPT_MEMBERSHIP: &'static [(char, char)] = &[
('ᵢ', 'ᵪ'), ('₀', '₎'), ('ₐ', 'ₜ'), ('ⱼ', 'ⱼ'), ];

/// Takes a unicode codepoint for a super- or subscript character, and returns 
/// a &str of its decomposition. (&str not char, because ^TM for example is two chars.)
pub fn lookup_decomposition(cp: char) -> &'static str {
    match cp as char {
        'ª' => "a",
        '²' => "2",
        '³' => "3",
        '¹' => "1",
        'º' => "o",
        'ʰ' => "h",
        'ʱ' => "ɦ",
        'ʲ' => "j",
        'ʳ' => "r",
        'ʴ' => "ɹ",
        'ʵ' => "ɻ",
        'ʶ' => "ʁ",
        'ʷ' => "w",
        'ʸ' => "y",
        'ˠ' => "ɣ",
        'ˡ' => "l",
        'ˢ' => "s",
        'ˣ' => "x",
        'ˤ' => "ʕ",
        'ჼ' => "ნ",
        'ᴬ' => "A",
        'ᴭ' => "Æ",
        'ᴮ' => "B",
        'ᴰ' => "D",
        'ᴱ' => "E",
        'ᴲ' => "Ǝ",
        'ᴳ' => "G",
        'ᴴ' => "H",
        'ᴵ' => "I",
        'ᴶ' => "J",
        'ᴷ' => "K",
        'ᴸ' => "L",
        'ᴹ' => "M",
        'ᴺ' => "N",
        'ᴼ' => "O",
        'ᴽ' => "Ȣ",
        'ᴾ' => "P",
        'ᴿ' => "R",
        'ᵀ' => "T",
        'ᵁ' => "U",
        'ᵂ' => "W",
        'ᵃ' => "a",
        'ᵄ' => "ɐ",
        'ᵅ' => "ɑ",
        'ᵆ' => "ᴂ",
        'ᵇ' => "b",
        'ᵈ' => "d",
        'ᵉ' => "e",
        'ᵊ' => "ə",
        'ᵋ' => "ɛ",
        'ᵌ' => "ɜ",
        'ᵍ' => "g",
        'ᵏ' => "k",
        'ᵐ' => "m",
        'ᵑ' => "ŋ",
        'ᵒ' => "o",
        'ᵓ' => "ɔ",
        'ᵔ' => "ᴖ",
        'ᵕ' => "ᴗ",
        'ᵖ' => "p",
        'ᵗ' => "t",
        'ᵘ' => "u",
        'ᵙ' => "ᴝ",
        'ᵚ' => "ɯ",
        'ᵛ' => "v",
        'ᵜ' => "ᴥ",
        'ᵝ' => "β",
        'ᵞ' => "γ",
        'ᵟ' => "δ",
        'ᵠ' => "φ",
        'ᵡ' => "χ",
        'ᵢ' => "i",
        'ᵣ' => "r",
        'ᵤ' => "u",
        'ᵥ' => "v",
        'ᵦ' => "β",
        'ᵧ' => "γ",
        'ᵨ' => "ρ",
        'ᵩ' => "φ",
        'ᵪ' => "χ",
        'ᵸ' => "н",
        'ᶛ' => "ɒ",
        'ᶜ' => "c",
        'ᶝ' => "ɕ",
        'ᶞ' => "ð",
        'ᶟ' => "ɜ",
        'ᶠ' => "f",
        'ᶡ' => "ɟ",
        'ᶢ' => "ɡ",
        'ᶣ' => "ɥ",
        'ᶤ' => "ɨ",
        'ᶥ' => "ɩ",
        'ᶦ' => "ɪ",
        'ᶧ' => "ᵻ",
        'ᶨ' => "ʝ",
        'ᶩ' => "ɭ",
        'ᶪ' => "ᶅ",
        'ᶫ' => "ʟ",
        'ᶬ' => "ɱ",
        'ᶭ' => "ɰ",
        'ᶮ' => "ɲ",
        'ᶯ' => "ɳ",
        'ᶰ' => "ɴ",
        'ᶱ' => "ɵ",
        'ᶲ' => "ɸ",
        'ᶳ' => "ʂ",
        'ᶴ' => "ʃ",
        'ᶵ' => "ƫ",
        'ᶶ' => "ʉ",
        'ᶷ' => "ʊ",
        'ᶸ' => "ᴜ",
        'ᶹ' => "ʋ",
        'ᶺ' => "ʌ",
        'ᶻ' => "z",
        'ᶼ' => "ʐ",
        'ᶽ' => "ʑ",
        'ᶾ' => "ʒ",
        'ᶿ' => "θ",
        '⁰' => "0",
        'ⁱ' => "i",
        '⁴' => "4",
        '⁵' => "5",
        '⁶' => "6",
        '⁷' => "7",
        '⁸' => "8",
        '⁹' => "9",
        '⁺' => "+",
        '⁻' => "−",
        '⁼' => "=",
        '⁽' => "(",
        '⁾' => ")",
        'ⁿ' => "n",
        '₀' => "0",
        '₁' => "1",
        '₂' => "2",
        '₃' => "3",
        '₄' => "4",
        '₅' => "5",
        '₆' => "6",
        '₇' => "7",
        '₈' => "8",
        '₉' => "9",
        '₊' => "+",
        '₋' => "−",
        '₌' => "=",
        '₍' => "(",
        '₎' => ")",
        'ₐ' => "a",
        'ₑ' => "e",
        'ₒ' => "o",
        'ₓ' => "x",
        'ₔ' => "ə",
        'ₕ' => "h",
        'ₖ' => "k",
        'ₗ' => "l",
        'ₘ' => "m",
        'ₙ' => "n",
        'ₚ' => "p",
        'ₛ' => "s",
        'ₜ' => "t",
        '℠' => "SM",
        '™' => "TM",
        'ⱼ' => "j",
        'ⱽ' => "V",
        'ⵯ' => "ⵡ",
        '㆒' => "一",
        '㆓' => "二",
        '㆔' => "三",
        '㆕' => "四",
        '㆖' => "上",
        '㆗' => "中",
        '㆘' => "下",
        '㆙' => "甲",
        '㆚' => "乙",
        '㆛' => "丙",
        '㆜' => "丁",
        '㆝' => "天",
        '㆞' => "地",
        '㆟' => "人",
        'ꚜ' => "ъ",
        'ꚝ' => "ь",
        'ꝰ' => "ꝯ",
        'ꟸ' => "Ħ",
        'ꟹ' => "œ",
        'ꭜ' => "ꜧ",
        'ꭝ' => "ꬷ",
        'ꭞ' => "ɫ",
        'ꭟ' => "ꭒ",
        '🅪' => "MC",
        '🅫' => "MD",
        '🅬' => "MR",
        _ => panic!("unknown codepoint in lookup_decomposition lookup function")
    }
}
