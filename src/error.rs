#[derive(Debug, Clone)]
pub enum Error {
    RankNotFound,
    NameNotFound
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::RankNotFound => "指定した難易度は存在しません",
            Error::NameNotFound => "指定した名前のポケモンは出現しません"
        };
        write!(f, "{s}")
    }
}

impl std::error::Error for Error {}