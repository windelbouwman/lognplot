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

use lognplot::tracer::{AnyTracer, Tracer};
use nalgebra::{Matrix1, Matrix2, RowVector2, Vector2};
use std::sync::Arc;
use std::time::Instant;

pub struct TimeTracker {
    /// State estimate
    ///
    /// This variable estimates the time and time propulsion speed
    x_hat: Vector2<f64>,

    /// Estimation covariance
    ///
    /// This value will increase when the estimate becomes uncertain.
    covariance: Matrix2<f64>,

    // previous timestamp!
    prev: Option<Instant>,

    /// The time tracker can be traced as well (inception!)
    perf_tracer: Arc<AnyTracer>,

    trace_prefix: String,
}

impl TimeTracker {
    pub fn new(perf_tracer: Arc<AnyTracer>, trace_prefix: &str) -> Self {
        TimeTracker {
            x_hat: Vector2::zeros(),
            covariance: Matrix2::identity(),
            prev: None,
            perf_tracer,
            trace_prefix: trace_prefix.to_owned(),
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
        self.covariance = Matrix2::identity();
        self.prev = None;
    }

    // Advance the model, to predict the new value
    pub fn predict(&mut self) {
        // Only predict when we are somewhat accurate:
        if self.covariance.norm() < 10.0 {
            let dt = self.get_dt();
            // State transition model:
            let transition_model = Matrix2::new(1.0, dt, 0.0, 1.0);

            // Assume (for now) time elapses one second per second:
            // let time_speed = 1.0;  // (s/s)

            // Estimated time increases:
            self.x_hat = transition_model * self.x_hat;

            // Some noise on the prediction (Q):
            let noise_model = Matrix2::new(0.001 * dt, 0.0, 0.0, 0.001 * dt);

            // This estimate becomes more unpredictable over time
            self.covariance =
                transition_model * self.covariance * transition_model.transpose() + noise_model;

            // nalgebra matrices are rendered pretty sweet in ascii!
            // println!("Predicted: x_hat={} P={}", self.x_hat, self.P);
        }

        self.trace();
    }

    // Inject a newly observed value!
    pub fn update(&mut self, observation: f64) {
        // Update to the last prediction possible:
        self.predict();

        // Observation model (H):
        let observation_model = RowVector2::new(1.0, 0.0);

        // Innovation:
        let y: Matrix1<f64> = Matrix1::new(observation) - observation_model * self.x_hat;

        if y.norm() > 5.0 {
            self.reset(observation);
        } else {
            // Some measurement noise (R):
            let measurement_noise: Matrix1<f64> = Matrix1::new(0.01);

            // Innovation covariance (S):
            let innovation_covariance: Matrix1<f64> =
                observation_model * self.covariance * observation_model.transpose()
                    + measurement_noise;

            // Optimal Kalman gain:
            let kalman_gain: Vector2<f64> = self.covariance
                * observation_model.transpose()
                * innovation_covariance
                    .try_inverse()
                    .expect("Inverse must work");

            // Update estimate:
            self.x_hat = self.x_hat + kalman_gain * y;

            // Update variance:
            self.covariance =
                (Matrix2::identity() - kalman_gain * observation_model) * self.covariance;

            // println!(
            //     "Updated with value {}: x_hat={} P={}",
            //     observation, self.x_hat, self.P
            // );
        }

        self.trace();
    }

    fn trace(&self) {
        let t1 = Instant::now();

        self.perf_tracer.log_metric(
            &format!("META.{}.x_hat[0]", self.trace_prefix),
            t1,
            self.x_hat[0],
        );
        self.perf_tracer.log_metric(
            &format!("META.{}.x_hat[1]", self.trace_prefix),
            t1,
            self.x_hat[1],
        );
        self.perf_tracer.log_metric(
            &format!("META.{}.P[0, 0]", self.trace_prefix),
            t1,
            self.covariance[(0, 0)],
        );
        self.perf_tracer.log_metric(
            &format!("META.{}.P[0, 1]", self.trace_prefix),
            t1,
            self.covariance[(0, 1)],
        );
        self.perf_tracer.log_metric(
            &format!("META.{}.P[1, 0]", self.trace_prefix),
            t1,
            self.covariance[(1, 0)],
        );
        self.perf_tracer.log_metric(
            &format!("META.{}.P[1, 1]", self.trace_prefix),
            t1,
            self.covariance[(1, 1)],
        );
    }

    pub fn get_estimate(&self) -> f64 {
        self.x_hat[0]
    }
}
