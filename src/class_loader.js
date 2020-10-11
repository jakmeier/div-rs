export function instantiate_class(className, target) {
    new window.panes[className]({ target, props: {} });
}
export function loading_progress() {
    window.panes = window.panes || {};
    return window.panes.____loaded || 0;
}