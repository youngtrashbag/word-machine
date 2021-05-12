import { useState, useEffect } from "react";

import Word from "./components/Word";
import "./Word.scss";

const WordListView = () => {
    // setting as false because it serves double purpose as "hasLoaded"
    const [ getWords, setWords ] = useState(false);

    useEffect(async () => {
        const response = await fetch("http://localhost:4322/all_words/en", {
            method: "GET", // *GET, POST, PUT, DELETE, etc.
            mode: "cors", // no-cors, *cors, same-origin
            cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
            credentials: "same-origin", // include, *same-origin, omit
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        });

        setWords(await response.json());
    }, []);

    if (!getWords) {
        return (<h1>Loading...</h1>);
    }

    return (
        <>
            <ul>
            {
                getWords.map( function(w) {
                    return (
                        <>
                        { Word(w) }
                        <br/>
                        </>
                    )
                })
            }
            </ul>
        </>
    );
}

export default WordListView;
