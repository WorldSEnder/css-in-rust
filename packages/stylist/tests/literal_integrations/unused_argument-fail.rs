fn main() {
    let _ = stylist::generic::css! {r#"
        background: ${used};
    "#, unused = 1000, used = "black"
    };
}
