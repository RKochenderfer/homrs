import React from 'react';
import {BrowserRouter, Route, Switch} from 'react-router-dom'
import {Layout} from "./components/Layout";
import './App.css';
import Login from "./components/Login";

function App() {
	return (
		<div className="App">
			<BrowserRouter>
				<Switch>
					<Route exact path="/" render={(props) => <Login {...props} />} />
				</Switch>
			</BrowserRouter>
		</div>
	);
}

export default App;
