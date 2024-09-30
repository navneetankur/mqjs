fn main() {
    futures_lite::future::block_on(
        mqjs::realmain(std::env::args())
    );
}
