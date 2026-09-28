use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {

    //-----------------------------------------------------------------
    //
    //               Unrecoverable errors with panic!
    //
    //-----------------------------------------------------------------

    // Uncomment these two lines and run with 'cargo run' to see
    // how the Rust runtime will panic and terminate the executable
    // when an erroneous runtime operation is executed. In this case
    // the vector only has 3 elements, but we try to access an element
    // at position 99 which does not exist. For a more verbose panic
    // description run with 'RUST_BACKTRACE=1 cargo run'
    //
    // let vec = vec![1, 2, 3];
    // vec[99];

    // Uncomment this line and run with 'cargo run' to see how we
    // can use the panic! macro to programmatically have the Rust
    // runtime terminate the executable when we detect a condition
    // that should result in termination of the executable.
    //
    // panic!("Something very bad is happening!");

    //-----------------------------------------------------------------
    //
    //               Recoverable errors with Result
    //
    //-----------------------------------------------------------------

    // The Result enum is returned by many functions and allows the
    // function to indicate success or failure. On success, the Ok(T)
    // variant is returned. On failure, the Err(E) variant is returned.
    // Then, we can use pattern matching to determine if the Result
    // was success or failure. By doing this we are forced to account
    // for both the success and failure scenarios due to the requirement
    // that pattern matching must be exhuastive.
    //
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // The 'file' variable is of type Result here. This is because the
    // File::open function returns a Result. After this API is used to
    // initialize the 'file' variable we must then check it to see if
    // it is the Ok(T) variant or the Err(E) varient.
    let file = File::open("hello.txt");

    // Use pattern matching to find out which Result variant 'file' is
    let _f = match file {
        // 'file' is the Ok(T) variant, so the File::open was successful
        Ok(file) => {
            println!("File hello.txt was opened!");
            file  // return the file handle
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File hello.txt was created!");
                    fc  // return the file handle
                },
                // terminate the executable
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                // terminate the executable
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    //-----------------------------------------------------------------
    //
    //               Shortcuts for panic! on Err(E)
    //
    //-----------------------------------------------------------------

    // The 'unwrap' method of Result will call panic! if the File::open
    // function returns an Err(E). However, the panic! message will be
    // generic.
    //
    // Uncomment the following line to see the 'unwrap' function in action
    //
    // let _file_unwrap = File::open("panic.txt").unwrap();

    // The 'expect' method of Result will also call panic! if the FIle:open
    // function returns an Err(E). With this we can specify a specific
    // panic! message.
    //
    // Uncomment the following line to see the 'expect' function in action
    //
    // let _file_expect = File::open("panic.txt").expect("File is missing, time to panic!");

    //-----------------------------------------------------------------
    //
    //               Propagating Errors with ?
    //
    //-----------------------------------------------------------------

    //
    // This example demonstrates the basic way to check the Result for
    // Ok(t) or Err(e), without using the '?' operator, as we've already
    // done in the previous examples . See the 'read_username_from_file()' 
    // function below for details.
    //

    let username = read_username_from_file();

    match username {
        Ok(str) => {
            println!("username is {}", str);
        },
        Err(e) => {
            println!("error is {}", e);
        }
    }

    //
    // This example is similar to the previous. The difference here is
    // that the function 'read_username_from_file_q()' uses the '?' operator
    // as a shortcut for the 'match' pattern.
    //

    let username_q = read_username_from_file_q();

    match username_q {
        Ok(str) => {
            println!("username is {}", str);
        },
        Err(e) => {
            println!("error is {}", e);
        }
    }

    //
    // This example is similar to the previous. The difference here is
    // that the function 'read_username_from_file_q_chained()' demonstrates
    // how to chain several calls to the '?' operator for making the code
    // more clear and concise.
    //

    let username_qc = read_username_from_file_q_chained();

    match username_qc {
        Ok(str) => {
            println!("username is {}", str);
        },
        Err(e) => {
            println!("error is {}", e);
        }
    }

}

// Attempts to open a file and read a username string from it. If
// succesful the username is returned in an Ok(t), otherwise an Err(e)
// is returned.
fn read_username_from_file() -> Result<String, io::Error> {

    // attempt to open the username.txt file
    let f = File::open("username.txt");

    // if success, 'f' is set to the file handle, otherwise an Err(e)
    // is propogated to the function caller
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),  // note the use of 'return' here
    };

    // create a mutable string to store the username from the file
    let mut s = String::new();

    // attempt to read the username from the file, if successful
    // return Ok(t) with the username string in it, otherwsie
    // return Err(e) with an error in it
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Attempts to open a file and read a username string from it. If
// succesful the username is returned in an Ok(t), otherwise an Err(e)
// is returned.
fn read_username_from_file_q() -> Result<String, io::Error> {

    // attempt to open the username.txt file...
    // if success, 'f' is set to the file handle, otherwise an Err(e)
    // is propogated to the function caller
    let mut f = File::open("username_q.txt")?;  //  note the use of '?' here

    // create a mutable string to store the username from the file
    let mut s = String::new();

    // attempt to read the username from the file, if successful
    // return Ok(t) with the username string in it, otherwsie
    // return Err(e) with an error in it
    f.read_to_string(&mut s)?;  //  note the use of '?' here

    Ok(s)
}

// Attempts to open a file and read a username string from it. If
// succesful the username is returned in an Ok(t), otherwise an Err(e)
// is returned.
fn read_username_from_file_q_chained() -> Result<String, io::Error> {

    // create a mutable string to store the username from the file
    let mut s = String::new();

    // attempt to read the username.txt file and store the username
    // string in an Ok(t). If failure for any reason then just return
    // an Err(e) with an error in it. If successful, the username
    // string is stored in the 's' variable
    File::open("username_q_chained.txt")?.read_to_string(&mut s)?;

    // finally, just return the username string in an Ok(t)
    Ok(s)
}