use libc::{c_int, cpuset_t, pthread_t, taskIdSelf};


extern "C" {

    fn pthread_setschedprio(thread_id: pthread_t, priority: c_int);


    pub fn taskCpuAffinitySet(
        tid: libc::TASK_ID, 	 /* task ID */
        new_affinity: cpuset_t   /* new affinity set */
    );


}

pub fn setschedprio(thread_id: pthread_t, priority: c_int) {
    unsafe {
    	/* Don't forget that this uses POSIX API for priority, which uses different values than VxWorks API */
        pthread_setschedprio(thread_id, priority)
    }
}

pub fn pincpu() {
    unsafe {
        let mut cpuset: cpuset_t = 1; // Only CPU0; cpuset_t is a bitmask, true at bit i = cpu_i enabled

        taskCpuAffinitySet(taskIdSelf(), cpuset);
    }
}
