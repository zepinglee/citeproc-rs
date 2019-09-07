// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright © 2018 Corporation for Digital Scholarship

mod utils;

#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate log;

use self::utils::ErrorPlaceholder;

use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::{future_to_promise, JsFuture};

use citeproc::Processor;
use citeproc::prelude::*;
use csl::locale::Lang;

#[wasm_bindgen]
pub struct Driver {
    engine: Rc<RefCell<Processor>>,
    fetcher: PromiseFetcher,
}

#[wasm_bindgen]
impl Driver {
    pub fn new(style: &str, promise_fetcher: PromiseFetcher) -> Result<Driver, JsValue> {
        utils::set_panic_hook();
        utils::init_log();

        // The Processor gets a "only has en-US, otherwise empty" fetcher.
        let us_fetcher = Arc::new(utils::USFetcher);
        let engine = Processor::new(style, us_fetcher, true)
            .map(RefCell::new)
            .map(Rc::new)
            .map_err(|e| JsValue::from_serde(&e).unwrap())?;

        // The Driver manually adds locales fetched via PromiseFetcher, which asks the consumer
        // asynchronously.
        Ok(Driver {
            engine,
            fetcher: promise_fetcher,
        })
    }

    #[wasm_bindgen(js_name = "setStyle")]
    pub fn set_style(&mut self, style_text: &str) -> JsValue {
        self.engine
            .borrow_mut()
            .set_style_text(style_text)
            .map_err(|e| JsValue::from_serde(&e).unwrap())
            .err()
            .unwrap_or(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "setReferences")]
    pub fn set_references(&mut self, refs: Box<[JsValue]>) -> Result<(), JsValue> {
        let refs = utils::read_js_array(refs)?;
        Ok(self.engine.borrow_mut().set_references(refs))
    }

    #[wasm_bindgen(js_name = "insertReference")]
    pub fn insert_reference(&mut self, js_refr: JsValue) -> Result<(), JsValue> {
        let refr = js_refr
            .into_serde()
            .map_err(|_| ErrorPlaceholder::throw("could not parse Reference from host"))?;
        // inserting & replacing are the same
        self.engine.borrow_mut().insert_reference(refr);
        Ok(())
    }

    #[wasm_bindgen(js_name = "toFetch")]
    pub fn locales_to_fetch(&self) -> JsValue {
        let langs: Vec<String> = self
            .engine
            .borrow()
            .get_langs_in_use()
            .iter()
            .map(|l| l.to_string())
            .collect();
        JsValue::from_serde(&langs).unwrap()
    }

    #[wasm_bindgen(js_name = "insertCluster")]
    pub fn insert_cluster(
        &mut self,
        cluster: JsValue,
    ) -> Result<(), JsValue> {
        let cluster = cluster
            .into_serde()
            .map_err(|e| ErrorPlaceholder::throw(&format!("could not parse cluster from host: {}", e)))?;
        let mut eng = self.engine.borrow_mut();
        Ok(eng.insert_cluster(cluster))
    }

    #[wasm_bindgen(js_name = "removeCluster")]
    pub fn remove_cluster(
        &mut self,
        cluster: u32,
    ) -> Result<(), JsValue> {
        let mut eng = self.engine.borrow_mut();
        Ok(eng.remove_cluster(cluster))
    }

    #[wasm_bindgen(js_name = "initClusters")]
    pub fn init_clusters(&mut self, clusters: Box<[JsValue]>) -> Result<(), JsValue> {
        let clusters = utils::read_js_array(clusters)?;
        Ok(self.engine.borrow_mut().init_clusters(clusters))
    }

    #[wasm_bindgen(js_name = "builtCluster")]
    pub fn built_cluster(&self, id: ClusterId) -> Result<JsValue, JsValue> {
        let built = (*self.engine.borrow().get_cluster(id)).clone();
        Ok(JsValue::from_serde(&built).unwrap())
    }

    #[wasm_bindgen(js_name = "renumberClusters")]
    pub fn renumber_clusters(&mut self, mappings: Box<[JsValue]>) -> Result<(), JsValue> {
        let mappings: Vec<(ClusterId, ClusterNumber)> = utils::read_js_array(mappings)?;
        let mut eng = self.engine.borrow_mut();
        eng.renumber_clusters(&mappings);
        Ok(())
    }

    #[wasm_bindgen(js_name = "batchedUpdates")]
    pub fn batched_updates(&self) -> JsValue {
        let eng = self.engine.borrow();
        let summary = eng.batched_updates();
        JsValue::from_serde(&summary).unwrap()
    }

    /// This asynchronously fetches all the locales that may be required, and saves them into the
    /// engine. It holds a mutable borrow for the duration of the fetches, and any other Driver
    /// method will fail until all fetches return.
    ///
    /// This needs improvement -- a fully queued architecture that is aware of any 'locks' would be
    /// better. You could enqueue `SetReferences` (which might trigger fetching) and be notified
    /// when your document needed updating.
    #[wasm_bindgen(js_name = "fetchAll")]
    pub fn fetch_all(&self) -> Promise {
        let rc = self.engine.clone();
        let fetcher = self.fetcher.clone();
        let future = async move {
            // Keep these two RefCell borrows short-lived. The { scope } ensures the first borrow
            // ends before the JS code runs indefinitely. Just in case someone calls the Driver
            // while a request is in-flight. The ultimate effect is that the RefCell is only
            // borrowed for the time that the Rust code is in control.
            let langs: Vec<Lang> = {
                let eng = rc.borrow();
                eng.get_langs_in_use()
                    .iter()
                    // we definitely have en-US, it's statically included
                    .filter(|l| **l != Lang::en_us() && !eng.has_cached_locale(l))
                    .cloned()
                    .collect()
            };
            let pairs = fetch_all(&fetcher, langs).await;
            let mut eng = rc.borrow_mut();
            eng.store_locales(pairs);
            Ok(JsValue::null())
        };
        future_to_promise(future)
    }

    #[wasm_bindgen(js_name = "disambiguationDfaDot")]
    pub fn disambiguation_dfa_dot(&self, key: &str) -> String {
        let id = Atom::from(key);
        let eng = self.engine.borrow();
        if let Some(graph) = eng.ref_dfa(id) {
            return graph.debug_graph(&*eng);
        }
        "".to_string()
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_name = "Lifecycle")]
    pub type PromiseFetcher;

    #[wasm_bindgen(method, js_name = "fetchLocale")]
    fn fetch_locale(this: &PromiseFetcher, lang: &str) -> Promise;

    #[wasm_bindgen(js_name = "error", js_namespace = console)]
    fn log_js_error(val: JsValue);
}

// TODO: include note about free()-ing the Driver before an async fetchLocale() call comes back (in
// which case the Driver reference held to by the promise handler function is now a dangling
// wasm-bindgen pointer).
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

/** This interface lets citeproc retrieve locales or modules asynchronously,
    according to which ones are needed. */
export interface Lifecycle {
    /** Return locale XML for a particular locale. */
    fetchLocale(lang: string): Promise<string>;
}

export type DateLiteral = { "literal": string; };
export type DateRaw = { "raw": string; };
export type DatePartsDate = [number] | [number, number] | [number, number, number];
export type DatePartsSingle = { "date-parts": [DatePartsDate]; };
export type DatePartsRange = { "date-parts": [DatePartsDate, DatePartsDate]; };
export type DateParts = DatePartsSingle | DatePartsRange;

/** Locator type, and a locator string, e.g. `["page", "56"]`. */
export type DateOrRange = DateLiteral | DateRaw | DateParts;

/** Locator type, and a locator string, e.g. `["page", "56"]`. */
export type Locator = [string, string];

export type Cite<Affix = string> = {
    id: string;
    prefix?: Affix;
    suffix?: Affix;
    suppression?: "InText" | "Rest" | null;
    locators?: Locator[];
    locatorExtra?: string;
    locatorDate?: DateOrRange | null;
};

export type NoteCluster = {
    id: number;
    cites: Cite[];
    note: number | [number, number]
};

export type InTextCluster = {
    id: number;
    cites: Cite[];
    in_text: number;
};

export type Cluster = NoteCluster | InTextCluster;

export type Reference = {
    id: string;
    type: CslType;
    [key: string]: any;
};

export type CslType = "book" | "article" | "legal_case" | "article-journal";

export type UpdateSummary = {
    clusters: [number, any[]][];
};

type InvalidCsl = {
    severity: "Error" | "Warning";
    range: {
        start: number;
        end: number;
    };
    message: string;
    hint: string;
};
type ParseError = {
    ParseError: string;
};
type Invalid = {
    Invalid: InvalidCsl[];
};
type StyleError = Partial<ParseError & Invalid>;


"#;

/// Asks the JS side to fetch all of the locales that could be called by the style+refs.
async fn fetch_all(inner: &PromiseFetcher, langs: Vec<Lang>) -> Vec<(Lang, String)> {
    // Promises are push-, not pull-based, so this kicks all of the requests off at once. If the JS
    // consumer is making HTTP requests for extra locales, they will run in parallel.
    let thunks: Vec<_> = langs
        .into_iter()
        .map(|lang| {
            let promised = inner.fetch_locale(&lang.to_string());
            let future = JsFuture::from(promised);
            (lang.clone(), future)
        })
        // Must collect to avoid shared + later mutable borrows of self.cache in two different
        // stages of the iterator
        .collect();
    let mut pairs = Vec::with_capacity(thunks.len());
    for (lang, thunk) in thunks {
        // And collect them.
        match thunk.await {
            Ok(got) => match got.as_string() {
                Some(string) => pairs.push((lang, string)),
                // JS consumer did not return a string. Assume it was null/undefined/etc, so no
                // locale was available.
                None => {}
            },
            // ~= Promise.catch; some async JS code threw an Error.
            Err(e) => {
                error!("caught: failed to fetch lang {}", lang);
                log_js_error(e);
            }
        }
    }
    pairs
}
