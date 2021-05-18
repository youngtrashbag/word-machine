import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link
} from "react-router-dom";

import SearchView from "./SearchView";
import WordView from "./WordView";
import WordListView from "./WordListView";

import './App.scss';

function changeLanguage(event) {
    if (event.target.value == "de") {
        window.sessionStorage.setItem("language", "de");
    } else {
        window.sessionStorage.setItem("language", "en");
    }
}

function App() {
    const lang = sessionStorage.getItem("language");
    if (lang == null || lang == undefined) {
        window.sessionStorage.setItem("language", "en");
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
                <select onChange={changeLanguage}>
                    <option value="en">English</option>
                    <option value="de">Deutsch</option>
                </select>
            </div>
        </div>
        <div className="App">
            <div className="Container">
                <Switch>
                    <Route path="/all">
                        <WordListView />
                    </Route>
                    <Route path="/word/:word">
                        <WordView />
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
