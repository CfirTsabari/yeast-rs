/*!
yeast-rs is a rust implementation for the [`yeast`](https://github.com/unshiftio/yeast) JS package.

yeast is a tiny but linear growing unique id generator.

# Usage
```rust
# use yeast_rs::yeast;
# use std::{thread::sleep, time};
# fn main() {
    let id :String = yeast();
# }
```
# Example
Using the function:
```rust
# use yeast_rs::yeast;
# use std::{thread::sleep, time::Duration};
# fn main() {
    println!("{}", yeast());
    println!("{}", yeast());
    sleep(Duration::from_millis(1));
    println!("{}", yeast());
    println!("{}", yeast());
# }

```
# Cargo features
This crate provides these cargo features:
- `async-std-runtime`: include an async version of yeast built on the async-std runtime.
- `tokio-runtime`: include an async version of yeast built on the tokio runtime.
 */

mod utils;

use std::sync::Mutex;
use utils::Yeast;

lazy_static::lazy_static! {
    static ref INSTANCE: Mutex<Yeast> = Mutex::new(Yeast::new());
}
/// Returns a A unique id.
///
/// # Examples
///
/// ```rust
/// use yeast_rs::yeast;
/// let unique_id = yeast();
/// ```
pub fn yeast() -> String {
    INSTANCE.lock().unwrap().next().unwrap()
}

/// support for the async-std runtime
#[cfg(feature = "async-std-runtime")]
pub mod async_std {
    use crate::Yeast;
    use async_std::sync::Mutex;
    lazy_static::lazy_static! {
        static ref INSTANCE: Mutex<Yeast> = Mutex::new(Yeast::new());
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
    pub async fn yeast() -> String {
        INSTANCE.lock().await.next().unwrap()
    }
}

/// support for the tokio runtime
#[cfg(feature = "tokio-runtime")]
pub mod tokio {
    use crate::Yeast;
    use tokio::sync::Mutex;
    lazy_static::lazy_static! {
        static ref INSTANCE: Mutex<Yeast> = Mutex::new(Yeast::new());
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
    pub async fn yeast() -> String {
        INSTANCE.lock().await.next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::collections::HashSet;
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

    async fn test_sanity<Fut>(yeast: impl Fn() -> Fut)
    where
        Fut: Future<Output = String>,
    {
        let res: String = yeast().await;
        assert!(res.len() > 8)
    }

    async fn test_multiple_yeast_in_same_milli<Fut>(yeast: impl Fn() -> Fut)
    where
        Fut: Future<Output = String>,
    {
        for _ in 0..10 {
            wait_new_milli();
            let first = yeast().await;
            let second = yeast().await;
            let third = yeast().await;
            assert!(!first.contains('_'));
            assert!(second.contains(format!("_{}", crate::utils::encode(0)).as_str()));
            assert!(third.contains(format!("_{}", crate::utils::encode(1)).as_str()));
        }
    }

    async fn test_multiple_yeast<Fut>(yeast: impl Fn() -> Fut)
    where
        Fut: Future<Output = String>,
    {
        let mut res = HashSet::new();
        for _ in 0..100_000 {
            assert!(res.insert(yeast().await));
        }
    }

    async fn fake_async_yeast() -> String {
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
