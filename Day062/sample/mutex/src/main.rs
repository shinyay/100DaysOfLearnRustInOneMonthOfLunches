use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(10);
    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{:?}", my_mutex);
    println!("{:?}", mutex_changer);

    *mutex_changer *= 10;


    println!("my_mutex: {:?}", my_mutex);
    println!("{:?}", mutex_changer);

    let your_mutex = Mutex::new(10);
    {
        let mut mutex_changer = your_mutex.lock().unwrap();
        *mutex_changer += 1;

        println!("your_mutex: {:?}", your_mutex);
        println!("{:?}", mutex_changer);
    }

    println!("your_mutex: {:?}", your_mutex);
}
