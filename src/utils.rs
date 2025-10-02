use crate::Token;
use std::mem;

// Find the start of any comments that precede the given position
// Comments are included if they're separated by at most one newline and any spaces/tabs
pub(crate) fn find_preceding_comment_start(tokens: &[Token], idx: usize) -> usize {
    if idx == 0 {
        return 0;
    }
    
    let mut i = idx;
    
    while i > 0 {
        i -= 1;
        
        match tokens[i] {
            Token::Comment(_) => {
                // Found a comment! Continue looking backward for more comments
                // that might be connected to this one
                continue;
            }
            Token::Space | Token::Tab => {
                // Whitespace is okay, continue looking
            }
            Token::NewLine => {
                // Check if the previous token is also a newline
                if i > 0 && tokens[i-1] == Token::NewLine {
                    // Two newlines found - return position after them
                    return i + 1;
                }
                // Single newline is okay, continue
            }
            _ => {
                // Hit a non-whitespace, non-comment token, return position after it
                return i + 1;
            }
        }
    }
    
    // Reached start of file, return 0
    0
}



// Check if parameters contain type keywords
pub(crate) fn has_type_keywords_in_params(tokens: &[Token]) -> bool {
    let mut inside_parens = false;
    let mut paren_depth = 0;
    
    for token in tokens {
        match token {
            Token::OpenParen => {
                inside_parens = true;
                paren_depth += 1;
            }
            Token::CloseParen => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    inside_parens = false;
                }
            }
            Token::Object(obj) if inside_parens && paren_depth == 1 => {
                if matches!(
                    *obj,
                    "void" | "int" | "char" | "float" | "double" | "long" 
                    | "short" | "unsigned" | "signed" | "struct" | "union" 
                    | "enum" | "const" | "volatile" | "restrict"
                ) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

// Check for operators inside parameters (suggests call, not declaration)
pub(crate) fn has_operators_in_params(tokens: &[Token]) -> bool {
    let mut inside_parens = false;
    let mut paren_depth = 0;
    
    for token in tokens {
        match token {
            Token::OpenParen => {
                inside_parens = true;
                paren_depth += 1;
            }
            Token::CloseParen => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    inside_parens = false;
                }
            }
            Token::Plus | Token::Minus | Token::Asterisk | Token::ForwardSlash 
            | Token::Carrot | Token::Period | Token::OpenSquareBracket 
                if inside_parens && paren_depth == 1 => {
                return true;
            }
            _ => {}
        }
    }
    false
}


/// Updates `idx` to point to the next token specified. If the
/// token does not exist, `idx` will be set equal to tokens.len()
pub(crate) fn skip_to(tokens: &[Token], target: Token, idx: &mut usize) {
    for i in (*idx + 1)..tokens.len() {
        *idx = i;
        if tokens[i] == target {
            return;
        }
    }

    // If the for loop ends, we haven't found it, so set idx appropriately
    *idx = tokens.len();
}

/// Ignores values inside the targets, it just skips to the next token
/// that's one of the target variants
pub(crate) fn skip_to_oneof(tokens: &[Token], targets: &[Token], idx: &mut usize) {
    for i in (*idx + 1)..tokens.len() {
        *idx = i;
        for target in targets {
            if mem::discriminant(&tokens[i]) == mem::discriminant(target) {
                return;
            }
        }
    }
}

/// If we have a block commant (or multiple single line comments seperated by no more than a single \n character),
/// this function will skip to the end of all of them (including the trailing newline if it exists).
pub(crate) fn skip_to_end_comment(tokens: &[Token], idx: &mut usize) {
    assert_eq!(
        mem::discriminant(&tokens[*idx]),
        mem::discriminant(&Token::Comment(""))
    );

    while *idx < tokens.len()
        && (mem::discriminant(&tokens[*idx]) == mem::discriminant(&Token::Comment(""))
            || tokens[*idx] == Token::NewLine)
    {
        if tokens[*idx] == Token::NewLine && *idx+1 < tokens.len() && tokens[*idx + 1] == Token::NewLine {
            break;
        }
        *idx += 1;
    }
}