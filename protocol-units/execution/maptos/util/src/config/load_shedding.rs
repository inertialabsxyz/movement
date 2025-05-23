//! Configuration for load-shedding limits.

use super::common::{default_batch_production_time, default_max_transactions_in_flight};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
	/// The maximum number of transactions permitted to be in flight
	/// before new transactions are rejected.
	#[serde(default = "default_max_transactions_in_flight")]
	pub max_transactions_in_flight: Option<u64>,
	/// Time between 2 batch production.
	#[serde(default = "default_batch_production_time")]
	pub batch_production_time: u64,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			max_transactions_in_flight: default_max_transactions_in_flight(),
			batch_production_time: default_batch_production_time(),
		}
	}
}
