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

#[derive(Serialize, Deserialize, Debug)]//{{{
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
}//}}}

#[cfg(test)]
mod tests {

    use super::{Client, DataStore};
    use names::Generator;
    use random::{default, Source};
    use serde_json::to_string;
    use json_io::save;

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
        let ds = DataStore::default();
        let rnd_client = to_string(&random_client());
        assert!(rnd_client.is_ok());
        save(ds.client.as_path(), &random_client());
        // error is in the above line. random_client returns a super::Client struct.
        // super::Client struct derives serde::ser::Serialize. But I still get the error
        // `the trait `serde::ser::Serialize` is not implemented for `datastore::Client`
        // But it is. Otherwise line 87 wouldn't work. Because `to_string` requires its
        // argument to derive Serialize
    }
}
