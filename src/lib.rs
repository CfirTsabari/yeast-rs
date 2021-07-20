/*!
yeast-rs is a rust implementation for the [`yeast`](https://github.com/unshiftio/yeast) JS package.

yeast is a tiny but linear growing unique id generator.

# Usage
```rust
# use yeast_rs::yeast;
# use std::{thread::sleep, time};
# fn main() {
    let id :String = yeast().to_string();
# }
```
# Example
Using the function:
```rust
# use yeast_rs::{yeast,Yeast};
# use std::{thread::sleep, time::Duration,convert::TryInto};
# // get current time
# fn now() -> i64 {
#     chrono::Utc::now().timestamp_millis()
# }
# // wait until next milli arrive
# fn wait_new_milli() {
#     let cur = now();
#     while cur == now() {}
# }
# fn main() {
    # wait_new_milli();
    let first_id :Yeast = yeast();
    let second_id :Yeast = yeast();
    sleep(Duration::from_millis(1));
    let third_id :Yeast = yeast();

    // Based on the same timestamp but different values
    assert_ne!(first_id.to_string(), second_id.to_string());
    assert_eq!(first_id.timestamp_millis(), second_id.timestamp_millis());

    // Different Timestamps
    assert_ne!(first_id.timestamp_millis(), third_id.timestamp_millis());

    // Can be converted back
    let third_id_converted: Yeast = third_id.to_string().try_into().unwrap();
    assert_eq!(third_id_converted.timestamp_millis(), third_id.timestamp_millis());


# }

```
# Cargo features
This crate provides these cargo features:
- `async-std-runtime`: include an async version of yeast built on the async-std runtime.
- `tokio-runtime`: include an async version of yeast built on the tokio runtime.
 */

mod encode;
mod yeast;

use std::sync::Mutex;
use yeast::YeastGenerator;

pub use yeast::Yeast;

lazy_static::lazy_static! {
    static ref INSTANCE: Mutex<YeastGenerator> = Mutex::new(YeastGenerator::new());
}
/// Returns a A unique id.
///
/// # Examples
///
/// ```rust
/// use yeast_rs::yeast;
/// let unique_id = yeast();
/// ```
pub fn yeast() -> Yeast {
    INSTANCE.lock().unwrap().next().unwrap()
}

/// support for the async-std runtime
#[cfg(feature = "async-std-runtime")]
pub mod async_std {
    use crate::{Yeast, YeastGenerator};
    use async_std::sync::Mutex;
    lazy_static::lazy_static! {
        static ref INSTANCE: Mutex<YeastGenerator> = Mutex::new(YeastGenerator::new());
    }
    /// Returns a A unique id.
    ///
    /// # Examples
    ///
    /// ```
    /// use yeast_rs::async_std::yeast;
    /// # async fn doc() {
    /// let unique_id = yeast().await;
    /// # }
    /// ```
    pub async fn yeast() -> Yeast {
        INSTANCE.lock().await.next().unwrap()
    }
}

/// support for the tokio runtime
#[cfg(feature = "tokio-runtime")]
pub mod tokio {
    use crate::{Yeast, YeastGenerator};
    use tokio::sync::Mutex;
    lazy_static::lazy_static! {
        static ref INSTANCE: Mutex<YeastGenerator> = Mutex::new(YeastGenerator::new());
    }
    /// Returns a A unique id.
    ///
    /// # Examples
    ///
    /// ```
    /// use yeast_rs::tokio::yeast;
    /// # async fn doc() {
    /// let unique_id = yeast().await;
    /// # }
    /// ```
    pub async fn yeast() -> Yeast {
        INSTANCE.lock().await.next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Yeast;
    use chrono::Utc;
    use serial_test::serial;
    use std::collections::HashSet;
    use std::convert::TryInto;
    use std::future::Future;

    // get current time
    fn now() -> i64 {
        chrono::Utc::now().timestamp_millis()
    }
    // wait until next milli arrive
    fn wait_new_milli() {
        let cur = now();
        while cur == now() {}
    }

    #[test]
    #[serial]
    fn test_convert() {
        wait_new_milli();
        let bad_conversion: Result<Yeast, ()> = "...".to_string().try_into();
        let (now, id) = (Utc::now().timestamp_millis(), crate::yeast());
        let id_converted: Yeast = id.to_string().try_into().unwrap();
        assert_eq!(id_converted.timestamp_millis(), now);
        assert!(bad_conversion.is_err())
    }

    async fn test_sanity<Fut>(yeast: impl Fn() -> Fut)
    where
        Fut: Future<Output = Yeast>,
    {
        let res: String = yeast().await.to_string();
        assert!(res.len() > 5)
    }

    async fn test_multiple_yeast_in_same_milli<Fut>(yeast: impl Fn() -> Fut)
    where
        Fut: Future<Output = Yeast>,
    {
        for _ in 0..10 {
            wait_new_milli();
            let first = yeast().await.to_string();
            let second = yeast().await.to_string();
            let third = yeast().await.to_string();
            assert!(!first.contains('.'));
            assert!(second.contains(format!(".{}", crate::encode::encode(0)).as_str()));
            assert!(third.contains(format!(".{}", crate::encode::encode(1)).as_str()));
        }
    }

    async fn test_multiple_yeast<Fut>(yeast: impl Fn() -> Fut)
    where
        Fut: Future<Output = Yeast>,
    {
        let mut res = HashSet::new();
        for _ in 0..100_000 {
            assert!(res.insert(yeast().await.to_string()));
        }
    }

    async fn fake_async_yeast() -> Yeast {
        crate::yeast()
    }

    #[cfg(feature = "async-std-runtime")]
    #[async_std::test]
    #[serial]
    async fn sanity_async_std() {
        test_sanity(crate::async_std::yeast).await;
    }

    #[cfg(feature = "tokio-runtime")]
    #[async_std::test]
    #[serial]
    async fn sanity_tokio() {
        test_sanity(crate::tokio::yeast).await;
    }

    #[async_std::test]
    #[serial]
    async fn sanity() {
        test_sanity(fake_async_yeast).await;
    }

    #[cfg(feature = "async-std-runtime")]
    #[async_std::test]
    #[serial]
    async fn multiple_yeast_async_std() {
        test_multiple_yeast(crate::async_std::yeast).await;
    }

    #[cfg(feature = "tokio-runtime")]
    #[async_std::test]
    #[serial]
    async fn multiple_yeast_tokio() {
        test_multiple_yeast(crate::tokio::yeast).await;
    }

    #[async_std::test]
    #[serial]
    async fn multiple_yeast() {
        test_multiple_yeast(fake_async_yeast).await;
    }

    #[cfg(feature = "async-std-runtime")]
    #[async_std::test]
    #[serial]
    async fn multiple_yeast_in_same_milli_async_std() {
        test_multiple_yeast_in_same_milli(crate::async_std::yeast).await;
    }

    #[cfg(feature = "tokio-runtime")]
    #[async_std::test]
    #[serial]
    async fn multiple_yeast_in_same_milli_tokio() {
        test_multiple_yeast_in_same_milli(crate::tokio::yeast).await;
    }

    #[async_std::test]
    #[serial]
    async fn multiple_yeast_in_same_milli() {
        test_multiple_yeast_in_same_milli(fake_async_yeast).await;
    }
}
