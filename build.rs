fn main() {
    // 只在 Windows 靶上才链图标
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        // 告诉 cargo 重新编译的条件
        println!("cargo:rerun-if-changed=app.ico");
        // 让 rustc 把图标资源编进 PE
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=kernel32");
        // 实际嵌入
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
