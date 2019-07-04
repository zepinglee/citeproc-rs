import React, { Component, ChangeEvent, useEffect, useRef } from 'react';
import { asyncComponent } from 'react-async-component';
import { Reference, Cluster, Driver, StyleError, ParseError, Invalid } from '../../pkg';
import { useState } from 'react';
import { DocumentEditor } from './DocumentEditor';
import { Result, Err, Ok, Option, Some, None } from 'safe-types';
import { useDocument } from './useDocument';
import { string } from 'prop-types';

let initialStyle = `<style class="note">
  <features>
    <feature name="conditions" />
    <feature name="condition-date-parts" />
  </features>
  <locale>
    <terms>
      <term name="ibid">ibid</term>
    </terms>
  </locale>
  <citation et-al-min="3">
    <layout delimiter="; " suffix=".">
      <choose>
        <if position="ibid-with-locator">
          <group delimiter=" ">
            <text term="ibid" />
            <text variable="locator" />
          </group>
        </if>
        <else-if position="ibid">
          <text term="ibid" />
        </else-if>
        <else-if position="subsequent">
          <group delimiter=" ">
            <text variable="title" font-style="italic" />
            <text prefix="(n " variable="first-reference-note-number" suffix=")" />
            <text variable="locator" />
          </group>
        </else-if>
        <else>
          <group delimiter=", ">
            <text variable="title" font-style="italic" />
            <names variable="author" />
            <choose>
              <if>
                <conditions match="all">
                  <condition has-day="issued" />
                </conditions>
                <date variable="issued" form="numeric" />
              </if>
            </choose>
            <text variable="locator" />
          </group>
        </else>
      </choose>
    </layout>
  </citation>
</style>`;

const initialReferences: Reference[] = [
    {
        id: 'citekey',
        type: 'book',
        author: [{ given: "Kurt", family: "Camembert" }],
        title: "Where The Vile Things Are",
        issued: { "raw": "1999-08-09" },
        language: 'en-GB',
    },
    {
        id: 'foreign',
        type: 'book',
        title: "Some other title",
        language: 'fr-FR',
    }
];

const initialClusters: Cluster[] = [
    {
        id: 1,
        cites: [
            { citeId: 1, id: "citekey" }
        ],
        noteNumber: 1,
    },
    {
        id: 2,
        cites: [
            { citeId: 2, id: "citekey" }
        ],
        noteNumber: 2,
    },
    {
        id: 3,
        cites: [
            { citeId: 3, id: "citekey", locators: [["page", "56"]] }
        ],
        noteNumber: 3,
    },
];

const mono = {
    width: '100%',
    minHeight: '300px',
    fontFamily: 'monospace',
};

async function loadEditor() {
    // Load wasm before making it interactive.
    // Removes failed expectation of immediate response compared to lazily loading it.
    await import('../../pkg');

    const StyleEditor = ({style, setStyle, setReferences} : {
        inFlight: boolean,
        style: string,
        setStyle: React.Dispatch<string>,
        setReferences: (rs: Reference[]) => void;
    }) => {
        const [refsText, setRefsText] = useState(JSON.stringify(initialReferences, null, 2));

        const parseRefs = () => {
            try {
                let refs = JSON.parse(refsText);
                setReferences(refs);
            } catch (e) {
                console.error("could not parse references json", e);
            }
        };

        useEffect(parseRefs, [ refsText ]);

        const firstRun = useRef(true);
        if (firstRun.current) {
            firstRun.current = false;
            parseRefs();
        }

        let column = { width: '50%' };
        return <div>
            <div style={{display: 'flex'}}>
                <div style={column}>
                    <h3>Style</h3>
                    <textarea value={style} onChange={e => setStyle(e.target.value)} style={mono} />
                </div>
                <div style={column}>
                    <h3>References</h3>
                    <textarea value={refsText} onChange={e => setRefsText(e.target.value)} style={mono} />
                </div>
            </div>
        </div>;
    }

    return StyleEditor;
}

const AsyncEditor = asyncComponent({
    resolve: loadEditor,
    LoadingComponent: () => <div><i>(Loading editor)</i></div>, // Optional
    ErrorComponent: ({ error }) => <pre>{JSON.stringify(error)}</pre> // Optional
});

const Results = ({ driver, style }: { driver: Result<Driver, any>, style: string }) => {
    return driver.match({
        Ok: d => <p>
            locales in use:
            <code>{JSON.stringify(d.toFetch().sort())}</code>
        </p>,
        Err: e => <ErrorViewer style={style} error={e as StyleError} />
    });
};

const ErrorViewer = ({style, error}: { style: string, error: StyleError }) => {
    if (error.ParseError) {
        let e = error as ParseError;
        return <p>{ e.ParseError }</p>
    } else if (error.Invalid) {
        let e = error as Invalid;
        return <div>{ e.Invalid.map(i => {
            let text = style.slice(i.range.start, i.range.end);
            return <div key={i.range.start * style.length + i.range.end}
                        style={{backgroundColor: '#ff00002b', marginBottom: '5px'}}>
                <p>{ `${i.severity}: ${i.message}` }</p>
                <pre style={{marginLeft: "2em" }}>{ text }</pre>
                { i.hint && <p>{ i.hint } </p>}
            </div>
        }) } </div>
    } else {
        return null;
    }
}

const App = () => {
    const {
        document,
        driver,
        style,
        setStyle,
        inFlight,
        setDocument,
        resetReferences,
        updateReferences,
    } = useDocument(initialStyle, initialReferences, initialClusters);

    const docEditor = document.map(doc =>
        <DocumentEditor
            document={doc}
            onChange={newDoc => setDocument(Some(newDoc))} />
    ).unwrap_or(null);

    return (
        <div className="App">
            <header className="App-header">
                <a
                    className="App-link"
                    href="https://github.com/cormacrelf/citeproc-rs"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    Test driver for <code>citeproc-wasm</code>
                </a>
            </header>
            <AsyncEditor style={style} setStyle={setStyle} inFlight={inFlight} setReferences={resetReferences} />
            <Results style={style} driver={driver} />
            { docEditor }
        </div>
    );
};


export default App;
