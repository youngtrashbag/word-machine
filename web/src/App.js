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

function App() {

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
