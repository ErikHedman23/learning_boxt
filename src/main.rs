use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!(
        "Vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );
    // The Box::new() takes ownership of the vehicle variable, so the vehicle variable goes out of
    // scope.
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);

    println!(
        "boxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    //using the &* gives us a reference to the memory that is located on the heap.  The * is used as a
    //dereferencer, and shows the memory that is located on the heap.
    println!(
        "boxed_vehicle size on heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );

    //If we want to move the Box data from the heap to the stack, we can do so be dereferencing it:
    let unboxed_vehicle: Shuttle = *boxed_vehicle;

    println!(
        "unboxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );

    //reasons for using the Box<> data type:
    //-Store a type whose size cannot be known at compile time; however, you need to use the value
    //in a context that requires a known size.
    //An example could be a Recursive type.
    //-Transfer ownership of data that would normally live on the stack, rather than copying that
    //data.  This allows you to avoid copying large amounts of stack data.
}
