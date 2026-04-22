macro_rules! time_it {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = { $code };
        println!("{} 用时: {:?}", $name, start.elapsed());
        result
    }};
}
