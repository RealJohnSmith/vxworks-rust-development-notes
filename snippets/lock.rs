use libc::{_Vx_SEM_ID, c_int, SEM_ID_KERNEL};
use std_semaphore::Semaphore;



#[derive(strum_macros::Display, PartialEq)]
enum Mode {
    StandardSemaphore, // One implemented by standard library
    VxWithoutPriorityInheritanceFifo,
    VxWithoutPriorityInheritanceTaskPriority,
    VxPriorityInheritance
}


pub(crate) struct Lock {
    mode: Mode,

    std_sem: Semaphore,
    vx_sem: usize
}

impl Lock {
    pub fn new(mode: Mode) -> Lock {
        unsafe {
            let options = match mode {
                Mode::VxPriorityInheritance => {
                    SEM_INVERSION_SAFE + SEM_Q_PRIORITY
                }
                Mode::VxWithoutPriorityInheritanceFifo => {
                    SEM_Q_FIFO
                }
                Mode::VxWithoutPriorityInheritanceTaskPriority => {
                    SEM_Q_PRIORITY
                }
                _ => unsafe { -1 }
            };
            let vx_sem = semMCreate(options);
            if vx_sem.is_null() {
                println!("vx_sem is null");
            }

            Lock {
                mode,

                std_sem: Semaphore::new(1),
                vx_sem: vx_sem as usize
            }
        }
    }

    pub fn acquire(&self) {
        let vx_sem = self.vx_sem as _Vx_SEM_ID;
        match &self.mode {
            Mode::StandardSemaphore => {
                &self.std_sem.acquire();
            }
            _ => unsafe {
                semTake(vx_sem, WAIT_FOREVER);
            }
        }
    }

    pub fn release(&self) {
        let vx_sem = self.vx_sem as _Vx_SEM_ID;
        match &self.mode {
            Mode::StandardSemaphore => {
                &self.std_sem.release();
            }
            _ => unsafe {
                semGive(vx_sem);
            }
        }
    }
}

extern "C" {
    fn semMCreate(options: c_int) -> _Vx_SEM_ID;

    fn semTake(sem_id: _Vx_SEM_ID, timeout: c_int);
    fn semGive(sem_id: _Vx_SEM_ID);

}
const SEM_Q_PRIORITY: c_int = 0x1;
const SEM_Q_FIFO: c_int = 0x0;
const SEM_DELETE_SAFE: c_int = 0x4;
const SEM_INVERSION_SAFE: c_int = 0x8;

const WAIT_FOREVER: c_int = -1;
const NO_WAIT: c_int = 0;
