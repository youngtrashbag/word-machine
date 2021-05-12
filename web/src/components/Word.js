import "../Word.scss";

const Word = (word) => {
    return (
        <>
        <div className="WordContainer">
            <h1>{ word.word }</h1>
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
