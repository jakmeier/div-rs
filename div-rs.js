/**
 * DIV-RS
 * Contains JS functions intended for usage within JS.
 * From the Rust side, there should no need to call JS code directly
 * when working with div.
 */

export function init_div_rs() {
    window.__div_rs = window.__div_rs || { svcom: {}, loaded: 0 };
}
export function register_svelte_component(name, component) {
    window.__div_rs.svcom[name] = component;
}