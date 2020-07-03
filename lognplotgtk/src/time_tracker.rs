//! Time tracking
//!
//! Idea: track time of a remote system based
//! upon a Kalman filter.
//!
//! The filter will have an update and predict function
//! so that the time of the system under test can be
//! estimated based on observations and also based upon
//! a prediction by a model.
//!
//! Some extra notes:
//! - We should stop using the time estimate when no
//!   new values arrive, and P becomes too large.
//!

use nalgebra::{Matrix1, Matrix2, RowVector2, Vector2};
use std::time::Instant;

pub struct TimeTracker {
    /// State estimate
    ///
    /// This variable estimates the time and time propulsion speed
    x_hat: Vector2<f64>,

    /// Estimation covariance
    ///
    /// This value will increase when the estimate becomes uncertain.
    P: Matrix2<f64>,

    // previous timestamp!
    prev: Option<Instant>,
}

impl TimeTracker {
    pub fn new() -> Self {
        TimeTracker {
            x_hat: Vector2::zeros(),
            P: Matrix2::identity(),
            prev: None,
        }
    }

    fn get_dt(&mut self) -> f64 {
        // Determine dt since last prediction:
        let now = Instant::now();

        let prev = self.prev.replace(now.clone());
        if let Some(prev) = prev {
            let delta = now - prev;
            delta.as_secs_f64()
        } else {
            // First prediction
            0.0
        }
    }

    fn reset(&mut self, observation: f64) {
        self.x_hat = Vector2::new(observation, 1.0);
        self.P = Matrix2::identity();
        self.prev = None;
    }

    // Advance the model, to predict the new value
    pub fn predict(&mut self) {
        // Only predict when we are somewhat accurate:
        if self.P.norm() < 10.0 {
            let dt = self.get_dt();
            // State transition model:
            let F = Matrix2::new(1.0, dt, 0.0, 1.0);

            // Assume (for now) time elapses one second per second:
            // let time_speed = 1.0;  // (s/s)

            // Estimated time increases:
            self.x_hat = F * self.x_hat;

            // Some noise on the prediction:
            let Q = Matrix2::new(0.001 * dt, 0.0, 0.0, 0.001 * dt);

            // This estimate becomes more unpredictable over time
            self.P = F * self.P * F.transpose() + Q;

            println!("Predicted: x_hat={} P={}", self.x_hat, self.P);
        }
    }

    // Inject a newly observed value!
    pub fn update(&mut self, observation: f64) {
        // Update to the last prediction possible:
        self.predict();

        // Observation model:
        let H = RowVector2::new(1.0, 0.0);

        // Innovation:
        let y: Matrix1<f64> = Matrix1::new(observation) - H * self.x_hat;

        if y.norm() > 5.0 {
            self.reset(observation);
        } else {
            // Some measurement noise:
            let R: Matrix1<f64> = Matrix1::new(0.01);

            // Innovation covariance:
            let S: Matrix1<f64> = H * self.P * H.transpose() + R;

            // Optimal Kalman gain:
            let K: Vector2<f64> =
                self.P * H.transpose() * S.try_inverse().expect("Inverse must work");

            // Update estimate:
            self.x_hat = self.x_hat + K * y;

            // Update variance:
            self.P = (Matrix2::identity() - K * H) * self.P;

            println!(
                "Updated with value {}: x_hat={} P={}",
                observation, self.x_hat, self.P
            );
        }
    }

    pub fn get_estimate(&self) -> f64 {
        self.x_hat[0]
    }
}
