import "./home_page.css";
import React, { Component } from 'react';
import { Navigate } from "react-router-dom";
import { authed, getPublicUrl } from 'js/base.js';
import { getUser, setRefreshToken } from "js/auth.js";

class HomePage extends Component {
  constructor(props) {
    super(props);
    this.state = {messages: [], msg: ""};
  }

  setMessage = (event) => {
    let msg = event.target.value.trim();
    if (!msg) {
      return;
    }
    // console.log(`~~~ ${msg}`);
    this.setState({msg: msg});
  }

  handleSend = () => {
    let content = this.state.msg.trim();
    if (!content) {
      return;
    }
    let msg = {senderName: "user", content: content, timestamp: new Date().getTime()};
    let messages = [...this.state.messages, msg];
    this.setState({messages: messages, msg: ""});
  }

  render() {
    if(!authed()) {
      return <Navigate to={getPublicUrl() + "/login"} />;
    }

    setRefreshToken();
    let user = getUser();

    /*
    return (<div className="home-container">
      <h2>Home</h2>
      <div> Welcome, {user.name}... </div>
    </div>);
    */

    return (<div className="chat-container">
      <div className="chat-header"> Welcome, {user.name}... </div>
      <div className="chat-window">
        {this.state.messages.map((msg, index) => <Message key={index} msg={msg} />)}
      </div>

      <div className="chat-input">
        <input type="text" placeholder="Type message here..." value={this.state.msg}
          onChange={this.setMessage.bind(this)}
        />
        <button onClick={() => this.handleSend()}>Send</button>
      </div>
    </div>);
  }
}

class Message extends Component {
  constructor (props) {
    super(props);
    this.state = {};
  }

  render() {
    const {content, senderName, timestamp} = this.props.msg;

    let cn = senderName === "user" ? "message-from-me" : "";

    return (
      <div className={"message-container " + cn} key={this.props.index}>
        <div className="message-sender" style={{display:"none"}}>{senderName}</div>
        <div className="message-content">{content}</div>
        <div className="message-timestamp">{timestamp}</div>
      </div>
    );
  }
};

export default HomePage;
