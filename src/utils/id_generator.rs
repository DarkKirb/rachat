//! Snowflake-like ID generator

use std::{
    cell::{Cell, LazyCell},
    sync::LazyLock,
};

use chrono::Utc;
use rand::{Rng, rng};

/// Generates a Snowflake ID
///
/// These IDs are roughly time-ordered across nodes, assuming that their clocks are reasonably in sync.
///
/// On a single system, the IDs are monotonically iff the system clock is monotonically increasing.
///
/// Due to the high accuracy as well as counters, duplicate IDs are avoided in the case that the clock is rewound.
///
/// # Panics
/// This function will panic if the current system time in UTC is before the start of the unix epoch [aka misconfigured].
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "Only possible with serious misconfiguration of the system"
)]
pub fn generate() -> u128 {
    /// Application-global Node ID
    static NODE_ID: LazyLock<u16> = LazyLock::new(|| rng().random());
    thread_local! {
        /// Random Thread ID
        static THREAD_ID: LazyCell<u16> = LazyCell::new(|| rng().random());
        /// Per-thread Counter
        static THREAD_COUNTER: Cell<u16> = const { Cell::new(0) };
    };

    let now = Utc::now();

    let counter = THREAD_COUNTER.get();
    THREAD_COUNTER.set(counter.wrapping_add(1));

    let mut id: u128 = now.timestamp().try_into().expect("Configured Clock");
    id *= 1_000_000_000; // Adjust for nanoseconds
    id += u128::from(now.timestamp_subsec_nanos());
    id <<= 16;
    id |= u128::from(*NODE_ID);
    id <<= 16;
    id |= u128::from(THREAD_ID.with(|v| **v));
    id <<= 16;
    id |= u128::from(counter);

    id
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use dashmap::DashSet;

    #[test]
    fn ensure_order() {
        let id1 = super::generate();
        let id2 = super::generate();

        assert!(id1 < id2);
    }

    #[test]
    fn ensure_unique() {
        let mut ids = HashSet::with_capacity(10_000);

        for _ in 0..ids.capacity() {
            ids.insert(super::generate());
        }

        assert_eq!(ids.len(), ids.capacity());
    }

    #[test]
    fn ensure_unique_mt() {
        const THREADS: usize = 16;
        const ITERS: usize = 10_000;
        let ids = Arc::new(DashSet::with_capacity(THREADS * ITERS));
        let mut handles = Vec::with_capacity(THREADS);

        for _ in 0..16 {
            let ids_clone = Arc::clone(&ids);
            handles.push(std::thread::spawn(move || {
                for _ in 0..ITERS {
                    ids_clone.insert(super::generate());
                }
            }));
        }

        for handle in handles {
            #[allow(clippy::unwrap_used)]
            handle.join().unwrap();
        }

        assert_eq!(ids.len(), THREADS * ITERS);
    }
}
