import React, {useState} from "react"
import { Route, Redirect } from "react-router-dom"
import { useAuth } from "./context/auth"
import {Layout} from "./components/Layout/Layout";

function PrivateRoute({ component: Component, ...rest }) {

	const {authValid} = useAuth()

	return (
		<Route
			{...rest}
			render={props =>
				authValid ? (
					<Layout>
						<Component {...props} />
					</Layout>
				) : (
					<Redirect to="/" />
				)
			}
		/>
	)
}

export default PrivateRoute