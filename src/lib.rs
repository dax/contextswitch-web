mod components;
mod error_pages;
mod templates;

use perseus::{define_app, Plugins};
use perseus_size_opt::{perseus_size_opt, SizeOpts};

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::list_tasks::get_template::<G>(),
        crate::templates::about::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    static_aliases: {
        "/test.txt" => "static/test.txt"
    },
    plugins: Plugins::new().plugin(perseus_size_opt, SizeOpts { wee_alloc: true, lto: true, opt_level: "s".to_string(), codegen_units: 1, enable_fluent_bundle_patch: false, })
}
