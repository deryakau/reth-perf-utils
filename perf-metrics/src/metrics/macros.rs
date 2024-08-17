use std::time::Instant;

#[allow(unused_macros)]
macro_rules! define_record_with_elapsed_time_function {
    ($name:ident, $field:ident, $time_counter:ident) => {
        pub(super) fn $name(&mut self) -> Instant {
            // Capture the current time.
            let now = Instant::now();
            // Calculate the duration since the last recorded time and reset the counter.
            let duration = now.duration_since(self.$time_counter);
            // Update the time counter to the current time.
            self.$time_counter = now;
            // Record the duration in the specified field.
            self.$field = self.$field.checked_add(duration.as_nanos() as usize).expect("overflow");
            now
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_record_time_function {
    ($name:ident, $field:ident, $time_counter:ident) => {
        pub(super) fn $name(&mut self) {
            // Capture the current time and calculate the duration since the last recorded time.
            let now = Instant::now();
            let duration = now.duration_since(self.$time_counter);
            // Update the time counter to the current time.
            self.$time_counter = now;
            // Record the duration in the specified field.
            self.$field = self.$field.checked_add(duration.as_nanos() as usize).expect("overflow");
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_record_size_function {
    ($name:ident, $field:ident) => {
        pub(super) fn $name(&mut self, size: usize) {
            // Record the size in the specified field.
            self.$field = self.$field.checked_add(size).expect("overflow");
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_start_functions {
    ($start_fn:ident, $start_field:ident) => {
        pub(super) fn $start_fn(&mut self) {
            // Start time recording.
            self.$start_field = Instant::now();
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_write_macro {
    ($struct_name:ident, $start_record_fn:ident, $record_upsert_time_fn:ident, $record_size_fn:ident) => {
        #[cfg(feature = "enable_execution_duration_record")]
        pub struct $struct_name {
            size: usize,
            // Add fields for time recording if necessary.
            start_time: Instant,
            // Additional fields for recording elapsed time and size.
            elapsed_time: usize,
            total_size: usize,
        }

        #[cfg(feature = "enable_execution_duration_record")]
        impl $struct_name {
            pub fn new(size: usize) -> Self {
                let mut instance = Self {
                    size,
                    start_time: Instant::now(),
                    elapsed_time: 0,
                    total_size: 0,
                };
                // Call the start recording function.
                instance.$start_record_fn();
                instance
            }
        }

        #[cfg(feature = "enable_execution_duration_record")]
        impl Drop for $struct_name {
            fn drop(&mut self) {
                // Call functions to record elapsed time and size.
                self.$record_upsert_time_fn();
                self.$record_size_fn(self.size);
            }
        }
    };
}
