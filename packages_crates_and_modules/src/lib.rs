
//-----------------------------------------------------------------
// By default, all items in a module are private.
// 
// The code inside a module can access any other item inside that
// same module (and its children). However, code outside a module
// cannot see its private items.
//-----------------------------------------------------------------


// bring the 'hosting' module paths into scope with 'use'
use crate::front_of_house::hosting;


//-----------------------------------------------------------------
//             Separating modules into different files
//-----------------------------------------------------------------

// Declare the 'online_house' module. Since the compiler cannot see
// a definition for this module it will look for the definition in
// a file named 'online_house.rs'.
pub mod online_house;

// Now that the compiler knows where the 'online_house' module is
// defined, lets import the 'website' module.
use crate::online_house::website;


//-----------------------------------------------------------------
//             Local (same file) module definitions
//-----------------------------------------------------------------


mod front_of_house {

    // Here we need to declare the 'hosting' module as public so
    // that code outside of 'front_of_house' can access it.
    pub mod hosting {

        //--------------------------
        // public functions
        //--------------------------

        pub fn add_to_waitlist() {}

        pub fn accept_payment() {}

        //--------------------------
        // private functions
        //--------------------------

        fn seat_at_table() {}
    }

    // Here we need to declare the 'serving' module as public so
    // that code outside of 'front_of_house' can access it.
    pub mod serving {

        //--------------------------
        // public functions
        //--------------------------

        pub fn take_order() {}

        //--------------------------
        // private functions
        //--------------------------

        fn serve_order() {}
        
        fn take_payment() {}
    }

    // Public so that it can be accessed by code outside of the
    // 'front_of_house' module
    pub fn complain() {

        //---------------------------------------------------------
        //
        //               Relative path with 'Super'
        //
        //---------------------------------------------------------

        super::back_of_house::fix_incorrect_order();
    }

}


mod back_of_house {

    // Lets declare this 'Breakfast' struct as public so that code from
    // outside of the 'back_of_house' module can access it.
    pub struct Breakfast {

        // By declaring the 'toast' field public, it can be accessed/modified
        // directly by user code. See the 'eat_at_restaurant' function below.
        pub toast: String,

        // By default the 'seasonal_fruit' field is private, so it cannot be
        // accessed directly by user code outside of the 'back_of_house' module.
        seasonal_fruit: String,
    }

    // Lets define some behavior for the 'Breakfast' struct
    impl Breakfast {

        // We need to declare this constructor function as public so that it
        // can be accessed by the user code in the 'eat_at_restaurant' function.
        pub fn summer(toast: &str) -> Breakfast {

            // Return a new 'Breakfast' object
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {}
}


//-----------------------------------------------------------------
//             Local (same file) function definitions
//-----------------------------------------------------------------


pub fn eat_at_restaurant() {

    //-------------------------------------------------------------
    //
    //                       Relative path
    //
    //-------------------------------------------------------------

    // Order a breakfast in the summer with Rye toast.
    // Note the use of relative paths when accessing the 'back_of_house' module.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like.
    // Note that we can only change the 'toast' field of the 'Breakfast'
    // struct because 'toast' is declared as a public field.
    meal.toast = String::from("Wheat");

    // This is an invalid operation because the 'seasonal_fruit" field
    // is private, it was not declared public.
    //
    // meal.seasonal_fruit = String::from("blueberries");

    //-------------------------------------------------------------
    //
    //                      Absolute path
    //
    //-------------------------------------------------------------

    crate::front_of_house::complain();

    //-------------------------------------------------------------
    //
    //            Bringing Paths into Scope with 'use'
    //
    //-------------------------------------------------------------

    // after eating is finished, lets pay the bill
    hosting::accept_payment();

    //-------------------------------------------------------------
    //
    //            Using an external module definition
    //
    //-------------------------------------------------------------

    // submit feedback on the restaurant's website
    website::accept_electronic_feedback(String::from("Great atmosphere!"));

}
