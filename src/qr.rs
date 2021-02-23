#[macro_use]
extern crate sciter;
use sciter::{ Value, types::{ BOOL, VALUE } };
use qrcode_generator::QrCodeEcc;
#[no_mangle]
pub extern "system" fn SciterLibraryInit(api: &'static sciter::ISciterAPI, exported: &mut VALUE) -> BOOL {
	sciter::set_host_api(api);
	let ext_api = vmap! { "encode" => encode };
	ext_api.pack_to(exported);
	true as BOOL
}


pub fn encode(args: &[Value]) -> Value {
	let result: String = qrcode_generator::to_svg_to_string(args[0].to_string().as_str(), QrCodeEcc::Low, 1024, None::<&str>).unwrap();
	println!("{:?}", result);
	Value::from(result)
}