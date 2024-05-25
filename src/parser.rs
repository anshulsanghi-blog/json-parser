use crate::token::{JsonTokenizer, Token};
use crate::value::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::iter::Peekable;
use std::slice::Iter;

/// Main parser which is the entrypoint for parsing JSON.
pub struct JsonParser;

impl JsonParser {
    /// Create a new [`JsonParser`] that parses JSON from bytes.
    pub fn parse_from_bytes<'a>(input: &'a [u8]) -> Result<Value, ()> {
        let mut json_tokenizer = JsonTokenizer::<BufReader<Cursor<&[u8]>>>::from_bytes(input);
        let tokens = json_tokenizer.tokenize_json()?;

        Ok(Self::tokens_to_value(tokens))
    }

    /// Create a new [`JsonParser`] that parses JSON from file.
    pub fn parse(reader: File) -> Result<Value, ()> {
        let mut json_tokenizer = JsonTokenizer::<BufReader<File>>::new(reader);
        let tokens = json_tokenizer.tokenize_json()?;

        Ok(Self::tokens_to_value(tokens))
    }

    fn tokens_to_value(tokens: &[Token]) -> Value {
        // Create a peekable iterator over tokens
        let mut iterator = tokens.iter().peekable();

        // Initialize final value to null.
        let mut value = Value::Null;

        // Loop while there are tokens in the iterator.
        // Note that you do not need to manually handle advancing the
        // iterator in this case which is why you can directly call
        // `iterator.next()`.
        while let Some(token) = iterator.next() {
            match token {
                Token::CurlyOpen => {
                    value = Value::Object(Self::process_object(&mut iterator));
                }
                Token::String(string) => {
                    value = Value::String(string.clone());
                }
                Token::Number(number) => {
                    value = Value::Number(*number);
                }
                Token::ArrayOpen => {
                    value = Value::Array(Self::process_array(&mut iterator));
                }
                Token::Boolean(boolean) => value = Value::Boolean(*boolean),
                Token::Null => value = Value::Null,
                // Ignore all delimiters as you don't need to explicitly do anything
                // when you encounter them.
                Token::Comma
                | Token::CurlyClose
                | Token::Quotes
                | Token::Colon
                | Token::ArrayClose => {}
            }
        }

        value
    }

    fn process_array(iterator: &mut Peekable<Iter<Token>>) -> Vec<Value> {
        // Initialise a vector of JSON Value type to hold the value of
        // array that's currently being parsed.
        let mut internal_value = Vec::<Value>::new();

        // Iterate over all tokens provided.
        while let Some(token) = iterator.next() {
            match token {
                Token::CurlyOpen => {
                    internal_value.push(Value::Object(Self::process_object(iterator)));
                }
                Token::String(string) => internal_value.push(Value::String(string.clone())),
                Token::Number(number) => internal_value.push(Value::Number(*number)),
                Token::ArrayOpen => {
                    internal_value.push(Value::Array(Self::process_array(iterator)));
                }
                // Break loop if array is closed. Due to recursive nature of process_array,
                // we don't need to explicitly check if the closing token matches the opening
                // one.
                Token::ArrayClose => {
                    break;
                }
                Token::Boolean(boolean) => internal_value.push(Value::Boolean(*boolean)),
                Token::Null => internal_value.push(Value::Null),
                // Ignore delimiters
                Token::Comma | Token::CurlyClose | Token::Quotes | Token::Colon => {}
            }
        }

        internal_value
    }

    fn process_object(iterator: &mut Peekable<Iter<Token>>) -> HashMap<String, Value> {
        // Whether the item being parsed is a key or a value. The first element
        // should always be a key so this is initialised to true.
        let mut is_key = true;

        // The current key for which the value is being parsed.
        let mut current_key: Option<&str> = None;

        // The current state of parsed object.
        let mut value = HashMap::<String, Value>::new();

        while let Some(token) = iterator.next() {
            match token {
                // If it is a nested object, recursively parse it and store
                // in the hashmap with current key.
                Token::CurlyOpen => {
                    if let Some(current_key) = current_key {
                        value.insert(
                            current_key.to_string(),
                            Value::Object(Self::process_object(iterator)),
                        );
                    }
                }
                // If this token is encountered, break the loop since it
                // indicates end of an object being parsed.
                Token::CurlyClose => {
                    break;
                }
                Token::Quotes | Token::ArrayClose => {}
                // If the token is a colon, it is the separator between key
                // and value pair. So the item being parsed from this point
                // ahead will not be a key.
                Token::Colon => {
                    is_key = false;
                }
                Token::String(string) => {
                    if is_key {
                        // If the process is presently parsing key, set the value
                        // as current key.
                        current_key = Some(string);
                    } else if let Some(key) = current_key {
                        // If the process already has a key set for present item,
                        // parse string as value instead, and set the current_key to none
                        // once done to prepare for the next key-value pair.
                        value.insert(key.to_string(), Value::String(string.clone()));
                        // Set current_key to None to prepare for the next key-value pair.
                        current_key = None;
                    }
                }
                Token::Number(number) => {
                    if let Some(key) = current_key {
                        value.insert(key.to_string(), Value::Number(*number));
                        // Set current_key to None to prepare for the next key-value pair.
                        current_key = None;
                    }
                }
                Token::ArrayOpen => {
                    if let Some(key) = current_key {
                        value.insert(key.to_string(), Value::Array(Self::process_array(iterator)));
                        // Set current_key to None to prepare for the next key-value pair.
                        current_key = None;
                    }
                }
                // If the token is a comma, it is the separator between multiple key-value pairs
                // in JSON. So the item being parsed from this point ahead will be a key.
                Token::Comma => is_key = true,
                Token::Boolean(boolean) => {
                    if let Some(key) = current_key {
                        value.insert(key.to_string(), Value::Boolean(*boolean));
                        // Set current_key to None to prepare for the next key-value pair.
                        current_key = None;
                    }
                }
                Token::Null => {
                    if let Some(key) = current_key {
                        value.insert(key.to_string(), Value::Null);
                        // Set current_key to None to prepare for the next key-value pair.
                        current_key = None;
                    }
                }
            }
        }

        value
    }
}
