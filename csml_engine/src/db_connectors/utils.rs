#[cfg(feature = "mongo")]
pub fn get_expires_at_for_mongodb(ttl: Option<chrono::Duration>) -> Option<bson::DateTime> {
    match ttl {
        Some(ttl) => {
            let expires_at = chrono::Utc::now() + ttl;

            Some(bson::DateTime::from_chrono(expires_at))
        }
        None => None,
    }
}

#[cfg(feature = "dynamo")]
pub fn get_expires_at_for_dynamodb(ttl: Option<chrono::Duration>) -> Option<i64> {
    match ttl {
        Some(ttl) => {
            let expires_at = chrono::Utc::now() + ttl;

            Some(expires_at.timestamp())
        }
        None => None,
    }
}

#[cfg(feature = "postgresql")]
pub fn get_expires_at_for_postgresql(
    ttl: Option<chrono::Duration>,
) -> Option<chrono::NaiveDateTime> {
    match ttl {
        Some(ttl) => {
            let expires_at = chrono::Utc::now().naive_utc() + ttl;

            Some(expires_at)
        }
        None => None,
    }
}

#[cfg(feature = "sqlite")]
pub fn get_expires_at_for_sqlite(ttl: Option<chrono::Duration>) -> Option<chrono::NaiveDateTime> {
    match ttl {
        Some(ttl) => {
            let expires_at = chrono::Utc::now().naive_utc() + ttl;

            Some(expires_at)
        }
        None => None,
    }
}
