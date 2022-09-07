use std::os::raw::*;

const MIN_BORDER_WIDTH: c_int = 0;
const MAX_BORDER_WIDTH: c_int = 10000;
const MIN_DEFAULT_CLIENTS_IN_MASTER: c_int = 1;
const MAX_DEFAULT_CLIENTS_IN_MASTER: c_int = 10000;
const MIN_GAP_SIZE: c_int = 0;
const MAX_GAP_SIZE: c_int = 10000;

pub fn border_width(border_width: c_int) -> c_int {
	if border_width < MIN_BORDER_WIDTH { return MIN_BORDER_WIDTH }
	if border_width > MAX_BORDER_WIDTH { return MAX_BORDER_WIDTH }
	border_width
}

pub fn default_clients_in_master(default_clients_in_master: c_int) -> c_int {
	if default_clients_in_master < MIN_DEFAULT_CLIENTS_IN_MASTER {
		return MIN_DEFAULT_CLIENTS_IN_MASTER
	}
	if default_clients_in_master > MAX_DEFAULT_CLIENTS_IN_MASTER {
		return MAX_DEFAULT_CLIENTS_IN_MASTER
	}
	default_clients_in_master
}

pub fn gap_size(gap_size: c_int) -> c_int {
	if gap_size < MIN_GAP_SIZE { return MIN_GAP_SIZE }
	if gap_size > MAX_GAP_SIZE { return MAX_GAP_SIZE }
	gap_size
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_border_width() {
		assert_eq!(border_width(-1), 0);

		assert_eq!(border_width(0), 0);
		assert_eq!(border_width(10), 10);
		assert_eq!(border_width(10_000), 10_000);

		assert_eq!(border_width(10_001), 10_000);
		assert_eq!(border_width(20_000), 10_000);
	}

	#[test]
	fn test_default_clients_in_master() {
		assert_eq!(default_clients_in_master(-1), 1);
		assert_eq!(default_clients_in_master(0), 1);

		assert_eq!(default_clients_in_master(1), 1);
		assert_eq!(default_clients_in_master(2), 2);
		assert_eq!(default_clients_in_master(9999), 9999);
		assert_eq!(default_clients_in_master(10_000), 10_000);

		assert_eq!(default_clients_in_master(10_001), 10_000);
		assert_eq!(default_clients_in_master(20_000), 10_000);
	}

	#[test]
	fn test_gap_size() {
		assert_eq!(gap_size(-2), 0);
		assert_eq!(gap_size(-1), 0);

		assert_eq!(gap_size(0), 0);
		assert_eq!(gap_size(1), 1);
		assert_eq!(gap_size(100), 100);
		assert_eq!(gap_size(9999), 9999);
		assert_eq!(gap_size(10_000), 10_000);

		assert_eq!(gap_size(10_001), 10_000);
		assert_eq!(gap_size(20_000), 10_000);
	}
}
