use wasm_bindgen::prelude::*;
use js_sys::{JsString, Reflect};
use oxigraph::io::GraphFormat;
use oxigraph::store::Store;
use petgraph::dot::Dot;
use petgraph::Graph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};

// Set up the panic hook for better error handling in WASM
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Expose this to JS
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Web-RDF-Class-Viz!");
}

// Visualizer struct to handle the graph and SPARQL query
pub struct Visualizer {
    store: Store,
    labels: Vec<String>,
    g: Graph<String, String>,
    nodes: HashMap<String, petgraph::graph::NodeIndex>,
    colors: HashMap<String, String>,
}

#[wasm_bindgen]
pub struct VisualizerWrapper {
    visualizer: Visualizer,
}

#[wasm_bindgen(js_class = Visualizer)]
impl VisualizerWrapper {
    pub fn new() -> VisualizerWrapper {
        VisualizerWrapper {
            visualizer: Visualizer {
                store: Store::new().unwrap(),
                labels: Vec::new(),
                g: Graph::new(),
                nodes: HashMap::new(),
                colors: HashMap::new(),
            },
        }
    }

    // Method to handle node clicks and update SPARQL query
    #[wasm_bindgen(js_name = onNodeClick)]
    pub fn on_node_click(&mut self, node_label: String) {
        let updated_query = format!("{} <{}>", self.get_current_sparql_query(), node_label);
        self.set_current_sparql_query(updated_query);
    }

    // Method to get the current SPARQL query (you can enhance this logic)
    pub fn get_current_sparql_query(&self) -> String {
        "SELECT * WHERE { ?subject ?predicate ?object . }".to_string()
    }

    // Method to set the updated SPARQL query
    pub fn set_current_sparql_query(&self, query: String) {
        web_sys::console::log_1(&JsValue::from_str(&format!("Updated SPARQL Query: {}", query)));
    }
}
