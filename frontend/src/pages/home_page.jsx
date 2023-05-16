import React, { Component } from 'react';
import { message, Dropdown, Space, Modal } from "antd";
import { DownOutlined } from '@ant-design/icons';
import { Navigate } from "react-router-dom";

import "./home_page.css";
import { authed, getPublicUrl } from 'js/base.js';
import { getUser, setRefreshToken, logout } from "js/auth.js";
import { datetime } from "js/utils.js";
import { chatQuery, sendMsg, chatItems2Msgs } from "js/chat.js";

class HomePage extends Component {
  constructor(props) {
    super(props);
    this.state = {
      messages: [],
      msg: "",
      logoutVisible: false,
      changePasswordVisible: false,
    };
  }

  componentDidMount() {
  }

  handleKeyPress = (event) => {
    if (this.state.messages.length === 0) {
      chatQuery((res) => {
        let data = res.data;

        if (!data.items || data.items.length === 0 ) {
          return;
        }

        let messages = chatItems2Msgs(data.items);
        // console.log(`~~~ ${JSON.stringify(messages)}`);
        this.setState({messages: messages});
      });
    }

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
    let msg = {
      sender: "user",
      role: "user",
      content: content,
      timestampMilli: ts.getTime(),
      at: ts.time,
    };

    let messages = [msg, ...this.state.messages];
    this.setState({messages: messages, msg: ""});

    sendMsg(msg, (res) => {
      let data = res.data;
      let ts = datetime(new Date(data.created*1000));

      if (!data.choices || data.choices.length === 0) {
        message.warn("no response!");
        return;
      }

      let choice = data.choices[0];
      console.log(`~~~ response: ${choice.message.content}`);

      let got = {
        sender: "system",
        role: choice.role,
        content: choice.message.content,
        timestampMilli: ts.getTime(),
        at: ts.time,
      };

      let messages = [got, ...this.state.messages];
      this.setState({messages: messages, msg: ""});
    });
  }

  menuClick = (e) => {
    if (e.key === "logout") {
      // logout();
      this.setState({logoutVisible: true});
    } else if (e.key === "change-password") {
      this.setState({changePasswordVisible: true});
    } else {
      console.log(`!!! TODO menuClick: ${e.key}`);
    }
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

    let userTitle = `${user.email || user.phone} (id: ${user.id})`;

    const menuItems = [
      {label: "Settings", key: "settings"},
      {label: "Change password", key: "change-password"},
      {label: "Logout", key: "logout"},
    ];

    // <Dropdown onVisibleChange={this.setVisible} visible={this.state.visible}> </Dropdown>
    // <a onClick={(e) => e.preventDefault()}> </a>
    return (<>
    <div className="chat-container">
      <div className="chat-header">
        <div> </div>

        <div>Chatting with AI...</div>

        <div>
          <Dropdown menu={{items: menuItems, onClick: this.menuClick}}>
            <Space title={userTitle}>
              {user.name} <DownOutlined />
            </Space>
          </Dropdown>
        </div>
      </div>

      <div className="chat-window">
        {this.state.messages.map((msg, index) => <Message key={index} msg={msg} />)}
      </div>

      <div className="chat-input">
        <textarea placeholder="Type message here..." value={this.state.msg}
          onChange={(event) => this.setState({msg: event.target.value})}
          onKeyPress={this.handleKeyPress}
        />
        <button onClick={() => this.handleSend()}>Send</button>
      </div>
    </div>

    <Modal title="Logout" visible={this.state.logoutVisible}
      onOk={() => logout() }
      onCancel={() => this.setState({logoutVisible: false})}
      okText="Confirm"
      cancelText="Cancel"
    >
      <p style={{paddingLeft: "2rem", fontSize: "1rem"}} textIndent="2rem">
        Are you sure you want to logout?
      </p>
    </Modal>

    <Modal title="Change password" visible={this.state.changePasswordVisible}
      onOk={() => {
        console.log(`!!! TODO: Change password`);
        this.setState({changePasswordVisible: false});
      }}
      onCancel={() => this.setState({changePasswordVisible: false})}
      okText="Confirm"
      cancelText="Cancel"
    >
      !!! TODO: Change password
    </Modal>
    </>);
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
