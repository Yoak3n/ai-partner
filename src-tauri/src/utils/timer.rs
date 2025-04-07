use std::collections::HashMap;
use anyhow::Result;
use std::sync::{
    Arc,
    Mutex,RwLock
};
use once_cell::sync::OnceCell;
use delay_timer::prelude::{DelayTimer, DelayTimerBuilder};
type TaskID = u64;

#[derive(Debug, Clone)]
pub struct TimerTask {
    pub task_id: TaskID,
    pub interval_minutes: u64,
    #[allow(unused)]
    pub last_run: i64, // Timestamp of last execution
}


pub struct Timer {
    /// cron manager
    pub delay_timer: Arc<RwLock<DelayTimer>>,

    /// save the current state - using RwLock for better read concurrency
    pub timer_map: Arc<RwLock<HashMap<String, TimerTask>>>,

    /// increment id - kept as mutex since it's just a counter
    pub timer_count: Arc<Mutex<TaskID>>,

    /// Flag to mark if timer is initialized - atomic for better performance
    pub initialized: Arc<std::sync::atomic::AtomicBool>,
}

impl Timer {
    pub fn global() -> &'static Timer {
        static TIMER: OnceCell<Timer> = OnceCell::new();

        TIMER.get_or_init(|| Timer {
            delay_timer: Arc::new(RwLock::new(DelayTimerBuilder::default().build())),
            timer_map: Arc::new(RwLock::new(HashMap::new())),
            timer_count: Arc::new(Mutex::new(1)),
            initialized: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    pub fn init(&self) -> Result<()> {
        // Use compare_exchange for thread-safe initialization check
        if self
            .initialized
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            )
            .is_err()
        {
            return Ok(());
        }


        // Initialize timer tasks
        if let Err(e) = self.refresh() {
            // Reset initialization flag on error
            self.initialized
                .store(false, std::sync::atomic::Ordering::SeqCst);
            return Err(e);
        }

        // let cur_timestamp = chrono::Local::now().timestamp();

        // Advance tasks outside of locks to minimize lock contention
 
        println!("init timer {}", chrono::Local::now().to_rfc3339());
        Ok(())
    }
    pub fn refresh(&self) -> Result<()> {
        // let timer_map = self.timer_map.write();
        // let delay_timer = self.delay_timer.write();

        Ok(())
    }
    // fn gen_map(&self) -> HashMap<String, u64> {
    //     let new_map = HashMap::new();



    //     new_map
    // }
    // fn gen_diff(&self) -> HashMap<String, DiffFlag> {
    //     let mut diff_map = HashMap::new();
    //     let new_map = self.gen_map();

    //     // Read lock for comparing current state
    //     let timer_map = self.timer_map.read().unwrap();

    //     // Find tasks to modify or delete
    //     for (uid, task) in timer_map.iter() {
    //         match new_map.get(uid) {
    //             Some(&interval) if interval != task.interval_minutes => {
    //                 // Task exists but interval changed
    //                 diff_map.insert(uid.clone(), DiffFlag::Mod(task.task_id, interval));
    //             }
    //             None => {
    //                 // Task no longer needed
    //                 diff_map.insert(uid.clone(), DiffFlag::Del(task.task_id));
    //             }
    //             _ => {
    //                 // Task exists with same interval, no change needed
    //             }
    //         }
    //     }

    //     // Find new tasks to add
    //     let mut next_id = *self.timer_count.lock().unwrap();

    //     for (uid, &interval) in new_map.iter() {
    //         if !timer_map.contains_key(uid) {
    //             diff_map.insert(uid.clone(), DiffFlag::Add(next_id, interval));
    //             next_id += 1;
    //         }
    //     }

    //     // Update counter only if we added new tasks
    //     if next_id > *self.timer_count.lock().unwrap() {
    //         *self.timer_count.lock().unwrap() = next_id;
    //     }

    //     diff_map
    // }
}

// #[derive(Debug)]
// enum DiffFlag {
//     Del(TaskID),
//     Add(TaskID, u64),
//     Mod(TaskID, u64),
// }