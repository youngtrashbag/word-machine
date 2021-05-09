import { Formik, Form, Field } from "formik";

import './App.css';

function App() {
    // only a single word
    const regexPattern = /^([a-z])\w{2,20}[^ ]*/;

    const initialValues = {
        word: ""
    };
    
    function validateWord(value) {
        var error;

        if (!value) {
            error = "Required";
        } else if (!regexPattern.test(value)) {
            error = "Please Enter a single word between 2 and 20 characters long";
        }

        console.log(error);
        return error;
    }

    const onSubmit = async ( data ) => {
        //const word = data.word.match(regexPattern)[0];

        var word = "soliloquy"
        var synonyms = ["discourse", "monologue"];
        var definition = "the act of talking to oneself";
        //var language = "en";
        var language = "English";

        console.log(word);
        await fetch("http://localhost:4322/word", {
            method: "POST", // *GET, POST, PUT, DELETE, etc.
            mode: "cors", // no-cors, *cors, same-origin
            cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
            credentials: "same-origin", // include, *same-origin, omit
            headers: {
                //"Content-Type": "application/json",
                "Content-Type": "application/x-www-form-urlencoded",
            },
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
            body: JSON.stringify({
                word: word,
                synonyms: synonyms,
                definition: definition,
                language: language,
            })
        });
    };

    return (
        <div className="App">
            <div className="Wrapper">
                <h1>Word Machine</h1>
                <div className="SameLine">
                    <Formik
                    initialValues={initialValues}
                    onSubmit={onSubmit}
                    >
                    { ( formik ) => {
                        return (
                        <Form>
                            <Field
                                id="word"
                                name="word"
                                value={formik.values.word}
                                validate={validateWord}
                                onChange={formik.handleChange}
                                type="Word"
                                placeholder="Enter word"
                            />
                            <button type="submit" disabled={!formik.isValid || !formik.dirty}>Info!</button>
                        </Form>
                        );
                    }}
                    </Formik>
                </div>
            </div>
        </div>
    );
}

export default App;
