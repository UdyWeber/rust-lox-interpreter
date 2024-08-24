fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

pub(crate) fn is_digit(val: char) -> bool {
    val >= '0' && val <= '9'
}

pub(crate) fn is_alpha_numeric(c: char) -> bool {
    return is_digit(c) || is_alpha(c);
}