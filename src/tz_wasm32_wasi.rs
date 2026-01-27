pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    #[cfg(any(target_env = "p1", target_env = "p2"))]
    {
        std::env::var("TZ").or_else(|_| Ok("Etc/UTC".to_owned()))
    }

    #[cfg(target_env = "p3")]
    {
        wasip3::clocks::timezone::iana_id().ok_or_else(|| crate::GetTimezoneError::OsError);
    }
}
