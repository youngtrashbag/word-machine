import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";

const DisplayWord = () => {
    const { word } = useParams();
    // setting as false because it serves double purpose as "hasLoaded"
    const [ getWord, setWord ] = useState(false);

    var resWord;

    useEffect(async () => {
        const response = await fetch(`http://localhost:4322/word/en/${word}`, {
            method: "GET", // *GET, POST, PUT, DELETE, etc.
            mode: "cors", // no-cors, *cors, same-origin
            cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
            credentials: "same-origin", // include, *same-origin, omit
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        });

        setWord(await response.json());
    }, []);

    if (!getWord) {
        return (<h1>Loading...</h1>);
    }

    return (<h1>{ getWord.definition }</h1>);

}

export default DisplayWord;
