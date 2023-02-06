# API bindings, snippets, known issues

*At the time of writing, rust can be used only for real-time processes (RTP), not for kernel modules (DKM)*

---

Feel free to explore any of the provided code snippets. Feel free to add your own using a pull request.

* [priority.rs](priority.rs) - illustrates functions for changing thread priority and cpu affinity

* [lock.rs](lock.rs) - illustrates various implementations of mutex lock - One provided by standard library and few other using `semLib.h` directly. It is worth noting that the mutex provided in standard library does not implement priority inheritance mechanism (it can still be achieved using methods directly from `semLib.h`)

* System time - There is a mechanism to get a current system time, implemented both in rust standard library or optional using `clockLib.h` - `clock_gettime( )`. Neither one of these mechanisms is not able to return current time with precision better than whole seconds. The field `tv_nsec` in `struct timespec` is always set to `0`. This was observed with different types of clocks. On real hardware platform.


