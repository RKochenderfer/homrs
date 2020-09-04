import React, {useState} from 'react'
import {
	Card,
	CardHeader,
	CardFooter,
	CardBody,
	Button,
	Container,
	Form,
	FormGroup,
	Input,
	Label,
	Row,
	Col,
	Alert
} from 'reactstrap'
import {Link, Redirect} from "react-router-dom";
import {useAuth} from "../context/auth";

function CreateAccount(props) {
	const [isLoggedIn, setLoggedIn] = useState(false)
	const [firstName, setFirstName] = useState('')
	const [lastName, setLastName] = useState('')
	const [email, setEmail] = useState('')
	const [password, setPassword] = useState('')
	const [isError, setIsError] = useState(false)
	const [errorMsg, setErrorMsg] = useState('')
	const {setAuth} = useAuth()

	async function makeRequest() {
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
		const response = await fetch('/api/users', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body
		})

		return response
	}

	function createAccount() {
		makeRequest()
			.then(res => {
				console.log(res)
				if (res.status === 200) {
					res.json().then(body => {
						setAuth(true)
						setLoggedIn(true)
					})
				}
			})
			.catch(e => {
				setAuth(false)
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