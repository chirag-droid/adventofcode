use std::collections::HashMap;
use std::sync::Once;
use anyhow::Result;

// Global solution registry type
type SolutionFn = fn(u32) -> Result<()>;
type SolutionRegistry = HashMap<u32, SolutionFn>;

// Singleton registry management
pub struct SolutionManager {
    registry: SolutionRegistry,
}

impl SolutionManager {
    fn global() -> &'static mut Self {
        static mut INSTANCE: Option<SolutionManager> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                INSTANCE = Some(SolutionManager {
                    registry: HashMap::new()
                });
            });

            INSTANCE.as_mut().unwrap()
        }
    }

    // Register a solution for a specific day
    pub fn register(day: u32, solution: SolutionFn) {
        let manager = Self::global();
        manager.registry.insert(day, solution);
    }

    // Retrieve a solution for a given day
    pub fn get(day: u32) -> Option<SolutionFn> {
        let manager = Self::global();
        manager.registry.get(&day).copied()
    }

    // List all registered days
    pub fn list_days() -> Vec<u32> {
        let manager = Self::global();
        manager.registry.keys().cloned().collect()
    }
}

// Macro to simplify solution registration
#[macro_export]
macro_rules! register_solution {
    ($day:expr, $solution_fn:path) => {
        // Static initialization block
        #[used]
        #[cfg_attr(target_os = "linux", link_section = ".init_array")]
        #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
        static __SOLUTION_REGISTER: fn() = || {
            $crate::SolutionManager::register($day, $solution_fn);
        };
    };
}
