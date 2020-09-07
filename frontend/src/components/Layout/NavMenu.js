import React, {useState} from 'react'
import {Redirect} from 'react-router-dom'
import {Collapse, Nav, Navbar, NavbarBrand, NavbarToggler, NavItem, NavLink} from 'reactstrap'
import {useAuth} from "../../context/auth";

const NavMenu = (props) => {
	const [isOpen, setOpen] = useState(false)
	const {setAuthValid} = useAuth()

	const toggleNavbar = () => setOpen(!isOpen)

	async function makeRequest() {
		return await fetch('/api/logout', {
			method: 'DELETE',
		})
	}

	function logout(e) {
		e.preventDefault()
		fetch('/api/logout', {
			method: 'DELETE'
		})
			.then(res => {
				setAuthValid(false)
			})
			.catch(e => {
				console.error(e)
			})
		// makeRequest()
		// 	.then(res => {
		// 		if (res.status === 200) {
		// 		} else {
		// 			// The logout attempt failed
		// 			console.error(res)
		// 		}
		// 	})
	}

	return (
		<div>
			<Navbar color="dark" dark expand="md">
				<NavbarBrand href="/home">Homrs</NavbarBrand>
				<NavbarToggler onClick={toggleNavbar}/>
				<Collapse isOpen={isOpen} navbar/>
				<Nav className="mr-auto" navbar>
					<NavItem>
						<NavLink href="/meals">Meals</NavLink>
					</NavItem>
					<NavItem>
						<NavLink href="/intercom">Intercom</NavLink>
					</NavItem>
					<NavItem>
						<NavLink href="/" onClick={e => logout(e)}>Log out</NavLink>
					</NavItem>
				</Nav>
			</Navbar>
		</div>
	)
}

export default NavMenu