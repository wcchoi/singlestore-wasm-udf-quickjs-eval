wit_bindgen_rust::export!("qjs.wit");
use quickjs_wasm_rs::Context;

struct Qjs;

impl qjs::Qjs for Qjs {
    fn qjs_eval(jscode: String, arg1: String) -> String {
        let context = Context::default();

        let _ = context.eval_global("udf", &jscode).unwrap();
        let global = context.global_object().unwrap();
        let entrypoint = global.get_property("udf").unwrap();
    
        let output_value = match entrypoint.call(&global, &[context.value_from_str(&arg1).unwrap()]) {
            Ok(result) => result,
            Err(err) => panic!("{}", err.to_string()),
        };

        output_value.as_str().unwrap().to_string()
    }
}
