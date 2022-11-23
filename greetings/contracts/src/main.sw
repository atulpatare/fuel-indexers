contract;

use std::hash::sha256;
use std::logging::log;
use std::auth::msg_sender;

abi Greeting {
    fn greet(greeting: str[10]);
}

struct NewGreeting {
    greeting: b256,
}

impl Greeting for Contract {
    fn greet(greeting: str[10]) {

        // logging the hash of the greeting
        log(NewGreeting {
            greeting: sha256(greeting),
        });

        // logging greeter
        log(msg_sender().unwrap());
    }
}
