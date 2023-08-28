use neon::{prelude::*, types::buffer::TypedArray};

fn js_xxh3_hash64(mut cx: FunctionContext) -> JsResult<JsString> {
  let input = cx.argument::<JsBuffer>(0)?;
  let output = twox_hash::xxh3::hash64(input.as_slice(&cx));
  Ok(cx.string(output.to_string()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("xxh3_hash64", js_xxh3_hash64)?;
  Ok(())
}
