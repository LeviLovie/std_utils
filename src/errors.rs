// std::result::Error<T, String> -> anyhow::Result<T, anyhow::Error>
pub trait StringAnyhow<T> {
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error>;
    fn anymsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error>;
}

impl<T> StringAnyhow<T> for std::result::Result<T, String> {
    #[inline]
    #[track_caller]
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error> {
        self.map_err(|e| anyhow::Error::msg(e))
    }

    #[inline]
    #[track_caller]
    fn anymsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error> {
        self.map_err(|e| anyhow::Error::msg(format!("{}: {}", msg, e)))
    }
}

// std::result::Error<T, String> -> anyhow::Result<T, anyhow::Error>
pub trait ErrorAnyhow<T> {
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error>;
    fn anymsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error>;
}

impl<T> ErrorAnyhow<T> for std::result::Result<T, anyhow::Error> {
    #[inline]
    #[track_caller]
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error> {
        self.map_err(|e| e.into())
    }

    #[inline]
    #[track_caller]
    fn anymsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error> {
        self.map_err(|e| anyhow::Error::msg(format!("{}: {}", msg, e)))
    }
}

// Option<T> -> anyhow::Result<T, anyhow::Error>
pub trait OptionAnyhow<T> {
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error>;
    fn anymsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error>;
}

impl<T> OptionAnyhow<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error> {
        self.ok_or_else(|| anyhow::Error::msg("Option is None"))
    }

    #[inline]
    #[track_caller]
    fn anymsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error> {
        self.ok_or_else(|| anyhow::Error::msg(msg.to_string()))
    }
}

// Log, LogMsg, Context, Dot for anyhow::Result<T, anyhow::Error>
pub trait Debugging<T> {
    fn log(self) -> anyhow::Result<T, anyhow::Error>;
    fn logmsg(self, msg: &str) -> Result<T, anyhow::Error>;
    fn context<C>(self, context: C) -> anyhow::Result<T>
    where
        C: std::fmt::Debug + Send + Sync + 'static;
    fn dot(self) -> anyhow::Result<T, anyhow::Error>;
}

impl<T> Debugging<T> for Result<T, anyhow::Error> {
    #[inline]
    #[track_caller]
    fn log(self) -> anyhow::Result<T, anyhow::Error> {
        if let Err(e) = &self {
            tracing::error!("{:?}", e);
        }
        self
    }

    #[inline]
    #[track_caller]
    fn logmsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error> {
        if let Err(e) = &self {
            tracing::error!("{}: {:?}", msg, e);
        }
        self
    }

    #[inline]
    #[track_caller]
    fn context<C>(self, context: C) -> anyhow::Result<T>
    where
        C: std::fmt::Debug + Send + Sync + 'static,
    {
        let caller = std::panic::Location::caller();
        anyhow::Context::context(
            self,
            format!(
                "{:?} at `{}@{}:{}`",
                context,
                caller.file(),
                caller.line(),
                caller.column()
            ),
        )
    }

    #[inline]
    #[track_caller]
    fn dot(self) -> anyhow::Result<T> {
        let caller = std::panic::Location::caller();
        anyhow::Context::context(
            self,
            format!(
                "at `{:?}@{}:{}`",
                caller.file(),
                caller.line(),
                caller.column()
            ),
        )
    }
}

impl<T> Debugging<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn log(self) -> anyhow::Result<T, anyhow::Error> {
        if self.is_none() {
            tracing::error!("Option is None");
        }
        self.ok_or_else(|| anyhow::Error::msg("Option is None"))
    }

    #[inline]
    #[track_caller]
    fn logmsg(self, msg: &str) -> anyhow::Result<T, anyhow::Error> {
        if self.is_none() {
            tracing::error!("{}", msg);
        }
        self.ok_or_else(|| anyhow::Error::msg(msg.to_string()))
    }

    #[inline]
    #[track_caller]
    fn context<C>(self, context: C) -> anyhow::Result<T>
    where
        C: std::fmt::Debug + Send + Sync + 'static,
    {
        let caller = std::panic::Location::caller();
        anyhow::Context::context(
            self.ok_or_else(|| anyhow::Error::msg("Option is None")),
            format!(
                "{:?} at `{}@{}:{}`",
                context,
                caller.file(),
                caller.line(),
                caller.column()
            ),
        )
    }

    #[inline]
    #[track_caller]
    fn dot(self) -> anyhow::Result<T> {
        let caller = std::panic::Location::caller();
        anyhow::Context::context(
            self.ok_or_else(|| anyhow::Error::msg("Option is None")),
            format!(
                "at `{:?}@{}:{}`",
                caller.file(),
                caller.line(),
                caller.column()
            ),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_anyhow() {
        let result: Result<i32, String> = Ok(1);
        let result = result.anyhow();
        assert_eq!(result.unwrap(), 1);
        let result: Result<i32, String> = Err("error".to_string());
        let result = result.anymsg("test");
        assert_eq!(result.unwrap_err().to_string(), "test: error");
    }

    #[test]
    fn error_anyhow() {
        let result: Result<i32, anyhow::Error> = Ok(1);
        let result = result.anyhow();
        assert_eq!(result.unwrap(), 1);
        let result: Result<i32, anyhow::Error> = Err(anyhow::Error::msg("error"));
        let result = result.anymsg("test");
        assert_eq!(result.unwrap_err().to_string(), "test: error");
    }

    #[test]
    fn option_anyhow() {
        let result: Option<i32> = Some(1);
        let result = result.anyhow();
        assert_eq!(result.unwrap(), 1);
        let result: Option<i32> = None;
        let result = result.anymsg("test");
        assert_eq!(result.unwrap_err().to_string(), "test");
    }

    #[test]
    fn debugging() {
        let result: Result<i32, anyhow::Error> = Ok(1);
        let result = result.log();
        assert_eq!(result.unwrap(), 1);
        let result: Result<i32, anyhow::Error> = Err(anyhow::Error::msg("error"));
        let result = result.logmsg("test");
        assert_eq!(result.unwrap_err().to_string(), "error");
        let result: Option<i32> = Some(1);
        let result = result.log();
        assert_eq!(result.unwrap(), 1);
        let result: Option<i32> = None;
        let result = result.logmsg("test");
        assert_eq!(result.unwrap_err().to_string(), "test");
    }
}
