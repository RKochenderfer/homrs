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
	const [authValid, setAuthValid] = useState(false)

	const setAuthConfirmed = valid => {
		if (valid !== undefined) {
			setAuthValid(valid)
		} else {
			fetch('/api/sessions/status', {
				method: 'GET'
			})
				.then(res => {
					setAuthValid(res.ok)
				})
				.catch(e => {
					setAuthValid(false)
					console.error(e)
				})
		}
	}

	return (
		<div>
			<AuthContext.Provider value={{ authValid, setAuthValid: setAuthConfirmed }}>
			{/*<AuthContext.Provider value={{auth, setAuth: setAuthConfirmed}}>*/}
				<BrowserRouter>
					<Switch>
						<Route exact path="/" render={(props) => <Login {...props} />}/>
						<Route exact path="/signup" component={CreateAccount} />
						<PrivateRoute exact path="/home" component={Home} />
						<PrivateRoute path="/meals" component={Meals} />
						<PrivateRoute path="/intercom" component={Intercom} />
						<PrivateRoute path="/admin" component={Admin} />
					</Switch>
				</BrowserRouter>
			</AuthContext.Provider>
		</div>
	);
}

export default App;
