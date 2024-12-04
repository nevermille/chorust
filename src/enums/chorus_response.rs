use crate::response::chorus::ChorusError;
use serde::de::DeserializeOwned;

pub enum ChorusResponse<T> {
    Success(T),
    Error(ChorusError),
    Unknown(String),
}

impl<T> ChorusResponse<T> {
    pub fn is_success(&self) -> bool {
        matches!(self, ChorusResponse::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, ChorusResponse::Error(_))
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, ChorusResponse::Unknown(_))
    }

    pub fn type_str(&self) -> String {
        match self {
            ChorusResponse::Success(_) => "Success".to_string(),
            ChorusResponse::Error(_) => "Error".to_string(),
            ChorusResponse::Unknown(_) => "Unknown".to_string(),
        }
    }

    pub fn unwrap(self) -> T {
        if let ChorusResponse::Success(v) = self {
            return v;
        }

        panic!(
            "called `ChorusResponse::unwrap()` on a `{}` value",
            self.type_str()
        );
    }

    pub fn unwrap_err(self) -> ChorusError {
        if let ChorusResponse::Error(v) = self {
            return v;
        }

        panic!(
            "called `ChorusResponse::unwrap_err()` on a `{}` value",
            self.type_str()
        );
    }

    pub fn unwrap_unknown(self) -> String {
        if let ChorusResponse::Unknown(v) = self {
            return v;
        }

        panic!(
            "called `ChorusResponse::unwrap_unknown()` on a `{}` value",
            self.type_str()
        );
    }
}

impl<T> ChorusResponse<T>
where
    T: DeserializeOwned,
{
    pub fn from_json(json: &str) -> Self {
        let try_success = serde_json::from_str(json);
        let try_error = serde_json::from_str(json);

        match (try_success, try_error) {
            (Ok(v), _) => ChorusResponse::Success(v),
            (Err(_), Ok(v)) => ChorusResponse::Error(v),
            (_, _) => ChorusResponse::Unknown(json.to_string()),
        }
    }
}
