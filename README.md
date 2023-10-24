# Kopernikus_Rust_C-CodeChallenge
Coding challenge completed.
Kopernikus Rust FFI Code Challenge Documentation

Table of Contents
1.	Introduction
2.	Getting Started
•	Prerequisites
•	Setup
3.	Project Structure
4.	Tasks and Challenges
•	Task 1: Define VehiclePose in Rust
•	Task 2: Call the create C function
•	Task 3: Create 10 Vehicles and Translate
•	Task 4: Access and Print next_id
•	Task 5: Call the create_heap function
•	Bonus Challenge 1: Dynamic Loading of Shared Library
•	Bonus Challenge 2: Thread Interaction
5.	Usage
•	Running the Program

######################################################################################################################################################################################################################################################
# 1. Introduction
This documentation provides an overview of the Kopernikus Rust FFI code challenge, which involves interacting with C code from a Rust project. The challenge consists of several tasks and bonus challenges to test your Rust skills.
# 2. Getting Started
## 2.1 Prerequisites
Before starting, make sure you have the following prerequisites:
•	Rust programming environment (Rust 1.61 or later)
•	C compiler (GCC)
•	Cargo (Rust package manager)
## 2.2 Setup
1.	Clone the repository containing the challenge code.
$ git clone https://github.com/jaypee92/Kopernikus_Rust_C-CodeChallenge.git 
2.	Build the C shared library.
$ gcc -shared -fPIC kpns.c -o libkpns.so 
3.	Update your Cargo.toml to include the required dependencies, as mentioned in the challenge description.
4.	Build and run the Rust project using the following command.
$ cargo run 
# 3. Project Structure
•	Cargo.toml: Rust project configuration file.
•	main.rs: Main Rust code implementing the challenge tasks.
•	build.rs: Rust build script for compiling C code.
# 4. Tasks and Challenges
## 4.1 Task 1: Define VehiclePose in Rust
This task defines the VehiclePose struct in Rust, compatible with the C code's definition.
## 4.2 Task 2: Call the create C function
Here calls the C function to create and display the vehicle state.
## 4.3 Task 3: Create 10 Vehicles and Translate
Creates 10 vehicles, puts them in a vector, and calls the translate function to modify their positions.
## 4.4 Task 4: Access and Print next_id
Accesses and then prints the next_id variable defined in the C code.
## 4.5 Task 5: Call the create_heap function
Calls the create_heap function and convert the result into an Option containing a reference.
## 4.6 Bonus Challenge 1: Dynamic Loading of Shared Library
Compiled the C code as a shared library and load it dynamically in Rust. This task will perform operations using the shared library.
## 4.7 Bonus Challenge 2: Thread Interaction
Spawns two thread and passes a reference to the heap-allocated VehiclePose between them.
# 5. Usage
## 5.1 Running the Program
Execute the Rust program using the following command:
$ cargo run 
This will run the code for the tasks and bonus challenges, producing the expected results as described in the program's output.


