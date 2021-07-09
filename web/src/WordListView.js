import { useState, useEffect } from "react";

import Word from "./components/Word";
import "./Word.scss";

const WordListView = (props) => {
    const { language } = props;
    // setting as false because it serves double purpose as "hasLoaded"
    const [ response, setResponse ] = useState(false);

    useEffect(async () => {
        const response = await fetch(`http://localhost:${process.env.REACT_APP_PORT_DATA}/all_words/${language}`, {
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
    }, [language]);

    if (!response) {
        return (<h1>Loading...</h1>);
    } else {
        if (response.ok) {
            return (
                <>
                    <ul>
                    {
                    response.json.map( function(w) {
                        return (
                            <>
                            <Word word={ w } />
                            <br/>
                            </>
                        )
                    })
                    }
                    </ul>
                </>
            );
        } else {
            return (
                <>
                <div className="Error">
                    <p>Could not fetch words</p>
                </div>
                </>
            );
        }
    }
}

export default WordListView;
