import "./home_page.css";
import React, { Component } from 'react';
import { Navigate } from "react-router-dom";
import { authed, getPublicUrl } from 'js/base.js';
import { getUser, setRefreshToken } from "js/auth.js";
import { datetime } from "js/utils.js";
import { sendMsg } from "js/chat.js";

class HomePage extends Component {
  constructor(props) {
    super(props);
    this.state = {messages: [], msg: ""};
  }

  handleKeyPress = (event) => {
    if (event.ctrlKey && event.keyCode === 0) {
      // console.log(`~~~ Ctrl + Enter pressed`);
      this.handleSend();
    }
  };

  handleSend = () => {
    let content = this.state.msg.trim();
    if (!content) {
      return;
    }

    let ts = datetime(new Date());
    // let now = datetime();
    // let at = now.date === ts.date ? ts.time : ts.date + " " + ts.time;
    let msg = {sender: "user", content: content, timestampMilli: ts.getTime(), at: ts.time};

    let messages = [...this.state.messages, msg];
    this.setState({messages: messages, msg: ""});

    sendMsg(msg, (res) => {
      let data = res.data;

      let ts = datetime(new Date(data.timestampMilli));
      let got = {
        sender: data.sender, content: data.content,
        timestampMilli: ts.getTime(), at: ts.time,
      };

      let messages = [...this.state.messages, got];
      this.setState({messages: messages, msg: ""});
    })
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
          onKeyPress={this.handleKeyPress}
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
    const {content, sender, timestampMilli, at} = this.props.msg;
    let ts = datetime(new Date(timestampMilli));

    return (
      <div className={"message-container message-from-" +sender} key={this.props.index}>
        <div className="message-sender" style={{display:"none"}}>{sender}</div>
        <div className="message-content">{content}</div>
        <div className="message-timestamp" title={ts.rfc3339ms}>{at}</div>
      </div>
    );
  }
};

export default HomePage;
