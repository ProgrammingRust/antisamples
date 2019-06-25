
use std::sync::mpsc;
use std::thread::spawn;

trait OffThreadExt: Iterator {
    /// Transform this iterator into an off-thread iterator: the
    /// `next()` calls happen on a separate worker thread, so the
    /// iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T: Iterator> OffThreadExt for T
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        // Create a channel to transfer items from the worker thread.
        let (sender, receiver) = mpsc::sync_channel(1024);

        // Move this iterator to a new worker thread and run it there.
        spawn(move || {
            //~^ ERROR: `T` cannot be sent between threads safely
            //~| NOTE: `T` cannot be sent between threads safely
            //~| HELP: the trait `std::marker::Send` is not implemented for `T`
            //~| HELP: consider adding a `where T: std::marker::Send` bound
            //~| NOTE: required because it appears within the type
            //~| NOTE: required by `std::thread::spawn`
            //~| ERROR: `<T as std::iter::Iterator>::Item` cannot be sent between threads safely
            //~| NOTE: `<T as std::iter::Iterator>::Item` cannot be sent between threads safely
            //~| HELP: the trait `std::marker::Send` is not implemented for `<T as std::iter::Iterator>::Item`
            //~| HELP: consider adding a `where <T as std::iter::Iterator>::Item: std::marker::Send` bound
            //~| NOTE: required because of the requirements on the impl of `std::marker::Send` for `std::sync::mpsc::SyncSender<<T as std::iter::Iterator>::Item>`
            //~| NOTE: required because it appears within the type
            //~| NOTE: required by `std::thread::spawn`
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        // Return an iterator that pulls values from the channel.
        receiver.into_iter()
    }
}
