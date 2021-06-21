import {
    HashRouter as Router,
    Switch,
    Route,
    Link
} from "react-router-dom";
import { useEffect, useState } from "react";

import SearchView from "./SearchView";
import WordView from "./WordView";
import WordListView from "./WordListView";

import './App.scss';


function App() {
    const [language, setLanguage] = useState("en");

    useEffect(() => {
        // change the selected language to the one in session storage
        const changeLangElement = document.getElementById("changeLanguage");

        switch (language) {
            case("de"):
                changeLangElement.options[1].selected = true;
                break;
            // english language should be selected by default
            default:
                changeLangElement.options[0].selected = true;
                break;
        }
    }, [language]);

    function changeLanguage(event) {
        if (event.target.value === "de") {
            setLanguage("de");
        } else {
            setLanguage("en");
        }
    }

    return (
        <>
        <Router>
        <div className="Header">
            <ul>
                <div>
                    <Link to="/">Search</Link>
                </div>
                <div>
                    <Link to="/all">List of Words</Link>
                </div>
            </ul>
            <div className="LanguageSelection">
                <p>Language:</p>
                &nbsp;
                <select id="changeLanguage" onChange={changeLanguage}>
                    <option value="en">English</option>
                    <option value="de">Deutsch</option>
                </select>
            </div>
        </div>
        <div className="App">
            <div className="Container">
                <Switch>
                    <Route path="/all">
                        <WordListView language={language} />
                    </Route>
                    <Route path="/word/:word">
                        <WordView language={language} />
                    </Route>
                    <Route exact path="/">
                        <SearchView />
                    </Route>
                    <Route path="*">
                        <h1>Not Found</h1>
                    </Route>
                </Switch>
            </div>
        </div>
        </Router>
        </>
    );
}

export default App;
