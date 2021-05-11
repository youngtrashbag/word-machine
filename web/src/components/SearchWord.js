import { Formik, Form, Field } from "formik";
import { useHistory } from "react-router-dom";

import "../App.scss";

const SearchWord = () => {
    // only a single word
    const regexPattern = /^([a-z])\w{2,20}[^ ]*/;
    const history = useHistory();

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

        return error;
    }

    const onSubmit = ( data ) => {
        const word = data.word.match(regexPattern)[0];

        //redirect
        history.push(`/word/${word}`);
    };

    return (
        <>
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
        </>
    );
}

export default SearchWord;
