import React, {useState} from 'react'
import {
	Alert,
	Button,
	Card,
	CardBody,
	CardFooter,
	CardHeader,
	Col,
	Container,
	Form,
	FormGroup,
	Input,
	Label,
	Row
} from 'reactstrap'
import {Link, Redirect} from "react-router-dom";
import {useAuth} from "../context/auth";

function CreateAccount() {
	const [isLoggedIn, setLoggedIn] = useState(false)
	const [firstName, setFirstName] = useState('')
	const [lastName, setLastName] = useState('')
	const [email, setEmail] = useState('')
	const [password, setPassword] = useState('')
	const [isError, setIsError] = useState(false)
	const [errorMsg, setErrorMsg] = useState('')
	const {setAuthValid} = useAuth()

	async function makeCreateRequest() {
		// TODO: ONCE ROLES ARE ESTABLISHED UPDATE USR
		let body = JSON.stringify(
			{
				'email': email,
				'first_name': firstName,
				'last_name': lastName,
				'password': password,
				'user_role': 'USR',
			}
		)
		return await fetch('/api/users', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body
		})
	}

	async function makeLoginRequest() {
		let body = JSON.stringify({"email": email, "password": password})

		return await fetch('/api/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body
		})
	}

	function login() {
		makeLoginRequest()
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

	function createAccount() {
		makeCreateRequest()
			.then(res => {
				console.log(res)
				if (res.status === 200) {
					res.json().then(_body => login)
				}
			})
			.catch(e => {
				console.error(e)
				setAuthValid(false)
				setLoggedIn(false)
				setIsError(true)
				setErrorMsg('Something went wrong creating your account')
			})
	}

	if (isLoggedIn) {
		return <Redirect to="/home"/>
	}

	return (
		<Container>
			<Row>
				<Col>
					<Card className="card-form">
						<CardHeader>Create an Account</CardHeader>
						<CardBody>
							<Form>
								<FormGroup>
									<Label for="first-name">First Name</Label>
									<Input type="text" value={firstName} name="first-name" id="first-name"
										   placeholder="First Name" onChange={e => {
										setFirstName(e.target.value)
									}}/>
								</FormGroup>
								<FormGroup>
									<Label for="last-name">Last Name</Label>
									<Input type="text" vale={lastName} name="last-name" id="last-name"
										   placeholder="Last Name" onChange={e => {
										setLastName(e.target.value)
									}}/>
								</FormGroup>
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
								<Button onClick={createAccount}>Create Account</Button>
							</Form>
							<Alert color="warning" isOpen={isError}>
								{errorMsg}
							</Alert>
						</CardBody>
						<CardFooter>
							<Link to="/">Already have an account?</Link>
						</CardFooter>
					</Card>
				</Col>
			</Row>
		</Container>
	)
}

export default CreateAccount