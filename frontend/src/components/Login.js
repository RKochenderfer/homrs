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
import {useAuth} from "../context/auth";

function Login() {
	const [isLoggedIn, setLoggedIn] = useState(false)
	const [isError, setIsError] = useState(false)
	const [errorMsg, setErrorMsg] = useState('')
	const [email, setEmail] = useState('')
	const [password, setPassword] = useState('')
	const {setAuthValid} = useAuth()

	async function makeRequest() {
		let body = JSON.stringify({"email": email, "password": password})

		return await fetch('/api/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body
		})
	}

	if (isLoggedIn) {
		return <Redirect to="/home"/>
	}

	function login() {
		makeRequest()
			.then(res => {
				if (res.status === 200) {
					res.json().then(body => {
						if (body.logged_in === true) {
							setAuthValid(true)
							setLoggedIn(true)
						} else {
							setIsError(true)
							setAuthValid(false)
							setErrorMsg('Email or password is incorrect.')
						}
					})
				}
			})
			.catch(e => {
				console.error(e)
				setIsError(true)
				setAuthValid(false)
				setErrorMsg('There was an error logging you in')
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
						<Button onClick={login}>Login</Button>
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