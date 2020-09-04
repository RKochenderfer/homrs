import React from "react"
import { Route, Redirect } from "react-router-dom"
import { useAuth } from "./context/auth"

function PrivateRoute({ component: Component, ...rest }) {
	const {authValid} = useAuth();

	return (
		<Route
			{...rest}
			render={props =>
				authValid ? (
					<Component {...props} />
				) : (
					<Redirect to="/" />
				)
			}
		/>
	)
}

export default PrivateRoute