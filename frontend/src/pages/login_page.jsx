import "./login_page.css";

import React, { Component } from 'react';

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

    this.setState({
      [name]: value
    });
  }

  handleSubmit = (event) => {
    event.preventDefault();
    console.log(`~~~ login: ${JSON.stringify(this.state)}`);

    // Call backend API to validate user's email/phone and password
    // ...

    // Redirect to user's desired page or display success/error message
    // ...
  }

  render() {
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
