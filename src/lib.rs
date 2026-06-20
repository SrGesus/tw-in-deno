use std::sync::Mutex;

use rustyscript::{
    Module,
    deno_core::ModuleId,
    serde_json::json,
    worker::{DefaultWorker, DefaultWorkerOptions},
};

const TWIND_MODULE: Module = Module::new_static("twind.js", include_str!("twind.umd.js"));

const TWIND_SHEETS_MODULE: Module = Module::new_static("sheets.js", include_str!("sheets.umd.js"));

const BUILDER_MODULE: Module = Module::new_static(
    "builder",
    r#"
        const sheet = twindSheets.virtualSheet();
        twind.setup({
            mode: 'silent',
            sheet
        });

        export const add_classes = (classes) => {
            twind.tw(classes);
        }

        export const bundle = () => {
            return twindSheets.getStyleTagProperties(sheet).textContent;
        }
    "#,
);

pub struct TailwindBuilder {
    builder_handle: ModuleId,
    worker: Mutex<DefaultWorker>,
}

impl Default for TailwindBuilder {
    fn default() -> Self {
        let worker = DefaultWorker::new(DefaultWorkerOptions {
            ..Default::default()
        })
        .unwrap();

        worker.load_module(TWIND_MODULE).unwrap();
        worker.load_module(TWIND_SHEETS_MODULE).unwrap();
        let builder_handle = worker.load_module(BUILDER_MODULE).unwrap();

        Self {
            builder_handle,
            worker: Mutex::new(worker),
        }
    }
}

impl std::fmt::Debug for TailwindBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TailwindBuilder")
            .field("builder_handle", &self.builder_handle)
            .field("worker", &"rustyscript::worker::DefaultWorker")
            .finish()
    }
}

impl TailwindBuilder {
    pub fn add_classes(&mut self, classes: &str) {
        self.worker
            .lock()
            .unwrap()
            .call_function(
                Some(self.builder_handle),
                "add_classes".to_owned(),
                vec![json!(classes)],
            )
            .unwrap()
    }

    pub fn bundle(&mut self) -> String {
        self.worker
            .lock()
            .unwrap()
            .call_function(Some(self.builder_handle), "bundle".to_owned(), vec![])
            .unwrap()
    }
}
