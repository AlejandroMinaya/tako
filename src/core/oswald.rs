use crate::core::ports::*;

// https://www.imdb.com/title/tt0293734/
struct Oswald {
    data_store: Box<dyn DataStore>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::ports::test;

    #[test]
    fn test_load_all_tasks_from_data_store() {
        let oswald = Oswald {
            data_store: Box::new(test::MockDataStore::default()),
        };
    }
}
