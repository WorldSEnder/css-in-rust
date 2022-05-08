fn main() {
    let _ = stylist::generic::css! {
        .outer {
            .inner {
                background-color: red;
            }
        }
    };
}
