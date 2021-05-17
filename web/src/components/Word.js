import { Link } from "react-router-dom";

import "../Word.scss";

const Word = (word) => {
    return (
        <>
        <div className="WordContainer">
            <h1><Link to={ `/word/${word.word}` }>{ word.word }</Link></h1>
            <p>{ word.definition }</p>
            <div className="SynonymContainer">
                <ul>
                {
                    word.synonyms.map( s => <li><a href={ `/word/${s}` }>{ s }</a></li>)
                }
                </ul>
            </div>
        </div>
        </>
    );
}

export default Word;
