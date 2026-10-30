/// Ownership Rules
/// Is a set of rules that governs how Rust manages memory.
/// Some langugage keeps looking for gc, some needs you to do manually, Rust uses third way.
/// Memory is managed through system of ownership that a set of rules that compiler checks and if any rules violated the program wont compile.
/// Ownership does not slow down your program while its running.


/**
 * Stack and Heap
 * - Both are used to store data in the memory.
 * - Stack is LIFO and has a known size in the memory, means the size has to be known at the compile time
 * - Heap is unorganised and it searches for the space in the memory and can grow as needed. It allocates pointers into the memory whos size is known too.
 * - Stack eg: Plates when stacked can only be picked mostly from the top. So LIFO, and is fast as memory knows where the data is.
 * - Heap eg: Allocator takes more effort to search to allocate big space from memory and it has to follow pointers and maintain a bookkeeping.
 * - Stacks memory are close to each other hence accessing data is faster, while heap because it can be far away it can be slower.
*/

/**
 * Ownership Rules
 * - Every value has a owner in Rust and only one at a time.
 * - If owner goes out of the scope the value will be dropped.
 */
pub fn ownership_rules() {
    // This is how it works in most of the language
    { // At this point s is unknown
        let mut s = "Hello"; // s is declared and can be used from here
        s = "1212";
        println!("{s}")
        // use s
    } // s is out of scope and hence it will be removed

    /*
     *  
     */
    let mut s = String::from("Hello");
    s.push_str("Tom");
    println!("{s}")

}