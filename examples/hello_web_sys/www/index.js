import * as wasm from "../pkg/hello_web_sys_bg.wasm";
import MyComponent from "../src/MyComponent.svelte";
import { register_svelte_component, init_div_rs } from "../../../div-rs.js";

init_div_rs();
register_svelte_component("MyComponent", MyComponent);
wasm.main();