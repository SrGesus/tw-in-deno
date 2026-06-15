use std::time::Duration;

use rustyscript::worker::{DefaultWorker, DefaultWorkerOptions};

fn main() {
    let worker = DefaultWorker::new(DefaultWorkerOptions {
        default_entrypoint: None,
        timeout: Duration::from_secs(3),
        startup_snapshot: None,
        shared_array_buffer_store: None,
    })
    .unwrap();

    // Run twind as umds
    worker
        .eval::<()>(include_str!("../node_modules/twind/twind.umd.js").to_string())
        .unwrap();
    worker
        .eval::<()>(include_str!("../node_modules/twind/sheets/sheets.umd.js").to_string())
        .unwrap();

    let js_code = r#"
        const sheet = twindSheets.virtualSheet();

        twind.setup({ theme: {}, sheet });
        const element = twind.tw`bg-blue-500 text-white p-4`;
        twindSheets.getStyleTagProperties(sheet).textContent
    "#;

    let styles: String = worker.eval::<String>(js_code.to_string()).unwrap();

    println!("{}", styles);
}
