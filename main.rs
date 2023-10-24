// main.rs
extern crate libloading;

use libloading::{Library, Symbol};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VehiclePose {
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
    pub id: u64,
}

extern {
    fn create(x: f32, y: f32, yaw: f32) -> VehiclePose;
    fn translate(vehicles: *mut VehiclePose, n: u64, dx: f32, dy: f32);
    static mut next_id: u64;
    fn create_heap() -> *mut VehiclePose;
    fn free_heap(vehicle: *mut VehiclePose);
}

fn create_vehicle() -> Option<&'static VehiclePose> {
    let vehicle_ptr = unsafe { create_heap() };
    if vehicle_ptr.is_null() {
        None
    } else {
        let vehicle_ref: &VehiclePose = unsafe { &*vehicle_ptr };
        Some(vehicle_ref)
    }
}

fn main() {
    // Task 2: Call the create C function and display the vehicle state.
    let vehicle = unsafe { create(1.0, 2.0, 3.0) };
    println!("Task 2: Created vehicle: {:?}", vehicle);

    // Task 3: Create 10 vehicles, put them in a vector, and call the translate function.
    let mut vehicles: Vec<VehiclePose> = Vec::new();
    for _ in 0..10 {
        let vehicle = unsafe { create(0.0, 0.0, 0.0) };
        vehicles.push(vehicle);
    }
    unsafe {
        let mut vehicles_ptr = vehicles.as_mut_ptr();
        translate(vehicles_ptr, 10, 1.0, 1.0);
    }
    println!("Task 3: Translated vehicles: {:?}", vehicles);

    // Task 4: Access and print the next_id variable defined in the C code.
    unsafe {
        println!("Task 4: next_id in C: {}", next_id);
    }

    // Task 5: Call create_heap function and convert the resulting nullable pointer into an Option containing a reference.
    let vehicle_option = create_vehicle();
    match vehicle_option {
        Some(vehicle) => {
            println!("Task 5: Created heap vehicle: {:?}", vehicle);
            unsafe {
                free_heap(vehicle as *const _ as *mut VehiclePose);
            }
        },
        None => {
            println!("Task 5: Failed to create heap vehicle.");
        }
    }

    // Bonus Challenge 1: Load the C shared library dynamically and use it.
    let lib = Library::new("libkpns.so").expect("Failed to load shared library");
    unsafe {
        let create: Symbol<unsafe extern "C" fn(f32, f32, f32) -> VehiclePose> = lib.get(b"create").expect("Symbol not found");
        let translate: Symbol<unsafe extern "C" fn(*mut VehiclePose, u64, f32, f32)> = lib.get(b"translate").expect("Symbol not found");

        let vehicle = create(4.0, 5.0, 6.0);
        println!("Bonus Challenge 1: Created vehicle using shared library: {:?}", vehicle);

        let mut shared_lib_vehicles: Vec<VehiclePose> = vec![vehicle; 10];
        let mut shared_vehicles_ptr = shared_lib_vehicles.as_mut_ptr();
        translate(shared_vehicles_ptr, 10, 2.0, 2.0);
        println!("Bonus Challenge 1: Translated vehicles using shared library: {:?}", shared_lib_vehicles);
    }

    // Bonus Challenge 2: Spawn two threads and pass the reference to the heap-allocated VehiclePose.
    let vehicle_option = create_vehicle();
    match vehicle_option {
        Some(vehicle) => {
            let vehicle_clone = vehicle.clone();
            let handle = std::thread::spawn(move || {
                println!("Bonus Challenge 2: In thread: {:?}", vehicle_clone);
            });

            handle.join().unwrap();
        },
        None => {
            println!("Bonus Challenge 2: Failed to create heap vehicle.");
        }
    }
}
