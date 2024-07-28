fn main() {
    let mngr = Oswald::new(Box::new(SQLiteStore::new()));
}
