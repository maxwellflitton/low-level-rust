

pub trait DisplayLibC {

    fn print(&self) -> ();

}


pub fn print<T: DisplayLibC>(t: T) -> () {
    t.print();
}
