wit_bindgen::generate!({
    world: "check",
});

struct CheckRunner;
impl Guest for CheckRunner {
    fn run() {
        reportmetric("my.metric.name", 24.0, &["hello:world".to_string()]);
    }
}

export!(CheckRunner);
