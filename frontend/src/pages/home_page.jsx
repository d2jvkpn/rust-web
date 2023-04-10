import React, { Component } from 'react';
import { Navigate } from "react-router-dom";
import { authed, getPublicUrl } from 'js/base.js';

class HomePage extends Component {
  constructor(props) {
    super(props);
    this.state = {};
  }

  render() {
    if(!authed()) {
      return <Navigate to={getPublicUrl() + "/login"} />;
    }

    return (<div className="home-container">
      <h2>Home</h2>
      <div> Welcome, {}... </div>
    </div>);
  }
}

export default HomePage;
