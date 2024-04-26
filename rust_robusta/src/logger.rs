use android_logger::Config;

pub fn init_android_logger(tag: &str, max_levels: Option<&[log::LevelFilter]>) {
    let mut config = Config::default().with_tag(tag);
    match max_levels {
        Some(levels) => {
            for &max_level in levels {
                config = config.with_max_level(max_level);
            }
        }
        None => {
            config = config.with_max_level(log::LevelFilter::Debug);
        }
    }
    android_logger::init_once(config);
}
