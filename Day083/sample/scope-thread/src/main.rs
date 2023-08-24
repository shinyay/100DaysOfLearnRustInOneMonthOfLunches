use std::{ thread, time };

fn main() {
   // create a scope
   thread::scope(|scope| {

       // spawn first thread
       scope.spawn(|| {
           thread::sleep( time::Duration::from_secs(1) );
           // wait for 1 second before printing "Hello, from thread 1"
           println!("Hello, from thread 1");
       });

       // spawn second thread
       scope.spawn(|| {
           thread::sleep( time::Duration::from_secs(2) );
           // wait for 2 seconds before printing "Hello, from thread 2"
           println!("Hello, from thread 2");
       });

       // spawn third thread
       scope.spawn(|| {
           thread::sleep( time::Duration::from_secs(3) );
           // wait for 3 seconds before printing "Hello, from thread 3"
           println!("Hello, from thread 3");
       });
   });

   // all threads within the scope has to be closed
   // for the program to continue
   println!("All threads completed!");
}