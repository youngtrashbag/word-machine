import { Link } from "react-router-dom";

import "../Word.scss";

const Synonyms = (props) => {
    const { synonyms } = props;

    if (synonyms.length > 1) {
        return (
            <div className="SynonymContainer">
                <ul>
                {
                    synonyms.map( s => <li><Link to={ `/word/${s}` }>{ s }</Link></li>)
                }
                </ul>
            </div>
        )
    } else {
        return null;
    }
}

const Word = (props) => {
    const { word } = props;

    return (
        <>
        <div className="WordContainer">
            <h1><Link to={ `/word/${word.word}` }>{ word.word }</Link></h1>
            <p>{ word.definition }</p>
            <Synonyms synonyms={word.synonyms} />
        </div>
        </>
    );
}

export default Word;
