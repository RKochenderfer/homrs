import React, {useState} from 'react'
import {Link} from 'react-router-dom'
import {Button, Collapse, Navbar, NavbarToggler, NavbarBrand, Nav, NavItem, NavLink} from 'reactstrap'
import { useAuth } from "../../context/auth";

const NavMenu = (props) => {
	const [collapsed, setCollapsed] = useState(true)
	const { setAuth } = useAuth()

	const toggleNavbar = () => setCollapsed(!collapsed)


	function logout() {
		setAuth(false)
	}

	return (
		<div>
			<Navbar color="faded" dark>
				<Link to="/"><NavbarBrand className="mr-auto">Homrs</NavbarBrand></Link>
				<Button onClick={logout}>Log out</Button>
			</Navbar>
		</div>
	)
}

export default NavMenu