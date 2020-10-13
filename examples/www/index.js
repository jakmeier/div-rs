/**
 * The code in here is not really all that much about DIV-RS.
 * It just some JS code to load the different examples defined in their own Rust crate.
 * You should better look at the lib.rs files in those subfolders to learn more about div.
 * Also, look at this npm package's configurations to learn about the integration.
 */

import "./styles.css";

import * as hello_world from "../hello_world/pkg/hello_world_bg.wasm";
import * as reposition from "../reposition/pkg/reposition_bg.wasm";
import * as styled from "../styled/pkg/styled_bg.wasm";
import * as toggle from "../toggle/pkg/toggle_bg.wasm";


import * as hello_svelte from "../hello_svelte/pkg/hello_svelte_bg.wasm";
import { register_svelte_component, init_div_rs } from "../../div-rs.js";
import MyComponent from "../hello_svelte/src/MyComponent.svelte";

const examples = [];

function example(name, help, fn) {
    return { name, help, fn };
}

// Add examples one-by-one with name and call to main
examples.push(example(
    "Hello World",
    "let x = 100;\n" +
    "let y = 100;\n" +
    "let w = 500;\n" +
    "let h = 500;\n" +
    "let html = \"Hello world\";\n" +
    "div::new_pane(x, y, w, h, html);",
    () => {
        hello_world.main();
    }));
examples.push(example(
    "Reposition",
    "Use arrow keys to move all panes and +,- to change scaling factor.\n\n" +
    "div::reposition(x, y)\n" +
    "div::resize(w, h)\n\n" +
    " Use W,A,S,D to move only one pane and 1,2 for scaling.\n\n" +
    "pane_a.reposition(x,y)\n" +
    "pane_a.resize(w,h)\n" +
    "\n\nThis is meant to be used as hovering test over a canvas that needs to be rescaled when the window size changes.",
    () => {
        reposition.main();
    }));
examples.push(example(
    "Styled",
    "These panes are dynamically styled from within Rust, using CSS.",
    () => {
        styled.main();
    }));
examples.push(example(
    "Toggle",
    "hi.show()\nbye.hide()\n\n" +
    "Two panes are periodically displayed and hidden, as controlled by Rust code.",
    () => {
        toggle.main();
    }));

// Svelte example need some more initialization
examples.push(example(
    "Hello Svelte",
    "This component is defined within a *.svelte file and loaded dynamically through Rust.\n\n" +
    "let class = JsClass::preregistered(\"Component\")?;\n" +
    "div::from_js_class(X, Y, W, H, class)?;",
    () => {
        init_div_rs();
        register_svelte_component("MyComponent", MyComponent);
        hello_svelte.main();
    }));

loadExampleSelection(examples);

let params = new URLSearchParams(location.search);
let displayedExample = params.get('example');

if (displayedExample) {
    const example = examples[displayedExample];
    example.fn();
    displayHint(example.help);
} else {
    displayHint("Use the drop down to switch between examples.");
}

function loadExampleSelection(examples) {
    const form = document.createElement("form");
    form.setAttribute("method", "GET");
    const button = document.createElement("input");
    button.setAttribute("type", "submit");
    button.setAttribute("value", "Show");
    form.appendChild(button);

    const select = document.createElement("select");
    select.setAttribute("name", "example")
    form.appendChild(select);
    for (let i = 0; i < examples.length; i++) {
        let option = document.createElement("option");
        option.value = i;
        option.text = examples[i].name;
        select.appendChild(option);
    }
    const main = document.getElementsByTagName("main")[0];
    main.prepend(form);
}

function displayHint(text) {
    const floatingText = document.createElement("p");
    if (text) {
        floatingText.className = "hint";
        floatingText.innerText = text;
        const body = document.getElementsByTagName("body")[0];
        body.appendChild(floatingText);
    }
}