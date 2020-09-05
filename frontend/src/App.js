import React, {useState} from 'react';
import {BrowserRouter, Route, Switch} from 'react-router-dom'
import {Layout} from "./components/Layout/Layout";
import './App.css';
import Login from "./components/Login";
import {AuthContext} from "./context/auth";
import PrivateRoute from "./PrivateRoute";
import Admin from "./components/Admin";
import Home from "./components/Home";
import Meals from "./components/Meals";
import Intercom from "./components/Intercom";
import CreateAccount from "./components/CreateAccount";

function App() {
	const existingSession = document.cookie.match(/^(.*;)?\s*session-token\s*=\s*[^;]+(.*)?$/) === undefined
	const [authValid, setAuthValid] = useState(existingSession)

	const setAuthConfirmed = valid => {
		setAuthValid(valid)
	}

	return (
		<div>
			<AuthContext.Provider value={{ authValid, setAuthValid: setAuthConfirmed }}>
			{/*<AuthContext.Provider value={{auth, setAuth: setAuthConfirmed}}>*/}
				<BrowserRouter>
					<Switch>
						<Route exact path="/" render={(props) => <Login {...props} />}/>
						<Route exact path="/signup" component={CreateAccount} />
						<Route exact path="/home" render={props => <Layout><Home {...props}/></Layout>} />
						<PrivateRoute path="/meals" render={props => <Layout><Meals {...props}/></Layout>} />
						<PrivateRoute path="/intercom" render={props => <Layout><Intercom {...props}/></Layout>} />
						<PrivateRoute path="/admin" render={props => <Layout><Admin {...props}/></Layout>} />
					</Switch>
				</BrowserRouter>
			</AuthContext.Provider>
		</div>
	);
}

export default App;
