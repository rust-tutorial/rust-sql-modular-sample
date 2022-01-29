use mysql::Opts;
use mysql::Pool;
use mysql::PooledConn;

pub fn connect(uri: String) -> Pool {
    let opt = Opts::from_url(uri.as_str()).expect("Url error");
    Pool::new(opt).expect("Error creating pool")
}
