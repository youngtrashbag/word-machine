import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";

import Word from "./components/Word";

import "./App.scss";

const WordView = (props) => {
    const { word } = useParams();
    const { language } = props;
    // setting as false because it serves double purpose as "hasLoaded"
    const [ response, setResponse ] = useState(false);

    // eslint-disable-next-line
    useEffect(async () => {
        const response = await fetch(`http://localhost:${process.env.REACT_APP_PORT_DATA}/word/${language}/${word}`, {
            method: "GET", // *GET, POST, PUT, DELETE, etc.
            mode: "cors", // no-cors, *cors, same-origin
            cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
            credentials: "same-origin", // include, *same-origin, omit
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        });

        setResponse({
            ok: await response.ok,
            json: await response.json(),
        });
    }, [word, language]);

    if (!response) {
        return (<h1>Loading...</h1>);
    } else {
        if (response.ok) {
            return (
                <>
                <div className="Wrapper">
                    <Word word={ response.json } />
                </div>
                </>
            );
        } else {
            return (
                <>
                <div className="Error">
                    <p>Could not fetch word {word}</p>
                </div>
                </>
            );
        }
    }
}

export default WordView;
