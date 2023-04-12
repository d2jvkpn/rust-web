import "./home_page.css";
import React, { Component } from 'react';
import { Navigate } from "react-router-dom";
import { authed, getPublicUrl } from 'js/base.js';
import { getUser, setRefreshToken } from "js/auth.js";
import { datetime } from "js/utils.js";

class HomePage extends Component {
  constructor(props) {
    super(props);
    this.state = {messages: [], msg: ""};
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
        {this.state.messages.reverse().map((msg, index) => <Message key={index} msg={msg} />)}
      </div>

      <div className="chat-input">
        <input type="text" placeholder="Type message here..." value={this.state.msg}
          onChange={(event) => this.setState({msg: event.target.value})}
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
    let ts = datetime(new Date(timestamp));

    return (
      <div className={"message-container " + cn} key={this.props.index}>
        <div className="message-sender" style={{display:"none"}}>{senderName}</div>
        <div className="message-content">{content}</div>
        <div className="message-timestamp" title={ts.rfc3339ms}>{ts.date + " " + ts.time}</div>
      </div>
    );
  }
};

export default HomePage;
