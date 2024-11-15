use std::iter::Peekable;

use super::{
    item::{Attribute, WppItem},
    tokenizer::{self, Token},
};

// Responsible for parsing the entire w++ entry
pub fn parse(input: &str) -> Result<WppItem, String> {
    let mut tokens = tokenizer::tokenize(input).into_iter().peekable();
    let (iden, name) = parse_header(&mut tokens)?;
    let mut item = WppItem::new(&iden, &name);
    let attributes = parse_attributes(&mut tokens)?;
    for attribute in attributes {
        item.add_attribute(attribute);
    }
    if let Some(Token::RightBracket) = tokens.next() {
    } else {
        return Err(format!(
            "Expected `]` got {:?}, while parsing character",
            tokens.next()
        ));
    }
    Ok(item)
}

// Responsible for parsing the header of the w++ entry e.g. `[Character("Nika Orchid")\n{`
fn parse_header(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<(String, String), String> {
    if let Some(Token::LeftBracket) = tokens.next() {
    } else {
        return Err("Expected `[`".to_string());
    }
    let iden = match tokens.next() {
        Some(Token::Identifier(iden)) => iden,
        _ => return Err("Expected w++ entry name".to_string()),
    };
    if let Some(Token::LeftParen) = tokens.next() {
    } else {
        return Err("Expected `(`".to_string());
    }
    let name = match tokens.next() {
        Some(Token::StringLiteral(name)) => name,
        _ => return Err("Expected string literal".to_string()),
    };
    if let Some(Token::RightParen) = tokens.next() {
    } else {
        return Err("Expected `)`, while parsing character".to_string());
    }
    if let Some(Token::LeftBrace) = tokens.next() {
    } else {
        return Err("Expected `{`".to_string());
    }
    Ok((iden, name))
}

// Responsible for parsing the values of an attribute e.g. `ThisPartHere("value1" + "value2")`
fn parse_attributes(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Attribute>, String> {
    let mut attributes = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Identifier(key) => {
                let values = parse_attribute_values(tokens)?;
                let mut attribute = Attribute::new(&key);
                for value in values {
                    attribute.add_value(&value);
                }
                attributes.push(attribute);
            }
            Token::RightBrace => break,
            _ => {
                return Err(format!(
                    "Expected `}}` got {:?}, while parsing attribute",
                    token
                ))
            }
        }
    }
    Ok(attributes)
}

// Responsible for parsing the values of an attribute e.g. `Nickname(THIS PART HERE)`
fn parse_attribute_values(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<String>, String> {
    let mut values = Vec::new();
    if let Some(Token::LeftParen) = tokens.peek() {
        tokens.next();
    } else {
        return Err("Expected `(`".to_string());
    }
    for token in tokens.by_ref() {
        match token {
            Token::StringLiteral(value) => {
                values.push(value.clone());
            }
            Token::Plus => {
                continue;
            }
            Token::RightParen => {
                break;
            }
            _ => {
                return Err(format!(
                    "Expected `)` got {:?}, while parsing attribute values",
                    token
                ))
            }
        }
    }
    Ok(values)
}
