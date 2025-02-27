# Bank Application in Rust (Fictional)

>[!NOTE]  
>This is a test project I created while learning Rust.  
>It is a simple and efficient banking management system in Rust.

## Description  
This project is a bank application developed in Rust that allows user management with features such as:  
- Account creation  
- Balance and personal information inquiry  
- Balance management (deposit or withdraw money)

## Features  
1. Create new users with:  
  - Name  
  - Phone (optional)  
  - Card number (randomly generated if not provided)  
  - Age  
  - Initial balance  
2. Query user information.  
3. View available balance.  
4. Add or withdraw balance.  
5. Intuitive command-line interface.

## Requirements  
To run this project, you'll need:  
- [Rust](https://www.rust-lang.org/) installed on your system.  
- Cargo to compile and run the project.

## Installation  
```sh  
# Clone the repository  
git clone https://github.com/aleee4442/App-banco-Rust.git  
cd App-banco-Rust  

# Compile and run the program  
cargo run  
```

## Usage  
1. When running the application, a menu will appear with the following options:  
   - **Add user**  
   - **View user balance**  
   - **View user information**  
   - **Add/Withdraw balance**  
   - **Exit**  
2. Enter the desired option and follow the on-screen instructions.

>[!TIP]  
> 💡 You can try different users and manage multiple accounts simultaneously.

## Libraries used  
- `std::io` for input and output.  
- `rand` for random number generation.  
- `std::thread` and `std::time::Duration` for pauses in execution.

This project has helped me better understand how Rust works, as I had to research a lot on my own.
