use rustyscript::{Module, ModuleHandle, Runtime, RuntimeOptions, json_args};

const TWIND_MODULE: Module = Module::new_static(
    "twind.js",
    include_str!("../node_modules/twind/twind.umd.js"),
);

const TWIND_SHEETS_MODULE: Module = Module::new_static(
    "sheets.js",
    include_str!("../node_modules/twind/sheets/sheets.umd.js"),
);

const BUILDER_MODULE: Module = Module::new_static(
    "builder",
    r#"
        const sheet = twindSheets.virtualSheet();
        twind.setup({
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
    builder_handle: ModuleHandle,
    runtime: Runtime,
}

impl Default for TailwindBuilder {
    fn default() -> Self {
        let mut runtime = Runtime::new(RuntimeOptions {
            ..RuntimeOptions::default()
        })
        .unwrap();

        runtime.load_module(&TWIND_MODULE).unwrap();
        runtime.load_module(&TWIND_SHEETS_MODULE).unwrap();
        let builder_handle = runtime.load_module(&BUILDER_MODULE).unwrap();

        Self {
            builder_handle,
            runtime,
        }
    }
}

impl TailwindBuilder {
    pub fn add_classes(&mut self, classes: &str) {
        self.runtime
            .call_function(
                Some(&self.builder_handle),
                "add_classes",
                json_args!(classes),
            )
            .unwrap()
    }

    pub fn bundle(&mut self) -> String {
        self.runtime
            .call_function(Some(&self.builder_handle), "bundle", json_args!())
            .unwrap()
    }
}
