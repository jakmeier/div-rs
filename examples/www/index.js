/**
 * The code in here is not really all that much about DIV-RS.
 * It just some JS code to load the different examples defined in their own Rust crate.
 * You should better look at the lib.rs files in those subfolders to learn more about div.
 * Also, look at this npm package's configurations to learn about the integration.
 */

import "./styles.css";

import * as hello_world from "../hello_world/pkg/hello_world_bg.wasm";
import * as reposition from "../reposition/pkg/reposition_bg.wasm";


import * as hello_svelte from "../hello_svelte/pkg/hello_svelte_bg.wasm";
import { register_svelte_component, init_div_rs } from "../../div-rs.js";
import MyComponent from "../hello_svelte/src/MyComponent.svelte";

const examples = [];

// Add examples one-by-one with name and call to main
examples.push(example(
    "Hello World",
    () => {
        hello_world.main();
    }));
examples.push(example(
    "Reposition",
    () => {
        reposition.main();
    }));

// Svelte example need some more initialization
examples.push(example(
    "Hello Svelte",
    () => {
        init_div_rs();
        register_svelte_component("MyComponent", MyComponent);
        hello_svelte.main();
    }));

loadExampleSelection(examples);

let params = new URLSearchParams(location.search);
let displayedExample = params.get('example');

if (displayedExample) {
    examples[displayedExample].fn();
}

function example(name, fn) {
    return { name, fn };
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