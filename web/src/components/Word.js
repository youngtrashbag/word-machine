import { Link } from "react-router-dom";

import "../Word.scss";

const Word = (word) => {
    const link = `/word/${word.word}`
    return (
        <>
        <div className="WordContainer">
            <h1><Link to={ link }>{ word.word }</Link></h1>
            <p>{ word.definition }</p>
            <div className="SynonymContainer">
                <ul>
                {
                    word.synonyms.map( s => <li>{ s }</li>)
                }
                </ul>
            </div>
        </div>
        </>
    );
}

export default Word;
