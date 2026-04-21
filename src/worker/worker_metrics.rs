use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossbeam_channel::Receiver;

pub struct HashCompletionReport {
    pub at: Instant,
    pub count: u64,
}

pub struct MetricsAggregator {
    samples: VecDeque<HashCompletionReport>,
    first_sample_at: Option<Instant>,

    /// Lifetime hash counter. Clone the Arc before spawning the metrics
    /// thread so `main` can read it for the bandit reward calculation.
    pub total_hashes: Arc<AtomicU64>,

    window_15s: Duration,
    window_60s: Duration,
    window_15m: Duration,
}

impl MetricsAggregator {
    /// Returns `(aggregator, total_hashes_handle)`.
    /// Keep `total_hashes_handle` in `main`; move the aggregator into the thread.
    pub fn new() -> (Self, Arc<AtomicU64>) {
        let total_hashes = Arc::new(AtomicU64::new(0));
        let agg = MetricsAggregator {
            samples: VecDeque::new(),
            first_sample_at: None,
            total_hashes: total_hashes.clone(),
            window_15s: Duration::from_secs(15),
            window_60s: Duration::from_secs(60),
            window_15m: Duration::from_secs(15 * 60),
        };
        (agg, total_hashes)
    }

    /// Drain all pending hash counts from the channel (never blocks),
    /// then return (15 s, 60 s, 15 min) rates in H/s.
    /// `None` means the window hasn't filled yet — callers stay silent.
    pub fn push_hash_count(&mut self, hash_report: HashCompletionReport) {
        if self.first_sample_at.is_none() {
            self.first_sample_at = Some(hash_report.at);
        }
        self.total_hashes
            .fetch_add(hash_report.count, Ordering::Relaxed);
        self.samples.push_back(hash_report);
    }

    pub fn get_rates(&mut self) -> (Option<f64>, Option<f64>, Option<f64>) {
        let now = Instant::now();

        // Drop samples older than the largest window.
        let horizon = now - self.window_15m;
        while self.samples.front().map_or(false, |s| s.at < horizon) {
            self.samples.pop_front();
        }

        let elapsed = match self.first_sample_at {
            Some(t) => now.duration_since(t),
            None => return (None, None, None),
        };

        let rate_for = |window: Duration| -> Option<f64> {
            if elapsed < window {
                return None;
            }
            let cutoff = now - window;
            let hashes: u64 = self
                .samples
                .iter()
                .filter(|s| s.at >= cutoff)
                .map(|s| s.count)
                .sum();
            Some(hashes as f64 / window.as_secs_f64())
        };

        (
            rate_for(self.window_15s),
            rate_for(self.window_60s),
            rate_for(self.window_15m),
        )
    }
}

/// Format a hashrate value.  Returns aligned whitespace for `None` so the
/// log columns stay stable while windows are still filling.
pub fn fmt_rate(rate: Option<f64>) -> String {
    match rate {
        None => "     --     ".to_string(), // Blank if window not yet full.
        Some(h) if h >= 1_000_000.0 => format!("{:>7.2} MH/s", h / 1_000_000.0),
        Some(h) if h >= 1_000.0 => format!("{:>7.2} KH/s", h / 1_000.0),
        Some(h) => format!("{:>7.2}  H/s", h),
    }
}

/// Monitor and report hashrate metrics.
///
/// Run this on a dedicated thread.
pub fn run_metrics_loop(
    mut agg: MetricsAggregator,
    rx: &Receiver<HashCompletionReport>,
    stop: &AtomicBool,
) {
    let poll_interval = Duration::from_secs_f64(0.5);

    let report_interval = Duration::from_secs_f64(5.0);
    let mut last_report_time = Instant::now();

    loop {
        std::thread::sleep(poll_interval);

        if stop.load(Ordering::Relaxed) {
            // One final drain so total_hashes is accurate for the bandit.
            while let Ok(count) = rx.try_recv() {
                agg.push_hash_count(count);
            }

            return;
        }

        while let Ok(count) = rx.try_recv() {
            agg.push_hash_count(count);
        }

        if last_report_time.elapsed() >= report_interval {
            debug!(
                "total_hashes: {}, total_samples: {}, total_elapsed: {:?}",
                agg.total_hashes.load(Ordering::Relaxed),
                agg.samples.len(),
                agg.first_sample_at.and_then(|t| Some(t.elapsed()))
            );

            let (r15s, r60s, r15m) = agg.get_rates();

            if r15s.is_some() || r60s.is_some() || r15m.is_some() {
                println!(
                    "Hashrate - 15s: {} | 60s: {} | 15m: {}",
                    fmt_rate(r15s),
                    fmt_rate(r60s),
                    fmt_rate(r15m),
                );
            }

            last_report_time = Instant::now();
        }
    }
}
