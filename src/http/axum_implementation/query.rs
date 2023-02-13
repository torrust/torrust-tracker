use std::collections::HashMap;
use std::panic::Location;
use std::str::FromStr;

use thiserror::Error;
pub struct Query {
    params: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum ParseQueryError {
    #[error("invalid param {raw_param} in {location}")]
    InvalidParam {
        location: &'static Location<'static>,
        raw_param: String,
    },
}

impl FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(raw_query: &str) -> Result<Self, Self::Err> {
        let mut params: HashMap<String, String> = HashMap::new();

        let raw_params = raw_query.trim().trim_start_matches('?').split('&').collect::<Vec<&str>>();

        for raw_param in raw_params {
            let param: Param = raw_param.parse()?;
            params.insert(param.name, param.value);
        }

        Ok(Self { params })
    }
}

#[derive(Debug, PartialEq)]
struct Param {
    name: String,
    value: String,
}

impl FromStr for Param {
    type Err = ParseQueryError;

    fn from_str(raw_param: &str) -> Result<Self, Self::Err> {
        let pair = raw_param.split('=').collect::<Vec<&str>>();

        if pair.len() != 2 {
            return Err(ParseQueryError::InvalidParam {
                location: Location::caller(),
                raw_param: raw_param.to_owned(),
            });
        }

        Ok(Self {
            name: pair[0].to_owned(),
            value: pair[1].to_owned(),
        })
    }
}

impl Query {
    #[must_use]
    pub fn get_param(&self, name: &str) -> Option<String> {
        self.params.get(name).map(std::string::ToString::to_string)
    }
}

#[cfg(test)]
mod tests {
    use super::Query;
    use crate::http::axum_implementation::query::Param;

    #[test]
    fn it_should_parse_the_query_params_from_an_url_query_string() {
        let raw_query = "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548";

        let query = raw_query.parse::<Query>().unwrap();

        assert_eq!(
            query.get_param("info_hash").unwrap(),
            "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"
        );
        assert_eq!(query.get_param("peer_id").unwrap(), "-qB00000000000000001");
        assert_eq!(query.get_param("port").unwrap(), "17548");
    }

    #[test]
    fn it_should_fail_parsing_an_invalid_query_string() {
        let invalid_raw_query = "name=value=value";

        let query = invalid_raw_query.parse::<Query>();

        assert!(query.is_err());
    }

    #[test]
    fn it_should_ignore_the_preceding_question_mark_if_it_exists() {
        let raw_query = "?name=value";

        let query = raw_query.parse::<Query>().unwrap();

        assert_eq!(query.get_param("name").unwrap(), "value");
    }

    #[test]
    fn it_should_trim_whitespaces() {
        let raw_query = " name=value ";

        let query = raw_query.parse::<Query>().unwrap();

        assert_eq!(query.get_param("name").unwrap(), "value");
    }

    #[test]
    fn it_should_parse_a_single_query_param() {
        let raw_param = "name=value";

        let param = raw_param.parse::<Param>().unwrap();

        assert_eq!(
            param,
            Param {
                name: "name".to_string(),
                value: "value".to_string(),
            }
        );
    }

    #[test]
    fn it_should_fail_parsing_an_invalid_query_param() {
        let invalid_raw_param = "name=value=value";

        let query = invalid_raw_param.parse::<Param>();

        assert!(query.is_err());
    }
}
