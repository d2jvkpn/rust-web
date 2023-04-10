import React, { Component } from 'react';
import { Navigate } from "react-router-dom";
import { authed, getPublicUrl } from 'js/base.js';
import { getUser, setRefreshToken } from "js/auth.js";

class HomePage extends Component {
  constructor(props) {
    super(props);
    this.state = {};
  }

  render() {
    if(!authed()) {
      return <Navigate to={getPublicUrl() + "/login"} />;
    }

    setRefreshToken();
    let user = getUser();

    return (<div className="home-container">
      <h2>Home</h2>
      <div> Welcome, {user.name}... </div>
    </div>);
  }
}

export default HomePage;
