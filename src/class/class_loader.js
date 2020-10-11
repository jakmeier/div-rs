export function instantiate_svelte_component(className, target) {
    new window.__div_rs.svcom[className]({ target, props: {} });
}
export function loading_progress() {
    return window.__div_rs.loaded || 0;
}
export function init_div_rs() {
    window.__div_rs = window.__div_rs || { svcom: {}, loaded: 0 };
}

export function svelte_component_exists(name) {
    return window.__div_rs.svcom[name] !== undefined;
}