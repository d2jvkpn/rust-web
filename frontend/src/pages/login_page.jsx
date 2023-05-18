import "./login_page.css";

import React, { Component } from 'react';
import { Navigate } from "react-router-dom";

import { authed, getPublicUrl } from "js/base.js";
import { login } from "js/auth.js";

class LoginPage extends Component {
  constructor(props) {
    super(props);
    this.state = {
      emailOrPhone: '',
      password: ''
    };
  }

  handleInputChange = (event) => {
    const target = event.target;
    const value = target.value;
    const name = target.name;

    this.setState({[name]: value});
  }

  handleSubmit = (event) => {
    event.preventDefault();
    // console.log(`~~~ login: ${JSON.stringify(this.state)}`);

    if (!this.state.emailOrPhone || !this.state.password) {
      return;
    }
    // TODO: validator...

    let data = {password: this.state.password};

    if (this.state.emailOrPhone.includes("@")) {
      data.email = this.state.emailOrPhone;
    } else {
      data.phone = this.state.emailOrPhone;
    }

    login(data)

    // Call backend API to validate user's email/phone and password
    // ...

    // Redirect to user's desired page or display success/error message
    // ...
  }

  render() {
    if(authed()) {
      return <Navigate to={getPublicUrl() + "/home"} />;
    }

    return (<div className="login-container">
      <h2>Login</h2>

      <form onSubmit={this.handleSubmit}>
        <div>
          <label htmlFor="emailOrPhone">Email or Phone:</label>

          <input
            type="text"
            id="emailOrPhone"
            name="emailOrPhone"
            value={this.state.emailOrPhone}
            onChange={this.handleInputChange}
          />
        </div>

        <div>
          <label htmlFor="password">Password:</label>
          <input
            type="password"
            id="password"
            name="password"
            value={this.state.password}
            onChange={this.handleInputChange}
          />
        </div>
        <button type="submit">Login</button>
      </form>
    </div>);
  }
}

export default LoginPage;
