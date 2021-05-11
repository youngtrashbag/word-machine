import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link
} from "react-router-dom";

import SearchWord from "./components/SearchWord";
import DisplayWord from "./components/DisplayWord";

import './App.scss';

function App() {

    return (
        <>
        <Router>
        <div className="Header">
            <ul>
                <div>
                    <Link to="/">Home</Link>
                </div>
                <div>
                    <Link to="/all">List of Words</Link>
                </div>
            </ul>
        </div>
        <div className="App">
            <div className="Container">
                <Switch>
                    <Route path="/word/:word">
                        <DisplayWord />
                    </Route>
                    <Route exact path="/">
                        <SearchWord />
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
