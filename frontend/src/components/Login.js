import React, {useState} from 'react'
import {Link, Redirect} from 'react-router-dom'
import {
	Alert,
	Button,
	Card,
	CardBody,
	CardFooter,
	CardHeader,
	Container,
	Form,
	FormGroup,
	Input,
	Label
} from 'reactstrap'

import '../Login.css'
import {login} from "./Shared/Login";
import {useAuth} from "../context/auth";

function Login() {
	const [isLoggedIn, setLoggedIn] = useState(false)
	const [isError, setIsError] = useState(false)
	const [errorMsg, setErrorMsg] = useState('')
	const [email, setEmail] = useState('')
	const [password, setPassword] = useState('')
	const {setAuthValid} = useAuth()

	if (isLoggedIn) {
		return <Redirect to="/home"/>
	}

	function handleLogin() {
		login(email, password)
			.then(x => {
				setAuthValid(true)
				setLoggedIn(x)
			})
			.catch(e => {
				console.error(e)
			})
	}

	return (
		<Container className="h-100">
			<Card className="card-form">
				<CardHeader>Login</CardHeader>
				<CardBody>
					<Form>
						<FormGroup>
							<Label for="email">Email</Label>
							<Input type="email" value={email} name="email" id="email" placeholder="Enter Email"
								   onChange={e => {
									   setEmail(e.target.value)
								   }}/>
						</FormGroup>
						<FormGroup>
							<Label for="password">Password</Label>
							<Input type="password" value={password} name="password" id="password"
								   placeholder="Enter password" onChange={e => {
								setPassword(e.target.value)
							}}/>
						</FormGroup>
						<Alert color="warning" isOpen={isError}>
							{errorMsg}
						</Alert>
						<Button onClick={handleLogin}>Login</Button>
					</Form>
				</CardBody>
				<CardFooter>
					<Link to="/signup">Don't have an account?</Link>
				</CardFooter>
			</Card>
		</Container>
	)
}

export default Login