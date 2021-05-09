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

    const onSubmit = ( data ) => {
        // returns array; take first element of array
        const word = data.word.match(regexPattern);

        console.log(word);
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
