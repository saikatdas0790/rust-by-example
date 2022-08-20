mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

fn main() {
    other_function();

    println!("Entering block");
    {
        use crate::deeply::nested::function;
        function();

        println!("Leaving block");
    }

    function();
}
