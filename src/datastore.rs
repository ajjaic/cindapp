use names::Generator;
use std::path::PathBuf;
use core::default::Default;


struct DataStore {
    client: PathBuf,
    d_addr: PathBuf,
    restaurant: PathBuf,
    restaurant_item: PathBuf,
    driver_item: PathBuf,
}

impl Default for DataStore {
    fn default() -> Self {
        DataStore {
            client: PathBuf::from("./client.json"),
            d_addr: PathBuf::from("./d_addr.json"),
            restaurant: PathBuf::from("./restaurant.json"),
            restaurant_item: PathBuf::from("./restaurant_item.json"),
            driver_item: PathBuf::from("./driver_item.json"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    id: usize,
    name: String,
    phone: usize,
    delivery_addr: usize,
}

struct DeliveryAddr {
    id: usize,
    addr: String,
}

struct Restaurant {
    id: usize,
    name: String,
}

struct RestaurantItem {
    id: usize,
    name: String,
}

struct DriverItem {
    id: usize,
    name: String,
}

#[cfg(test)]
mod tests {
    extern crate names;
    extern crate random;

    use super::Client;
    use self::names::Generator;
    use self::random::default;
    use self::random::Source;


    fn random_name() -> String {
        let mut g = Generator::default().next();

        if g.is_none() { "None".to_string() } else { g.unwrap() }
    }

    fn random_number() -> usize {
        let mut r_src = default();

        r_src.read::<usize>()
    }

    fn random_client() -> Client {
        Client {
            id: random_number(),
            name: random_name(),
            phone: random_number(),
            delivery_addr: random_number(),
        }
    }

    #[test]
    fn test_write_client() {
        println!("{:?}", random_client());
    }
}
