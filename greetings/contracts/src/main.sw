contract;

use std::logging::log;
use std::auth::msg_sender;

abi Greeting {
    fn greet(id: u64, greeting: str[10]);
}

struct NewGreeting {
    id: u64,
    greeting: str[10],
}

impl Greeting for Contract {
    fn greet(id: u64, greeting: str[10]) {
        log(NewGreeting {
            id,
            greeting,
        });

        // logging greeter
        log(msg_sender().unwrap());
    }
}
