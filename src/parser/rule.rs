use crate::model::Vnum;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Rule {
    Isolate(Vnum),
    Separate(Vnum, Vnum),
}

impl TryFrom<&str> for Rule {
    type Error = ParseRuleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((a, b)) = value.split_once('=') {
            match a {
                "isolate" => u32::from_str(b)
                    .map_err(|_| ParseRuleError::InvalidVnum)
                    .map(|vnum| Rule::Isolate(vnum)),
                "separate" => match b.split_once(',') {
                    Some((v1, v2)) => {
                        let v1 = u32::from_str(v1).map_err(|_| ParseRuleError::InvalidVnum)?;
                        let v2 = u32::from_str(v2).map_err(|_| ParseRuleError::InvalidVnum)?;
                        Ok(Rule::Separate(v1, v2))
                    }
                    None => Err(ParseRuleError::InvalidVnum),
                },
                _ => Err(ParseRuleError::UnknownRule),
            }
        } else {
            Err(ParseRuleError::InvalidVnum)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ParseRuleError {
    UnknownRule,
    InvalidVnum,
}

#[cfg(test)]
mod test {
    use super::{ParseRuleError, Rule};

    #[test]
    fn valid_isolate_rule() {
        assert_eq!(Ok(Rule::Isolate(1234)), Rule::try_from("isolate=1234"));
    }

    #[test]
    fn invalid_isolate_rules() {
        assert_eq!(
            Err(ParseRuleError::InvalidVnum),
            Rule::try_from("isolate=abcd")
        );
        assert_eq!(
            Err(ParseRuleError::InvalidVnum),
            Rule::try_from("isolate=1234,5678")
        );
        assert_eq!(Err(ParseRuleError::InvalidVnum), Rule::try_from("isolate="));
        assert_eq!(Err(ParseRuleError::InvalidVnum), Rule::try_from("isolate"));
    }

    #[test]
    fn valid_separate_rule() {
        assert_eq!(
            Ok(Rule::Separate(1234, 5678)),
            Rule::try_from("separate=1234,5678")
        );
    }

    #[test]
    fn invalid_separate_rule() {
        assert_eq!(
            Err(ParseRuleError::InvalidVnum),
            Rule::try_from("separate=abcd,5678")
        );
        assert_eq!(
            Err(ParseRuleError::InvalidVnum),
            Rule::try_from("separate=1234,efgh")
        );
        assert_eq!(
            Err(ParseRuleError::InvalidVnum),
            Rule::try_from("separate=1234")
        );
        assert_eq!(
            Err(ParseRuleError::InvalidVnum),
            Rule::try_from("separate=")
        );
        assert_eq!(Err(ParseRuleError::InvalidVnum), Rule::try_from("separate"));
    }

    #[test]
    fn unknown_rule() {
        assert_eq!(
            Err(ParseRuleError::UnknownRule),
            Rule::try_from("go-away=1234")
        );
    }
}
